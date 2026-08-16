# MusicBee Feature Gap Analysis & Feature Roadmap

Comprehensive feature audit comparing our native Rust + Web Audio Player system (`rust_server`, CPAL/Symphonia audio engine, SQLite DB, Web UI) against **MusicBee**.

---

## Mobile Device & Phone Synchronization Suite

MusicBee features one of the most comprehensive phone sync engines available for Windows, seamlessly bridging desktop audio collections with mobile devices:

### 1. Dual Connection Protocols (Wired & Wireless)
* **Wired MTP / USB Storage Sync**: Plug-and-play USB connection for Android devices, iPhones/iPods (via MTP/Apple device drivers), and dedicated Hi-Res DAPs (Digital Audio Players).
* **Wi-Fi Sync (MusicBee Wi-Fi Sync App)**: Wireless local network sync using the native MusicBee Wi-Fi Sync Android app—initiates fast 2-way sync over Wi-Fi without needing a USB cable.

### 2. Two-Way Data & Play Stat Synchronization
* **Play Count & Rating Sync**: Syncs mobile play counts, last played timestamps, and star ratings back to the PC MusicBee SQLite database.
* **Bi-Directional Playlist Sync**: Changes made to playlists on mobile player apps (e.g., Poweramp, Symfonium, GoneMAD) sync back to desktop playlists.
* **Podcast & Audiobook Resume State**: Syncs exact playback time offsets so audiobooks resume on mobile exactly where left off on desktop.

### 3. On-The-Fly Dynamic Transcoding
* **Format & Sample Rate Auto-Conversion**: Automatically down-samples or converts high-res/unsupported formats (e.g., 24-bit/192kHz FLAC, ALAC, DSD) to 320kbps MP3, AAC, or OPUS on-the-fly during transfer without altering source files on PC.
* **Artwork Resizing for Mobile**: Resizes massive album art (e.g. 3000×3000px down to 500×500px) to save mobile RAM and storage.

### 4. Smart Storage Management & Filtering
* **Rule-Based Sync Profiles**: Selectively sync by specific playlists, genres, artists, or minimum star rating (e.g. "Sync all 4+ star tracks").
* **Storage Auto-Fill (Auto-DJ for Phone)**: Fills remaining free space on internal storage or MicroSD card with random unplayed or top-rated tracks up to a defined storage cap.
* **Custom Mobile Folder Structure**: Define mobile file paths (`/Music/<Artist>/<Album>/<Track#> - <Title>.mp3`).

---

## Panel Management System (UI Flexibility)

MusicBee's panel management engine gives complete control over workspace geometry and information layout:

### 1. Graphical Panel Arranger
* **Drag-and-Drop Docking**: Visual layout editor to dock, stack, resize, or float any UI element across 6 main zones (Left Sidebar, Left Main, Main Body, Right Main, Right Sidebar, Top Header, Bottom Bar).
* **Auto-Hide & Pinning**: Pin panels open or set sidebars to auto-hide, revealing themselves on mouse hover or hotkey press.
* **Per-Tab Panel Layouts**: Different tabs can maintain entirely distinct panel arrangements (e.g. Tab 1: 3-column spreadsheet view; Tab 2: Full-bleed Album Wall).

### 2. Multi-Tab Right Sidebar Context Inspector
* **Dockable Inspector Stack**: Right panel houses stacked tabs for Live Lyrics, Artist Biography (Wikipedia/Last.fm), Track Info/Metadata, Queue, and Similar Artists.
* **Detachable Floating Windows**: Any panel (Lyrics, Visualizer, Equalizer, Queue) can be popped out into an independent floating window for multi-monitor setups.

### 3. Customizable Header & Transport Control Bar
* **Relocatable Player Controls**: Playback controls, waveform seeker, and volume slider can be placed at the bottom, top header, or embedded directly inside the navigation bar.

---

## Library Management System

MusicBee is renowned for its enterprise-grade audio file and metadata management engine:

### 1. Library Inbox (Staging Ground)
* **Isolated Import Zone**: New downloads or un-tagged rips land in the "Inbox" staging panel first. Tracks are scrubbed, tagged, and auto-named before being committed to the main library database.

### 2. Auto-Organize & File Relocation Engine
* **Continuous File Re-structuring**: Rules engine (`<Genre>/<Artist>/<Year> - <Album>/<Track#> <Title>`) that continuously moves and renames physical files on disk as tags are modified.
* **Multi-Drive Consolidation**: Maps files scattered across internal SSDs, external HDDs, and network NAS shares (`\\NAS\Music`) into a unified virtual library.

### 3. Filter Category Tree & Multi-Column Drilldown
* **Hierarchical Filter Panes**: Cascading filter panes (Genre → Artist → Album → Year → Bitrate/Format) with real-time live track counts and instant search refinement.
* **Folder Hierarchy Explorer**: Direct file-system tree view integrated alongside tag-based category browser.

### 4. Duplicates & Database Health Maintenance
* **Duplicate Track Finder**: Audio stream fingerprinting and tag matching tool to isolate and merge duplicate tracks.
* **Orphan & Dead Link Cleaner**: Sweeps directory trees to delete dead database links or discover un-indexed audio files.
* **Multi-Library Switching**: Instant switching between isolated SQLite databases (e.g., Music, Audiobooks, DJ Crates, Classical).

---

## Playback, DSP & Audio Engineering Suite

### 1. Advanced DSP, VST Chains & Headphone Crossfeed
* **Bauer BS2B Headphone Crossfeed**: Built-in Chu Moy / BS2B headphone crossfeed algorithm reducing spatial listening fatigue on headphones.
* **WASAPI Exclusive DAC Pre-roll / Warm-up Frame**: Sends 50ms silence frame on WASAPI stream open to wake up external USB DACs (Schiit, Topping, iFi) and prevent initial transient clipping.
* **Instant Buffer Flush on Seek**: Immediate audio queue buffer flush on manual position seek for zero-latency seeking.
* **Auto-Pause on Device Disconnect**: Automatically pauses audio playback if headphones or Bluetooth audio endpoints disconnect.
* **VST Plugin Chain Preset Manager**: Chain multiple VST effects (Equalizer → Dynamic Compressor → Limiter) with loadable preset profiles.
* **Hardware DAC Clock Switching**: WASAPI Exclusive bit-exact sample rate matching dynamically switching DAC hardware clock between 44.1kHz, 48kHz, 96kHz, 192kHz based on source FLAC.
* **ITU BS.1770 / EBU R128 Loudness Scanner**: Modern loudness standard calculation alongside legacy ReplayGain 2.0.
* **32-Bit Floating Point Pipeline**: Internal 32-bit fp processing pipeline ensuring lossless DSP EQ gain without digital clipping.

### 2. Auto-DJ & Intelligent Queueing
* **Rule-Based Auto-DJ**: Dynamic queueing matching BPM/tempo, genre affinity, Last.fm recommendation score, and artist repetition limits (e.g. "Do not play same artist within 60 minutes").
* **Dual-Pane Queue Staging**: Split-pane queue manager for instant track prioritization, temporary play-next ordering, and queue locking.
* **Per-Track Start/Stop Offsets**: Custom non-destructive start/stop timestamp cue offsets per track (skip intros or early fades without modifying file).

### 3. Search, Acoustic Fingerprinting & Web Lookup
* **Full-Text Indexed Fast Search**: Instant sub-millisecond search across title, artist, album, lyrics, comments, and virtual tags.
* **AcoustID / Chromaprint Matching**: Acoustic fingerprint matching against MusicBrainz to identify un-tagged raw audio files without tags.
* **Custom Web Search Integrations**: Right-click quick lookup on Youtube, Discogs, Bandcamp, RateYourMusic, or Google.

### 4. System, Protocols & Remote Integration
* **Custom URI Protocol Scheme**: Native `musicbee://play?track=...` URI handler allowing external web pages or scripts to trigger playback.
* **Automated Library Backup on Exit**: Configurable automated database backup triggers on application shutdown.
* **Windows Taskbar Progress & Overlay**: Taskbar button playback progress indicator fill and mini playback controls.
* **Wi-Fi Remote App Protocol**: Native API protocol supporting official MusicBee Remote apps on Android and iOS.
* **Hotkey Macro Commander**: Custom multi-action macro shortcuts (e.g. `Rate 5 Stars + Add to Playlist + Copy File Path` in one keypress).
* **100% Portable Edition Mode**: Zero-registry self-contained mode running fully off USB flash drives.

---

## UI & Design Aesthetics Breakdown (Why MusicBee UI is World-Class)

MusicBee's user interface is widely regarded as one of the best desktop media player UIs due to its balance of high information density, deep visual customizability, and slick aesthetics:

### 1. Zero-Waste Modular Panel Architecture
* **Total Docking Freedom**: Every section (Left Navigation, Main View, Right Context Panel, Top Header, Player Bar) can be resized, collapsed, split, or rearranged via drag-and-drop.
* **Multi-Tab Workspace**: Tabbed browsing (like a modern web browser), allowing users to keep multiple playlists, searches, and artist views open at once without losing context.
* **Right Sidebar Context Inspector**: Displays live scrolling lyrics, artist Wikipedia bios, high-res fanart photos, and upcoming queue side-by-side with the active library view.

### 2. Adaptive Theming & Dynamic Palette Engine
* **Album Art Color Match**: UI accent highlights, progress bars, and backdrop gradients dynamically adapt to match the dominant color palette of the playing album artwork.
* **Deep Skinning Engine**: Supports 100+ skin themes ranging from dark Fluent/Material designs to classic retro analog Hi-Fi hardware interfaces.

### 3. Flexible Multi-View Presentation Modes
* **Album Wall View**: High-density grid of album covers with smooth expansion cards.
* **Artwork + Track List Split View**: Master-detail view showing album art grid alongside expandable tracklists.
* **Artist Picture Grid**: Visual grid of artist photos scraped automatically from Last.fm / Fanart.tv.
* **High-Density Spreadsheet View**: Power-user grid display for managing 100,000+ track libraries with custom column width, alignment, and conditional color highlighting.
* **Theater Mode**: Full-screen cinematic presentation displaying moving fanart wallpapers, audio visualizer canvas, and synced lyrics.
* **Taskbar & Floating Mini-Players**: Borderless floating widget mini-player, always-on-top compact player, and native taskbar toolbar widget.

### 4. Visual Navigation & Feedback
* **Pre-Rendered Waveform Seekbar**: High-resolution audio peak amplitude visualization allowing visual navigation directly to drops, quiet passages, or choruses.
* **Quick Filter Pills**: Instant one-click library filtering by Hi-Res, Unrated, Recently Added, or Genre.

---

## Detailed Feature Matrix & Gap Breakdown

### 1. Tagging & Metadata Management
- [ ] **In-App ID3 / FLAC Tag Editor**: In-place GUI editor for ID3v2, FLAC/Vorbis, and M4A tags directly in player UI.
- [ ] **Auto-Tagger & Scraper**: Automatic metadata lookup via MusicBrainz, Discogs, and Last.fm API match engine.
- [ ] **AcoustID / Chromaprint Fingerprinting**: Audio fingerprinting to identify untagged audio tracks.
- [ ] **Tag Sanitizer**: Remove unwanted ID3 tags (`COMMENT`, `COMM`, `PRIV`, `ENCODER`, iTunes normalization tags).
- [ ] **Batch Tag Copy/Paste & Swap**: Swap Artist and Title tags across selected tracks in one click.
- [ ] **Genre Manager & Standardizer**: Consolidate variant genres (`Hip Hop`, `HipHop` → `Hip-Hop`).
- [ ] **Multi-Condition File Auto-Organize Rules**: Conditional file/folder restructuring (`If Lossless -> FLAC/<Artist>/; If Soundtrack -> Soundtracks/<Album>/`).
- [ ] **Track & Disc Re-Numbering Assistant**: Auto-number multi-disc releases (Disc X of Y, Track X of Y) with disc subtitle tagging.
- [ ] **Custom Tags & Virtual Tags**: User-defined schema fields and dynamic expression tags (`$If(<Album Artist>,<Album Artist>,<Artist>)`).
- [ ] **Embedded Synced & Unsynced Lyrics**: Multi-provider fallback chain (Local LRC -> Embedded SYLT -> Genius -> Musixmatch -> LrcLib -> AZLyrics) with per-line ms timestamp fine-tuning.
- [ ] **Batch Artwork Downloader & Embedder**: Fetch and embed covers directly into audio file headers (`PICTURE`/`APIC`).
- [ ] **Multi-Artwork Manager**: Manage multiple image types per track (Front Cover, Back Cover, Artist Photo, CD Label, Band Logo).
- [ ] **Mass Case Conversion & Encoding Fixer**: Title case conversion rules and Shift-JIS/GBK to UTF-8 character encoding fixer.
- [ ] **Classical Music Metadata Hierarchy**: Dedicated tagging structure for Composer, Conductor, Ensemble, Opus Number, Movement Name/Number.
- [ ] **Auto-Tag from Filename**: Regex file parser to construct track tags directly from structured filenames.
- [ ] **Library Inbox**: Staging area to review, tag, and organize new audio downloads before committing to library database.
- [ ] **CUE Sheet Tools**: Native split and export interface for FLAC/APE + CUE image files.

### 2. Audio Processing & Playback Engine
- [ ] **32-Bit Floating Point Audio DSP Pipeline**: Internal 32-bit fp audio processing pipeline to prevent digital clipping before volume scaling.
- [ ] **WASAPI DAC Pre-roll Frame**: Send silence pre-roll frame to wake external USB DACs without clipping initial audio transients.
- [ ] **Instant Buffer Flush on Seek**: Zero-latency queue buffer flush on position seek.
- [ ] **Auto-Pause on Endpoint Disconnect**: Pause playback immediately when headphones/Bluetooth disconnect.
- [ ] **VST Plugin Chain Preset Manager**: Chain multiple VST effects with loadable preset profiles.
- [ ] **Hardware DAC Sample Rate Passthrough**: Dynamic switching of DAC hardware clock between 44.1kHz, 48kHz, 96kHz, 192kHz based on source file.
- [ ] **BS2B Headphone Crossfeed**: Chu Moy / BS2B spatial headphone crossfeed processor.
- [ ] **ITU BS.1770 / EBU R128 Loudness Normalization**: EBU R128 loudness standard scanning and volume matching.
- [ ] **Native ASIO Driver Support**: Low-latency ASIO driver output alongside existing WASAPI Exclusive/Shared modes.
- [ ] **WASAPI Event / Push Mode & Microsecond Tuning**: Fine-grained buffer sizing slider and low-latency event-driven output to prevent DAC dropouts.
- [ ] **Automatic Audio Device Fallback**: Auto-switch output device to secondary soundcard/headphones on Bluetooth or DAC disconnect without playback interruption.
- [ ] **Bit-Perfect Output Telemetry Indicator**: Real-time bit-depth and sample rate delivery status light (e.g. 24-bit 96kHz bit-exact WASAPI status).
- [ ] **WASAPI Hardware Buffer Monitor**: Live buffer fill percentage gauge to diagnose audio driver latency under heavy system load.
- [ ] **Parametric Equalizer**: Multi-band parametric EQ (Q factor adjustment, high/low shelf filters, notch filters) alongside graphic EQ.
- [ ] **Mono Downmixing, L/R Swap & Phase Inversion**: Channel balance, stereo channel swapping, and mono summing options.
- [ ] **ReplayGain Engine**: Track/Album gain scanning, tag writing, and peak volume normalization playback.
- [ ] **Advanced Crossfading & Silence Stripping**: Custom track transition fades, fade-on-pause/stop, and silence removal.
- [ ] **VST & Winamp Plugin Host**: Host external VST effect plugins and Winamp DSP plugins.
- [ ] **Genre/Artist Auto-EQ**: Automatic DSP preset switching based on playing track's genre/artist tag.
- [ ] **Surround Sound Up/Down-Mixing**: 5.1/7.1 audio downmixing to stereo, or stereo upmixing to surround.
- [ ] **Time-Stretching & Pitch Shift**: Independent playback rate scaling without pitch distortion (0.5x–2.0x).
- [ ] **Sleep Timer & Auto-Shutdown**: Fade out audio and shutdown/hibernate PC after X minutes or at playlist end.
- [ ] **Winamp EQ Preset Import**: Loader for `.eqf` equalizer preset files.

### 3. Audio Conversion & CD Ripping
- [ ] **Format Transcoder & Converter**: Built-in batch audio converter (FLAC ↔ MP3 / AAC / OPUS) with customizable encoding presets.
- [ ] **CD Ripping Engine**: Audio CD ripping with AccurateRip checksum validation.
- [ ] **Duplicate Track Finder**: Audio fingerprinting, hash checking, and tag-based duplicate detection GUI.
- [ ] **Audio Stream Integrity Inspector**: FLAC MD5 audio stream verification scanner to detect corrupted files.

### 4. Smart Playlists, Queue & Analytics
- [ ] **Rule-Based Auto-DJ Engine**: Tempo/BPM matching, artist repeat limits, and Last.fm score weighting.
- [ ] **Per-Track Non-Destructive Cue Offsets**: Custom start/stop playback timestamps per file.
- [ ] **Nested Playlist Folders**: Group playlists hierarchically inside nested tree folders (`Playlists > Fitness > Running`).
- [ ] **Playlist Lock & Freeze**: Lock playlists to prevent accidental track deletion, re-ordering, or modification.
- [ ] **Dynamic Auto-Playlists**: Rule-based smart playlists using nested boolean logic (`Genre = Rock AND PlayCount > 10 AND LastPlayed > 30 days`).
- [ ] **Weighted Shuffle Algorithm**: Rating-aware, play-count-aware, and recency-aware shuffle algorithm.
- [ ] **Half-Star Granular Rating System**: 5-star rating system supporting half-stars (1–10 rating scale) with custom view highlights.
- [ ] **Play Analytics Dashboard**: Graphical charts for listening history, top genres, skip rates, and playback trends.
- [ ] **Party / Auto-DJ Mode**: Smart rule-based auto-queuing with guest lockdown mode.

### 5. Web, Online & System Integrations
- [ ] **Custom URI Protocol Scheme**: Handler for `musicbee://play?track=...` web protocol links.
- [ ] **Two-Way Last.fm Play Count Sync**: Sync scrobble count and last played dates from Last.fm back into local DB.
- [ ] **Custom Web Search Integrations**: Right-click web search lookup on Youtube, Discogs, Bandcamp, RateYourMusic, or Google.
- [ ] **Wi-Fi Mobile Remote App Protocol**: Remote playback control server for official Android/iOS apps.
- [ ] **Windows Taskbar Progress Overlay**: Taskbar button playback progress fill and mini-controls.
- [ ] **Offline Scrobble Queue**: Cache scrobbles locally when offline and auto-submit batch scrobbles to Last.fm/ListenBrainz when connected.
- [ ] **Windows Explorer Context Menu Shell Extension**: Native right-click "Play in MusicBee" and "Queue in MusicBee" Explorer integration.
- [ ] **Discord Rich Presence**: Real-time Discord status showing song title, artist, album art, and elapsed time.
- [ ] **Windows System Media Controls (SMTC)**: Integration with Windows 10/11 OS flyout volume media controls and hardware media keys.
- [ ] **Live OBS Streamer Overlay Output**: Export currently playing track metadata to local text/image files for streaming software overlays.
- [ ] **Live Scrobbling & Love Sync**: Last.fm and ListenBrainz live scrobbling and favorite track synchronization.
- [ ] **Artist Bio & Fanart Scraper**: Integrated Wikipedia bios, discography, tour dates, and high-res fanart scraping.
- [ ] **Discography Completion Tracker**: Highlight missing releases in an artist's discography based on MusicBrainz database.
- [ ] **Podcasts & Internet Radio**: RSS podcast feed subscription manager and Shoutcast/Icecast stream directory.
- [ ] **Concert / Tour Notifications**: Live concert notification integration via Songkick / Bandsintown.
- [ ] **CDDB / Gracenote Lookup**: Compact disc database fingerprinting for un-tagged physical CDs.

### 6. Hardware & Mobile Sync
- [ ] **Mobile Device Synchronization**: MTP/USB and Wi-Fi sync to Android/iOS mobile devices.
- [ ] **On-The-Fly Transcoding for Sync**: Automatic conversion of unsupported lossless files during mobile sync.
- [ ] **Two-Way Mobile Play Count & Rating Sync**: Sync play stats and ratings from mobile player apps back to PC.
- [ ] **On-The-Fly Mobile Cover Art Resizing**: Downscale cover art during transfer to save mobile RAM/storage.
- [ ] **NAS / Network UNC Path Support**: Direct handling and monitoring of `\\NAS\Music` network shares and symlinks.

### 7. UI, Layout & Display
- [ ] **Drag-and-Drop Dockable Panel Layout**: Fully customizable multi-panel UI layout engine.
- [ ] **Multiple Main View Types**: Album Wall view, Tracks Grid view, Artwork view, Artist Picture Grid view, Compact List view.
- [ ] **Folder Hierarchy Explorer View**: Disk directory tree panel alongside artist/album tag views.
- [ ] **Right Sidebar Context Inspector**: Tabbed inspector showing Lyrics, Bio, Details, Queue, and Similar Artists simultaneously.
- [ ] **Quick Filter Pills**: Dedicated library quick-filters (`Unrated`, `Added This Week`, `Hi-Res Audio`, `Explicit`, `Missing Artwork`).
- [ ] **XML-Based Custom Theater Modes**: Full-screen skin templates with custom animated canvas renderers.
- [ ] **Dynamic Artwork Palette Theming**: Auto-theming UI accent colors dynamically extracted from playing album art.
- [ ] **Multi-Tab Interface**: Open playlists, search, and artist views in tabs simultaneously.
- [ ] **Compact / Mini-Player Skins & Taskbar Deskband**: Windows Taskbar toolbar deskband mini-player, borderless widget, and always-on-top window.
- [ ] **Interactive Waveform Seekbar**: Pre-rendered full-track amplitude waveform navigation bar.
- [ ] **Global OS Hotkeys**: System-wide configurable shortcut keys operating when app is minimized.
- [ ] **Desktop Notification Toasts**: OS-native desktop popups with mini playback and rating controls.
- [ ] **Visualizers**: Milkdrop / 3D spectrum audio visualizer engine integration.
- [ ] **Multi-Library Support**: Instant database switching between isolated libraries (e.g. Music, Audiobooks, Classical).

### 8. Database, Admin & Security Tools
- [ ] **Automated Database Backup on Exit**: Auto-trigger DB backup when closing application.
- [ ] **Hotkey Macro Commander**: Assign multi-action macro shortcuts to single hotkeys.
- [ ] **100% Portable Edition Mode**: Self-contained flash-drive execution without Windows Registry footprint.
- [ ] **One-Click Full Backup & Restore Wizard**: Package SQLite database, settings, skins, DSP presets, and playlists into `.zip`/`.mbz` archives.
- [ ] **Password-Protected Kiosk Mode**: Lockdown UI preventing file deletion, tag editing, or setting modifications during events.
- [ ] **Database Maintenance & Re-indexing**: SQLite compact/vacuum tools and fast search index rebuilds.
- [ ] **Extensible Plugin API**: Support for third-party C# / Web extensions.
- [ ] **Command Line Interface (CLI)**: Terminal controls for player automation (`player.exe /play`, `/next`, `/rate=5`).
- [ ] **Multi-Format Playlist Import/Export**: Support for `.m3u`, `.m3u8`, `.xspf`, `.pls`, `.wpl`, `.asx`.
- [ ] **Background Monitored Folders**: Real-time folder watching auto-importing newly saved audio files.
- [ ] **Dead Link & Orphan Scanner**: Sweep tool to remove dead file references or list un-indexed loose files.

---

## MoSCoW Prioritization Matrix

| Priority | Feature Area | Key Deliverables |
| --- | --- | --- |
| **MUST** | Library Management | Library Inbox Staging, Auto-Organize Relocation Rules, Multi-Column Drilldown |
| **MUST** | Tagging & Metadata | In-App Tag Editor, MusicBrainz Auto-Tagger, AcoustID Fingerprinting, File Renamer |
| **MUST** | Mobile & Phone Sync | Wired MTP / USB & Wi-Fi Sync App, 2-Way Play Count & Rating Sync, On-The-Fly Transcoding |
| **MUST** | Audio Engine | ReplayGain / EBU R128 Normalization, Hardware DAC Sample Rate Passthrough, WASAPI Bit-Perfect Indicator, Sleep Timer |
| **MUST** | UI & Panel Management | Graphical Panel Arranger, Multi-Tab Right Sidebar Inspector, Dynamic Artwork Color Match |
| **SHOULD** | Integrations | Discord Rich Presence, Windows SMTC Media Controls, OBS Streamer Output, Wi-Fi Remote Protocol, Custom URI Protocol |
| **SHOULD** | Playlists & Queue | Rule-Based Auto-DJ Engine, Dynamic Smart Playlists, Weighted Shuffle, Inbox, Half-Star Ratings, Per-Track Cue Offsets |
| **SHOULD** | Online Services | Last.fm Scrobbling, Two-Way Play Count Sync, Lyrics & Bio Auto-Scraper, Discography Tracker |
| **SHOULD** | Library Tools | Duplicate Track Finder, Format Converter, FLAC Integrity Scanner, Folder Tree View, Backup Wizard |
| **COULD** | Audio Engine | Native ASIO Support, VST Chain Presets, DAC Pre-roll Frame, Parametric EQ, VST/Winamp Plugin Host, 32-bit FP Pipeline |
| **COULD** | UI & Sync | Waveform Seekbar, MTP Device Sync, Milkdrop Visualizer, Taskbar Mini-Player, Theater Mode Skins, Hotkey Macro Commander |
| **WON'T** | Physical Media | Audio CD Ripping & Gracenote CDDB |
