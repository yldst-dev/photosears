# PhotoSears

디렉터리에서 이미지와 비디오 파일을 쉽게 수집하는 Rust 기반 CLI 도구입니다.

[English Document](README.md)

## 주요 기능

- 지정된 디렉터리에서 이미지와 비디오 파일을 재귀적으로 검색
- 지원하는 모든 미디어 파일을 대상 디렉터리로 자동 복사
- 중복 파일 자동 감지 및 건너뛰기
- 복사 진행 상황 실시간 표시

## 지원하는 파일 형식

### 이미지
- jpg, jpeg, png, gif, bmp, tiff, webp
- heic, raw, cr2, nef

### 비디오
- mp4, mov, avi, mkv
- wmv, flv, webm, m4v, 3gp

## 사용법

```bash
./photosears "소스_디렉터리_경로" "대상_디렉터리_경로"
```

### 예시
```bash
./photosears "~/Pictures/photos" "~/Desktop/backup"
./photosears "/Volumes/DRIVE/사진" "/Users/backup"
```

## 설치

1. Rust가 설치되어 있어야 합니다.
2. 다음 명령어로 빌드:
```bash
cargo build --release
```

## 의존성

- walkdir: 디렉터리 재귀 탐색
- clap: CLI 인터페이스
- fs_extra: 파일 작업 유틸리티
