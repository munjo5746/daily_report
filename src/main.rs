use chrono::prelude::*;

#[derive(Debug)]
struct HourlyReport {
    datetime: DateTime<Utc>,
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

    let report = HourlyReport {
        datetime: Utc::now(),
        tasks,
    };

    // create daily_report directory
    let home_dir = dirs::home_dir().unwrap().join(".daily_report");
    std::fs::create_dir_all(home_dir).expect("directory for daily report could not be created.");

    println!("{:?}", report)
}
