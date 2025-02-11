# PhotoSears

A Rust-based CLI tool for collecting image and video files from directories.

[한국어 문서](README_ko.md)

## Features

- Recursively search for media files in specified directories
- Automatically copy all supported media files to target directory
- Automatic duplicate detection and skipping
- Real-time copy progress display

## Supported File Types

### Images
- jpg, jpeg, png, gif, bmp, tiff, webp
- heic, raw, cr2, nef

### Videos
- mp4, mov, avi, mkv
- wmv, flv, webm, m4v, 3gp

## Usage

```bash
./photosears "source_directory_path" "target_directory_path"
```

### Examples
```bash
./photosears "~/Pictures/photos" "~/Desktop/backup"
./photosears "/Volumes/DRIVE/photos" "/Users/backup"
```

## Installation

1. Make sure you have Rust installed
2. Build with:
```bash
cargo build --release
```

## Dependencies

- walkdir: Directory traversal
- clap: CLI interface
- fs_extra: File operation utilities
