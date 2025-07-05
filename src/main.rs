mod read_file;
mod data_analyzer;

use read_file::{read_file, save_dataset_to_csv, extract_subset};
use data_analyzer::DataAnalyzer;
use anyhow::Result;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "analyze" => {
            if args.len() < 3 {
                println!("Usage: {} analyze <file_path>", args[0]);
                return Ok(());
            }
            analyze_file(&args[2])?;
        }
        "demo" => {
            run_demo()?;
        }
        "help" => {
            print_usage();
        }
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("데이터 분석기 (Data Analyzer)");
    println!("Usage:");
    println!("  cargo run analyze <file_path>  - Analyze a CSV or Excel file");
    println!("  cargo run demo                 - Run demonstration with sample data");
    println!("  cargo run help                 - Show this help message");
    println!();
    println!("Features:");
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

    // 각 열에 대해 분석 수행
    for header in &dataset.headers {
        println!("\n분석 중인 열: {}", header);

        // 숫자 데이터인지 확인하고 기초통계량 계산
        if let Ok(stats) = analyzer.analyze_column(&dataset, header) {
            analyzer.print_basic_stats(&stats, header);

            // 그래프 생성
            if let Ok(numeric_data) = dataset.get_numeric_column(header) {
                let safe_header = header.replace(" ", "_").replace("/", "_");

                // Box Plot 생성
                let box_plot_path = format!("boxplot_{}.png", safe_header);
                if let Err(e) = analyzer.create_box_plot(&numeric_data, 
                    &format!("Box Plot - {}", header), &box_plot_path) {
                    println!("Box plot 생성 실패: {}", e);
                }

                // QQ Plot 생성
                let qq_plot_path = format!("qqplot_{}.png", safe_header);
                if let Err(e) = analyzer.create_qq_plot(&numeric_data, 
                    &format!("QQ Plot - {}", header), &qq_plot_path) {
                    println!("QQ plot 생성 실패: {}", e);
                }

                // Histogram 생성
                let histogram_path = format!("histogram_{}.png", safe_header);
                if let Err(e) = analyzer.create_histogram(&numeric_data, 
                    &format!("Histogram - {}", header), &histogram_path, 20) {
                    println!("Histogram 생성 실패: {}", e);
                }
            }
        } else {
            // 문자열 데이터의 경우 빈도 분석
            if let Ok(freq_data) = analyzer.analyze_column_frequency(&dataset, header) {
                analyzer.print_frequency_data(&freq_data, header);
            }
        }
    }

    // 표본 추출 예시
    if dataset.row_count() > 10 {
        println!("\n=== 표본 추출 예시 ===");
        let sample_size = (dataset.row_count() / 2).min(100);

        if let Ok(sample) = analyzer.random_sample(&dataset, sample_size) {
            println!("무작위 표본 추출 완료: {} 행", sample.row_count());
            let sample_path = format!("{}_random_sample.csv", 
                Path::new(file_path).file_stem().unwrap().to_str().unwrap());
            if let Err(e) = save_dataset_to_csv(&sample, &sample_path) {
                println!("표본 저장 실패: {}", e);
            } else {
                println!("표본이 {}에 저장되었습니다.", sample_path);
            }
        }
    }

    // 특정 열 추출 예시
    if !dataset.headers.is_empty() {
        println!("\n=== 열 추출 예시 ===");
        let first_column = &dataset.headers[0];
        let columns_to_extract = vec![first_column.clone()];

        if let Ok(subset) = extract_subset(&dataset, None, Some(columns_to_extract)) {
            let subset_path = format!("{}_column_{}.csv", 
                Path::new(file_path).file_stem().unwrap().to_str().unwrap(),
                first_column.replace(" ", "_"));
            if let Err(e) = save_dataset_to_csv(&subset, &subset_path) {
                println!("열 추출 파일 저장 실패: {}", e);
            } else {
                println!("'{}' 열이 {}에 저장되었습니다.", first_column, subset_path);
            }
        }
    }

    Ok(())
}

fn run_demo() -> Result<()> {
    println!("=== 데이터 분석기 데모 실행 ===");

    // 샘플 데이터 생성
    create_sample_data()?;

    // 생성된 샘플 데이터 분석
    analyze_file("sample_data.csv")?;

    println!("\n데모가 완료되었습니다!");
    println!("생성된 파일들:");
    println!("- sample_data.csv: 샘플 데이터");
    println!("- *.png: 생성된 그래프들");
    println!("- *_sample.csv: 표본 추출 결과");
    println!("- *_column_*.csv: 열 추출 결과");

    Ok(())
}

fn create_sample_data() -> Result<()> {
    use std::fs::File;
    use std::io::Write;
    use rand::Rng;

    let mut file = File::create("sample_data.csv")?;

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

    println!("샘플 데이터 'sample_data.csv'가 생성되었습니다.");
    Ok(())
}
