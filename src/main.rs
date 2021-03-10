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

    println!("{:?}", report)
}
