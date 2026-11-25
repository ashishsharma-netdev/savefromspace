# SaveFromSpace Desktop

Cross-platform desktop download manager built with Tauri 2, React, TypeScript and Rust. Supports direct downloads and media downloads/conversion through yt-dlp + FFmpeg for sources you are authorized to download.

## Prerequisites
- Node.js 20+
- Rust stable
- Tauri prerequisites for your OS (Windows: MSVC Build Tools, WebView2)
- yt-dlp and FFmpeg installed and available on PATH (or configure their paths in Settings)

## Development
```bash
npm install
npm run tauri dev
```

## Build
```bash
npm run tauri build
```

## Features in initial scaffold
- Direct file download via backend
- Media URL download via yt-dlp
- Playlist mode
- Audio extraction/conversion (MP3, AAC, M4A, WAV, FLAC, OPUS)
- Download queue UI, progress/status display and history placeholder

Media site support depends on yt-dlp and the site's rules. DRM-protected content is not supported.
