use clap::Parser;
use std::path::Path;
use walkdir::WalkDir;
use fs_extra::file::{self, CopyOptions};
use std::collections::HashSet;

const SUPPORTED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "heic", "raw", "cr2", "nef",
    "mp4", "mov", "avi", "mkv", "wmv", "flv", "webm", "m4v", "3gp"
];

#[derive(Parser, Debug)]
#[command(name = "photosears")]
#[command(about = "디렉터리에서 이미지와 비디오 파일을 수집하는 도구")]
#[command(help_template = "\n{about}\n\n사용법: {usage}\n\n{after-help}")]
#[command(after_help = "지원하는 파일 형식:
이미지: jpg, jpeg, png, gif, bmp, tiff, webp, heic, raw, cr2, nef
비디오: mp4, mov, avi, mkv, wmv, flv, webm, m4v, 3gp

사용 예시:
  ./photosears \"~/Pictures/photos\" \"~/Desktop/backup\"
  ./photosears \"/Volumes/DRIVE/사진\" \"/Users/backup\"")]
struct Args {
    #[arg(required = true, index = 1, help = "미디어 파일을 검색할 디렉터리 경로\n공백이 포함된 경로는 큰따옴표로 감싸주세요")]
    source_dir: String,

    #[arg(required = true, index = 2, help = "미디어 파일을 저장할 디렉터리 경로\n공백이 포함된 경로는 큰따옴표로 감싸주세요")]
    destination_dir: String,
}

fn main() {
    let args = Args::parse();
    let supported_extensions: HashSet<&str> = SUPPORTED_EXTENSIONS.iter().copied().collect();
    
    if !Path::new(&args.source_dir).exists() {
        eprintln!("오류: 소스 디렉터리를 찾을 수 없습니다: {}", args.source_dir);
        return;
    }

    if let Err(e) = std::fs::create_dir_all(&args.destination_dir) {
        eprintln!("오류: 대상 디렉터리를 생성하지 못했습니다: {}\n{}", args.destination_dir, e);
        return;
    }
    
    let mut copied_files = 0;
    let mut skipped_files = 0;
    let copy_options = CopyOptions::new();
    
    for entry in WalkDir::new(&args.source_dir) {
        match entry {
            Ok(entry) => {
                if !entry.file_type().is_file() {
                    continue;
                }
                
                let path = entry.path();
                
                if let Some(extension) = path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            let file_name = path.file_name().unwrap();
                            let destination = Path::new(&args.destination_dir).join(file_name);
                            
                            if destination.exists() {
                                println!("건너뜀 (이미 존재): {}", path.display());
                                skipped_files += 1;
                            } else {
                                match file::copy(path, &destination, &copy_options) {
                                    Ok(_) => {
                                        println!("복사됨: {}", path.display());
                                        copied_files += 1;
                                    }
                                    Err(e) => eprintln!("파일 복사 실패 {}: {}", path.display(), e),
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("디렉터리 탐색 중 오류 발생: {}", e),
        }
    }
    
    println!("\n작업 완료!");
    println!("복사된 파일: {} 개", copied_files);
    println!("건너뛴 파일: {} 개 (중복)", skipped_files);
    println!("총 처리된 파일: {} 개", copied_files + skipped_files);
}
