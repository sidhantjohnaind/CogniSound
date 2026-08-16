[CmdletBinding()]
param(
    [string]$Root = $PSScriptRoot,
    [int]$DelayMilliseconds = 150,
    [int]$Limit = 0
)

$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Net.Http
$tools = Join-Path $PSScriptRoot '.lyrics-tools'
$ffprobe = Get-ChildItem (Join-Path $tools 'ffmpeg') -Recurse -File -Filter 'ffprobe.exe' |
    Select-Object -First 1 -ExpandProperty FullName
$logPath = Join-Path $Root 'metadata-lyrics-scan.csv'
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
$http = New-Object System.Net.Http.HttpClient
$http.DefaultRequestHeaders.UserAgent.ParseAdd('MusicMetadataLyrics/1.0')
$http.Timeout = [TimeSpan]::FromSeconds(25)

if (-not $ffprobe) { throw 'ffprobe.exe was not found in .lyrics-tools.' }

function Get-Metadata([string]$Path) {
    $json = & $ffprobe -v error -show_entries 'format=duration:format_tags=title,artist,album,album_artist' -of json -- $Path
    if ($LASTEXITCODE -ne 0) { throw 'ffprobe failed.' }
    $data = ($json -join "`n") | ConvertFrom-Json
    $tags = $data.format.tags
    return [pscustomobject]@{
        Title = [string]$tags.title
        Artist = if ($tags.artist) { [string]$tags.artist } else { [string]$tags.album_artist }
        Album = [string]$tags.album
        Duration = [int][math]::Round([double]$data.format.duration)
    }
}

function Get-Json([string]$Uri) {
    $bytes = $http.GetByteArrayAsync($Uri).GetAwaiter().GetResult()
    return ([Text.Encoding]::UTF8.GetString($bytes) | ConvertFrom-Json)
}

function Normalize([string]$Text) {
    if (-not $Text) { return '' }
    return (($Text.Normalize([Text.NormalizationForm]::FormKD).ToLowerInvariant()) -replace '[^\p{L}\p{Nd}]', '')
}

function Test-ArtistMatch([string]$Wanted, [string]$Found) {
    $a = Normalize $Wanted
    $b = Normalize $Found
    return (-not $a -or $a -eq $b -or $a.Contains($b) -or $b.Contains($a))
}

function Find-Record($Metadata) {
    if (-not $Metadata.Title) { return $null }
    $base = 'track_name=' + [uri]::EscapeDataString($Metadata.Title)
    if ($Metadata.Artist) { $base += '&artist_name=' + [uri]::EscapeDataString($Metadata.Artist) }

    if ($Metadata.Artist -and $Metadata.Album -and $Metadata.Duration -gt 0) {
        $exact = $base +
            '&album_name=' + [uri]::EscapeDataString($Metadata.Album) +
            '&duration=' + $Metadata.Duration
        try {
            $record = Get-Json ('https://lrclib.net/api/get?' + $exact)
            if ($record) { return $record }
        }
        catch {
            if ($_.Exception.Message -notmatch '404') { throw }
        }
    }

    $results = @(Get-Json ('https://lrclib.net/api/search?' + $base))
    $wantedTitle = Normalize $Metadata.Title
    $matches = foreach ($record in $results) {
        $durationDifference = if ($Metadata.Duration -gt 0) {
            [math]::Abs([double]$record.duration - $Metadata.Duration)
        } else { 0 }
        if ((Normalize $record.trackName) -eq $wantedTitle -and
            (Test-ArtistMatch $Metadata.Artist $record.artistName) -and
            $durationDifference -le 5) {
            [pscustomobject]@{ Record = $record; Difference = $durationDifference }
        }
    }
    return ($matches | Sort-Object Difference | Select-Object -First 1).Record
}

$files = @(Get-ChildItem -LiteralPath $Root -Recurse -File -Filter '*.flac' |
    Where-Object { $_.FullName -notlike "$tools*" } |
    Sort-Object FullName)
if ($Limit -gt 0) { $files = @($files | Select-Object -First $Limit) }

$rows = New-Object System.Collections.Generic.List[object]
$created = 0
$existing = 0
$plainOnly = 0
$notFound = 0
$instrumental = 0
$errors = 0
$index = 0

foreach ($file in $files) {
    $index++
    $lrcPath = [IO.Path]::ChangeExtension($file.FullName, '.lrc')
    if ([IO.File]::Exists($lrcPath)) {
        $existing++
        continue
    }

    $status = ''
    $detail = ''
    try {
        $metadata = Get-Metadata $file.FullName
        Write-Host "[$index/$($files.Count)] $($metadata.Artist) - $($metadata.Title)"
        $record = Find-Record $metadata
        if (-not $record) {
            $status = 'not-found'
            $notFound++
        }
        elseif ($record.instrumental) {
            $status = 'instrumental'
            $instrumental++
        }
        elseif ($record.syncedLyrics) {
            [IO.File]::WriteAllText($lrcPath, ($record.syncedLyrics.TrimEnd() + "`r`n"), $utf8NoBom)
            $status = 'created-synced'
            $detail = "$($record.artistName) - $($record.trackName)"
            $created++
        }
        elseif ($record.plainLyrics) {
            $status = 'plain-only'
            $detail = "$($record.artistName) - $($record.trackName)"
            $plainOnly++
        }
        else {
            $status = 'not-found'
            $notFound++
        }
    }
    catch {
        $status = 'error'
        $detail = $_.Exception.Message
        $errors++
    }

    $rows.Add([pscustomobject]@{
        File = $file.FullName.Substring($Root.TrimEnd('\').Length).TrimStart('\')
        Status = $status
        Detail = $detail
    })
    $rows | Export-Csv -LiteralPath $logPath -NoTypeInformation -Encoding UTF8
    if ($DelayMilliseconds -gt 0) { Start-Sleep -Milliseconds $DelayMilliseconds }
}

$http.Dispose()
Write-Output "Scanned=$($files.Count) Existing=$existing Created=$created PlainOnly=$plainOnly Instrumental=$instrumental NotFound=$notFound Errors=$errors"
Write-Output "Log=$logPath"
