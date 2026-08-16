[CmdletBinding()]
param(
    [string]$Root = $PSScriptRoot,
    [int]$Limit = 0,
    [int]$Offset = 0,
    [string]$Language = 'ja',
    [switch]$KeepRejected
)

$ErrorActionPreference = 'Stop'
$tools = Join-Path $PSScriptRoot '.lyrics-tools'
$whisper = Join-Path $tools 'whisper\Release\whisper-cli.exe'
$model = Join-Path $tools 'ggml-large-v3-turbo-q5_0.bin'
$ffmpeg = Get-ChildItem (Join-Path $tools 'ffmpeg') -Recurse -File -Filter 'ffmpeg.exe' |
    Select-Object -First 1 -ExpandProperty FullName
$work = Join-Path $tools 'work'
$logPath = Join-Path $Root 'audio-lyrics-scan.csv'
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)

foreach ($required in @($whisper, $model, $ffmpeg)) {
    if (-not $required -or -not [System.IO.File]::Exists($required)) {
        throw "Required tool is missing: $required"
    }
}
[System.IO.Directory]::CreateDirectory($work) | Out-Null

function Normalize-Line([string]$Text) {
    if (-not $Text) { return '' }
    return (($Text.Normalize([Text.NormalizationForm]::FormKC).ToLowerInvariant()) -replace '[\s\p{P}\p{S}]', '')
}

function Test-VocalTranscript([string]$JsonPath) {
    $jsonText = [System.IO.File]::ReadAllText($JsonPath, [System.Text.Encoding]::UTF8)
    $json = $jsonText | ConvertFrom-Json
    $lines = @($json.transcription | ForEach-Object { Normalize-Line $_.text } |
        Where-Object { $_.Length -ge 2 })
    $unique = @($lines | Sort-Object -Unique)
    $largestRepeat = if ($lines.Count) {
        ($lines | Group-Object | Sort-Object Count -Descending | Select-Object -First 1).Count
    } else { 0 }
    $uniqueRatio = if ($lines.Count) { $unique.Count / $lines.Count } else { 0 }
    $characterCount = (($lines -join '')).Length
    $joined = $lines -join '|'
    $encodedHallucinations = @(
        '44GU6KaW6IG044GC44KK44GM44Go44GG44GU44GW44GE44G+44GX44Gf',
        '44OB44Oj44Oz44ON44Or55m76Yyy',
        '5L2c6KmeLirkvZzmm7IuKue3qOabsg==',
        '5Yid6Z+z44Of44Kv',
        '5a2X5bmVLiroppbogbQ='
    )
    $hallucinationPattern = ($encodedHallucinations | ForEach-Object {
        [System.Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($_))
    }) -join '|'
    $hallucinationHits = ([regex]::Matches($joined, $hallucinationPattern)).Count

    $accepted = $lines.Count -ge 4 -and
        $unique.Count -ge 3 -and
        $uniqueRatio -ge 0.20 -and
        $largestRepeat -le [math]::Max(10, [math]::Ceiling($lines.Count * 0.45)) -and
        $characterCount -ge 30 -and
        $hallucinationHits -lt [math]::Max(6, [math]::Ceiling($lines.Count * 0.25))

    return [pscustomobject]@{
        Accepted = $accepted
        Lines = $lines.Count
        UniqueLines = $unique.Count
        UniqueRatio = [math]::Round($uniqueRatio, 3)
        LargestRepeat = $largestRepeat
        Characters = $characterCount
        HallucinationHits = $hallucinationHits
    }
}

function Remove-HallucinatedLrcLines([string]$Lyrics) {
    $encodedPhrases = @(
        '44GU6KaW6IG044GC44KK44GM44Go44GG44GU44GW44GE44G+44GX44Gf',
        '44OB44Oj44Oz44ON44Or55m76Yyy',
        '5L2c6KmeLirkvZzmm7IuKue3qOabsg==',
        '5Yid6Z+z44Of44Kv',
        '5a2X5bmVLiroppbogbQ='
    )
    $pattern = ($encodedPhrases | ForEach-Object {
        [System.Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($_))
    }) -join '|'
    $cleaned = @($Lyrics -split "\r?\n" | Where-Object {
        $_ -notmatch $pattern
    })
    return (($cleaned -join "`r`n").TrimEnd() + "`r`n")
}

$rows = New-Object System.Collections.Generic.List[object]
$processedFiles = @{}
if ([System.IO.File]::Exists($logPath)) {
    try {
        $existingRows = Import-Csv -LiteralPath $logPath -Encoding UTF8
        foreach ($r in $existingRows) {
            $rows.Add($r)
            if ($r.Status -eq 'rejected-instrumental-name' -or $r.Status -eq 'created-from-audio') {
                $processedFiles[$r.File] = $true
            }
        }
    } catch {}
}

$vocalPathsFile = Join-Path $tools 'vocal_paths.txt'
if ([System.IO.File]::Exists($vocalPathsFile)) {
    $files = @(Get-Content -LiteralPath $vocalPathsFile | ForEach-Object {
        if ($_.Trim()) { Get-Item -LiteralPath $_.Trim() }
    } | Where-Object {
        $relPath = $_.FullName.Substring($Root.TrimEnd('\').Length).TrimStart('\')
        if ($processedFiles.ContainsKey($relPath)) { return $false }
        $lrc = [System.IO.Path]::ChangeExtension($_.FullName, '.lrc')
        if (-not [System.IO.File]::Exists($lrc)) { return $true }
        try {
            $content = [System.IO.File]::ReadAllText($lrc, [System.Text.Encoding]::UTF8)
            return $content -notmatch '\[\d{2}:\d{2}'
        } catch {
            return $true
        }
    } | Sort-Object FullName)
} else {
    $files = @(Get-ChildItem -LiteralPath $Root -Recurse -File -Filter '*.flac' |
        Where-Object { $_.FullName -notlike "$tools*" } |
        Where-Object {
            $relPath = $_.FullName.Substring($Root.TrimEnd('\').Length).TrimStart('\')
            if ($processedFiles.ContainsKey($relPath)) { return $false }
            $lrc = [System.IO.Path]::ChangeExtension($_.FullName, '.lrc')
            if (-not [System.IO.File]::Exists($lrc)) { return $true }
            try {
                $content = [System.IO.File]::ReadAllText($lrc, [System.Text.Encoding]::UTF8)
                return $content -notmatch '\[\d{2}:\d{2}'
            } catch {
                return $true
            }
        } |
        Sort-Object FullName)
}
if ($Offset -gt 0) { $files = @($files | Select-Object -Skip $Offset) }
if ($Limit -gt 0) { $files = @($files | Select-Object -First $Limit) }

# $rows was initialized at startup to preserve history
$created = 0
$rejected = 0
$existing = 0
$errors = 0
$index = 0

foreach ($file in $files) {
    $index++
    $lrcPath = [System.IO.Path]::ChangeExtension($file.FullName, '.lrc')
    $needsTranscribe = $true
    if ([System.IO.File]::Exists($lrcPath)) {
        try {
            $content = [System.IO.File]::ReadAllText($lrcPath, [System.Text.Encoding]::UTF8)
            if ($content -match '\[\d{2}:\d{2}') {
                $needsTranscribe = $false
                $existing++
            }
        }
        catch {
            # If reading fails, assume we need to transcribe
        }
    }
    
    if (-not $needsTranscribe) {
        continue
    }

    $status = ''
    $detail = ''
    $stem = [guid]::NewGuid().ToString('N')
    $wavPath = Join-Path $work "$stem.wav"
    $outputBase = Join-Path $work $stem
    $jsonPath = "$outputBase.json"
    $generatedLrc = "$outputBase.lrc"

    try {
        if ($file.BaseName -match '(?i)(^|[\s._\-\(\[])instrumental([\s._\-\)\]]|$)|karaoke|off[\s._-]*vocal|inst\.?\s*$') {
            $status = 'rejected-instrumental-name'
            $rejected++
        }
        else {
            Write-Host "[$index/$($files.Count)] Analyzing: $($file.FullName)"
            & $ffmpeg -y -v error -i $file.FullName -ar 16000 -ac 1 -c:a pcm_s16le $wavPath
            if ($LASTEXITCODE -ne 0) { throw "FFmpeg failed with exit code $LASTEXITCODE" }

            $oldErrorPreference = $ErrorActionPreference
            $ErrorActionPreference = 'Continue'
            try {
                & $whisper -m $model -f $wavPath -l $Language -olrc -ojf -of $outputBase -t 8 -np 2>$null | Out-Null
                $whisperExitCode = $LASTEXITCODE
            }
            finally {
                $ErrorActionPreference = $oldErrorPreference
            }
            if ($whisperExitCode -ne 0) { throw "Whisper failed with exit code $whisperExitCode" }
            if (-not [System.IO.File]::Exists($jsonPath) -or -not [System.IO.File]::Exists($generatedLrc)) {
                throw 'Whisper did not produce the expected output files.'
            }

            $quality = Test-VocalTranscript $jsonPath
            $detail = "lines=$($quality.Lines); unique=$($quality.UniqueLines); ratio=$($quality.UniqueRatio); repeat=$($quality.LargestRepeat); chars=$($quality.Characters); hallucinations=$($quality.HallucinationHits)"
            if ($quality.Accepted) {
                $lyrics = [System.IO.File]::ReadAllText($generatedLrc, [System.Text.Encoding]::UTF8)
                $lyrics = Remove-HallucinatedLrcLines $lyrics
                [System.IO.File]::WriteAllText($lrcPath, $lyrics, $utf8NoBom)
                $status = 'created-from-audio'
                $created++
                
                # Auto-translate generated Japanese lyrics to English
                Write-Host "  -> Translating generated Japanese lyrics to English..."
                $scriptPath = Join-Path $PSScriptRoot "translate_lrc.py"
                & py $scriptPath $lrcPath | Out-Null
            }
            else {
                $status = 'rejected-low-vocal-confidence'
                $rejected++
                if ($KeepRejected) {
                    [System.IO.File]::Copy($generatedLrc, [System.IO.Path]::ChangeExtension($file.FullName, '.rejected.lrc'), $true)
                }
            }
        }
    }
    catch {
        $status = 'error'
        $detail = $_.Exception.Message
        $errors++
        Write-Warning "$($file.FullName): $detail"
    }
    finally {
        foreach ($temporary in @($wavPath, $jsonPath, $generatedLrc)) {
            if ([System.IO.File]::Exists($temporary)) {
                [System.IO.File]::Delete($temporary)
            }
        }
    }

    $rows.Add([pscustomobject]@{
        File = $file.FullName.Substring($Root.TrimEnd('\').Length).TrimStart('\')
        Status = $status
        Detail = $detail
    })
    $rows | Export-Csv -LiteralPath $logPath -NoTypeInformation -Encoding UTF8
}

Write-Output "Scanned=$($files.Count) Existing=$existing Created=$created Rejected=$rejected Errors=$errors"
Write-Output "Log=$logPath"
