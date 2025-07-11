pub mod bar_chart;
pub mod pi_chart;
pub mod histogram;
pub mod box_plot;
pub mod stem_and_leaf;
pub mod dot_plot;

use crate::read_file::DataSet;
use crate::analyze::select_columns;
use anyhow::Result;
use std::io::{self, Write};

pub fn show_graph_menu(dataset: &DataSet) -> Result<()> {
    loop {
        println!("\n=== 그래프 그리기 ===");
        println!("그래프 종류를 선택하세요:");
        println!("1. 막대 그래프");
        println!("2. 원 그래프");
        println!("3. 히스토그램");
        println!("4. 상자 그림");
        println!("5. 줄기와 잎 그림");
        println!("6. 점도표");
        println!("7. 메인 메뉴로 돌아가기");
        print!("선택 (1-7): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        
        match choice {
            "1" => {
                if let Err(e) = bar_chart::create_bar_chart(dataset) {
                    println!("막대 그래프 생성 중 오류 발생: {}", e);
                }
            }
            "2" => {
                if let Err(e) = pi_chart::create_pie_chart(dataset) {
                    println!("원 그래프 생성 중 오류 발생: {}", e);
                }
            }
            "3" => {
                if let Err(e) = histogram::create_histogram(dataset) {
                    println!("히스토그램 생성 중 오류 발생: {}", e);
                }
            }
            "4" => {
                if let Err(e) = box_plot::create_box_plot(dataset) {
                    println!("상자 그림 생성 중 오류 발생: {}", e);
                }
            }
            "5" => {
                if let Err(e) = stem_and_leaf::create_stem_and_leaf(dataset) {
                    println!("줄기와 잎 그림 생성 중 오류 발생: {}", e);
                }
            }
            "6" => {
                if let Err(e) = dot_plot::create_dot_plot(dataset) {
                    println!("점도표 생성 중 오류 발생: {}", e);
                }
            }
            "7" => {
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