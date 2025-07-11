mod read_file;
mod data_analyzer;

use read_file::{read_file, save_dataset_to_csv, extract_subset};
use data_analyzer::DataAnalyzer;
use analyze::AnalyzeManager;
use anyhow::Result;
use std::path::Path;
use std::io::{self, Write};

mod analyze;

fn main() -> Result<()> {
    // 인사말 출력
    println!("안녕하세요. 데이터 분석 프로그램입니다");
    println!();
    print_usage();

    loop {
        print!("\n명령어를 입력하세요 (help, analyze, demo, exit): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];

        match command {
            "analyze" => {
                if parts.len() < 2 {
                    println!("사용법: analyze <파일경로>");
                    println!("예시: analyze data.csv");
                    continue;
                }
                let file_path = parts[1];
                if let Err(e) = analyze_file(file_path) {
                    println!("파일 분석 중 오류가 발생했습니다: {}", e);
                }
            }
            "demo" => {
                if let Err(e) = run_demo() {
                    println!("데모 실행 중 오류가 발생했습니다: {}", e);
                }
            }
            "help" => {
                print_usage();
            }
            "exit" | "quit" | "종료" => {
                println!("프로그램을 종료합니다. 안녕히 가세요!");
                break;
            }
            _ => {
                println!("알 수 없는 명령어입니다: {}", command);
                println!("사용 가능한 명령어: help, analyze, demo, exit");
            }
        }
    }

    Ok(())
}

fn print_usage() {
    println!("데이터 분석기 (Data Analyzer)");
    println!("사용 가능한 명령어:");
    println!("  analyze <파일경로>  - CSV 또는 Excel 파일 분석");
    println!("  demo               - 샘플 데이터로 데모 실행");
    println!("  help               - 도움말 표시");
    println!("  exit               - 프로그램 종료");
    println!();
    println!("주요 기능:");
    println!("  - CSV/Excel 파일 읽기");
    println!("  - 기초통계량 계산 (평균, 중앙값, 표준편차 등)");
    println!("  - 빈도 분석");
    println!("  - 그래프 생성 (Box Plot, QQ Plot, Histogram)");
    println!("  - 특정 열/행 추출");
    println!("  - 표본 추출");
}

fn analyze_file(file_path: &str) -> Result<()> {
    println!("파일 분석 중: {}", file_path);

    // 파일 읽기
    let dataset = read_file(file_path)?;
    let analyzer = DataAnalyzer::new();

    // 데이터셋 요약 정보 출력
    analyzer.print_dataset_summary(&dataset);

    // 새로운 인터랙티브 분석 시스템 시작
    let analyze_manager = AnalyzeManager::new(dataset);
    analyze_manager.run_interactive_analysis()?;

    Ok(())
}

fn run_demo() -> Result<()> {
    println!("=== 데이터 분석기 데모 실행 ===");

    // 샘플 데이터 생성
    let sample_filename = create_sample_data()?;

    // 생성된 샘플 데이터 분석
    analyze_file(&sample_filename)?;

    println!("\n데모가 완료되었습니다!");
    println!("생성된 파일들:");
    println!("- {}: 샘플 데이터 (sample 폴더에 저장됨)", sample_filename);
    println!("- result/*.png: 생성된 그래프들 (result 폴더에 저장됨)");
    println!("- result/*_sample.csv: 표본 추출 결과 (result 폴더에 저장됨)");
    println!("- result/*_column_*.csv: 열 추출 결과 (result 폴더에 저장됨)");

    Ok(())
}

fn create_sample_data() -> Result<String> {
    use std::fs::File;
    use std::io::Write;
    use rand::Rng;

    // sample 디렉토리가 없으면 생성
    std::fs::create_dir_all("sample")?;

    // 파일명 결정 로직
    let mut filename = "sample/sample_data.csv".to_string();
    let mut counter = 1;

    // sample_data.csv가 이미 존재하면 번호를 붙여서 새 파일 생성
    while Path::new(&filename).exists() {
        filename = format!("sample/sample_data_{}.csv", counter);
        counter += 1;
    }

    let mut file = File::create(&filename)?;

    // CSV 헤더 작성
    writeln!(file, "이름,나이,점수,등급,도시")?;

    let names = ["김철수", "이영희", "박민수", "최지영", "정태호", "한소영", "임동현", "윤미래"];
    let grades = ["A", "B", "C", "D"];
    let cities = ["서울", "부산", "대구", "인천", "광주"];

    let mut rng = rand::thread_rng();

    // 100개의 샘플 데이터 생성
    for i in 0..100 {
        let name = format!("{}_{}", names[i % names.len()], i + 1);
        let age = rng.gen_range(20..65);
        let score = rng.gen_range(60.0..100.0);
        let grade = grades[rng.gen_range(0..grades.len())];
        let city = cities[rng.gen_range(0..cities.len())];

        writeln!(file, "{},{},{:.1},{},{}", name, age, score, grade, city)?;
    }

    println!("샘플 데이터 '{}'가 생성되었습니다.", filename);
    Ok(filename)
}
