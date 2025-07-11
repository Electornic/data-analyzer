pub mod descriptive_statistics;
pub mod frequency_distribution;
pub mod draw_graph;
pub mod t_test;

use crate::read_file::DataSet;
use anyhow::Result;
use std::io::{self, Write};

pub struct AnalyzeManager {
    dataset: DataSet,
}

impl AnalyzeManager {
    pub fn new(dataset: DataSet) -> Self {
        Self { dataset }
    }

    pub fn run_interactive_analysis(&self) -> Result<()> {
        loop {
            self.show_main_menu();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();

            match choice {
                "1" => {
                    if let Err(e) = descriptive_statistics::show_descriptive_statistics(&self.dataset) {
                        println!("기초 통계량 표시 중 오류 발생: {}", e);
                    }
                }
                "2" => {
                    if let Err(e) = frequency_distribution::show_frequency_distribution(&self.dataset) {
                        println!("빈도 표시 중 오류 발생: {}", e);
                    }
                }
                "3" => {
                    if let Err(e) = draw_graph::show_graph_menu(&self.dataset) {
                        println!("그래프 그리기 중 오류 발생: {}", e);
                    }
                }
                "4" => {
                    if let Err(e) = t_test::show_t_test_menu(&self.dataset) {
                        println!("t 검정 중 오류 발생: {}", e);
                    }
                }
                "5" => {
                    println!("분석을 종료합니다.");
                    break;
                }
                _ => {
                    println!("잘못된 선택입니다. 다시 선택해주세요.");
                }
            }
            
            println!("\n계속하려면 Enter를 누르세요...");
            let mut _continue = String::new();
            io::stdin().read_line(&mut _continue)?;
        }
        
        Ok(())
    }

    fn show_main_menu(&self) {
        println!("\n=== 데이터 분석 메뉴 ===");
        println!("데이터셋: {} 행, {} 열", self.dataset.row_count(), self.dataset.headers.len());
        println!("헤더: {:?}", self.dataset.headers);
        println!();
        println!("원하는 분석을 선택하세요:");
        println!("1. 기초 통계량 표시");
        println!("2. 빈도 표시");
        println!("3. 그래프 그리기");
        println!("4. t 검정");
        println!("5. 종료");
        print!("선택 (1-5): ");
        io::stdout().flush().unwrap();
    }
}

// 헬퍼 함수들
pub fn select_columns(headers: &[String], message: &str, allow_multiple: bool) -> Result<Vec<String>> {
    println!("\n{}", message);
    println!("사용 가능한 열:");
    for (i, header) in headers.iter().enumerate() {
        println!("{}. {}", i + 1, header);
    }
    
    if allow_multiple {
        println!("선택할 열 번호들을 쉼표로 구분하여 입력하세요 (예: 1,3,5):");
    } else {
        println!("선택할 열 번호를 입력하세요:");
    }
    
    print!("선택: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    
    let mut selected_columns = Vec::new();
    
    if allow_multiple {
        for part in input.split(',') {
            let index: usize = part.trim().parse::<usize>()
                .map_err(|_| anyhow::anyhow!("잘못된 번호 형식입니다"))?;
            if index == 0 || index > headers.len() {
                return Err(anyhow::anyhow!("잘못된 열 번호입니다: {}", index));
            }
            selected_columns.push(headers[index - 1].clone());
        }
    } else {
        let index: usize = input.parse()
            .map_err(|_| anyhow::anyhow!("잘못된 번호 형식입니다"))?;
        if index == 0 || index > headers.len() {
            return Err(anyhow::anyhow!("잘못된 열 번호입니다: {}", index));
        }
        selected_columns.push(headers[index - 1].clone());
    }
    
    Ok(selected_columns)
}