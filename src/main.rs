use chrono::prelude::*;

#[derive(Debug)]
struct HourlyReport {
    datetime: DateTime<Utc>,
    tasks: Vec<String>,
}

fn main() {
    let report = HourlyReport {
        datetime: Utc::now(),
        tasks: vec![String::from("aa"), String::from("bb")],
    };
    println!("{:?}", report)
}
