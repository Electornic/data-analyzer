pub mod one_sample_t_test;
pub mod independent_samples_t_test;
pub mod paired_samples_t_test;

use crate::read_file::DataSet;
use anyhow::Result;
use std::io::{self, Write};

pub fn show_t_test_menu(dataset: &DataSet) -> Result<()> {
    loop {
        println!("\n=== t 검정 ===");
        println!("t 검정 종류를 선택하세요:");
        println!("1. 일표본 t 검정");
        println!("2. 독립표본 t 검정");
        println!("3. 대응표본 t 검정");
        println!("4. 메인 메뉴로 돌아가기");
        print!("선택 (1-4): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        
        match choice {
            "1" => {
                if let Err(e) = one_sample_t_test::perform_one_sample_t_test(dataset) {
                    println!("일표본 t 검정 중 오류 발생: {}", e);
                }
            }
            "2" => {
                if let Err(e) = independent_samples_t_test::perform_independent_samples_t_test(dataset) {
                    println!("독립표본 t 검정 중 오류 발생: {}", e);
                }
            }
            "3" => {
                if let Err(e) = paired_samples_t_test::perform_paired_samples_t_test(dataset) {
                    println!("대응표본 t 검정 중 오류 발생: {}", e);
                }
            }
            "4" => {
                println!("메인 메뉴로 돌아갑니다.");
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