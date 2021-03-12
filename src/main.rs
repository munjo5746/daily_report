use std::io::Write;

use chrono::prelude::*;
use serde::Serialize;
use serde_json::to_string;

#[derive(Debug, Serialize)]
struct HourlyReport {
    timestamp: i64,
    tasks: Vec<String>,
}

fn main() {
    let mut tasks = Vec::new();
    println!("Describe each task you have completed in the past an hour (quit to finish).");

    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read task.");

        let input = input.trim().to_owned();

        if input.as_str() == "quit" {
            break;
        }

        tasks.push(input);
    }

    let datetime = Utc::now();
    let report = HourlyReport {
        timestamp: datetime.timestamp(),
        tasks,
    };

    // create daily_report directory
    let daily_report_root_path = dirs::home_dir().unwrap().join(".daily_report");
    std::fs::create_dir_all(daily_report_root_path.clone())
        .expect("directory for daily report could not be created.");

    let formatted_date = datetime.format("%Y_%m_%d").to_string();
    let file_name = format!("{0}{1}", formatted_date, ".txt");

    let daily_report_file_path = daily_report_root_path.clone().join(file_name);
    let mut file: std::fs::File;
    if !daily_report_file_path.exists() {
        file = std::fs::File::create(daily_report_file_path).unwrap();
    } else {
        file = std::fs::OpenOptions::new().append(true).open(daily_report_file_path).unwrap();
    }
    let json = to_string(&report).unwrap();

    writeln!(file, "{}", json).unwrap();

}
