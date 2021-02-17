use scraper::{Html, Selector, ElementRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct WorkoutSummary {
    pub id: String,
    pub title : String,
    pub date : String,
    pub distance : String,
}

pub fn parse_workout_summaries(raw_html : String) -> Result<Vec<WorkoutSummary>, Box<dyn std::error::Error>> {
    let workout_html = Html::parse_document(raw_html.as_str());
    let table_selector = Selector::parse(".workout-table").unwrap();
    let row_selector = Selector::parse(".workoutRows").unwrap();
    let mut workouts : Vec<WorkoutSummary> = vec![];
    for element in workout_html.select(&table_selector) {
        for row in element.select(&row_selector) {
            let workout = parse_workout_summary_row(row)?;
            println!("#{:?}", workout);
            workouts.push(workout);
        }
    }

    Ok(workouts)
}

fn parse_workout_summary_row(row : ElementRef) -> Result<WorkoutSummary, Box<dyn std::error::Error>> {
    let title_selector = Selector::parse(".title a").unwrap();

    let mut title : Option<String> = None;
    let mut id : Option<String> = None;

    for e in row.select(&title_selector) {
        title = Some(e.inner_html());
        let tokens : Option<Vec<&str>> = e.value().attr("href").map(|href| href.rsplit('/').collect());
        id = tokens.map(|v| v.first().unwrap().to_string());
    }

    let date_selector = Selector::parse(".six").unwrap();
    let mut date : Option<String> = None;
    for e in row.select(&date_selector) {
        date = Some(e.inner_html());
    }

    let distance_selector = Selector::parse(".three").unwrap();
    let mut distance : Option<String> = None;
    for e in row.select(&distance_selector) {
        distance = e.text().next().map(|s| s.to_string());
    }

    Ok(WorkoutSummary {
        title: title.unwrap(),
        id: id.unwrap(),
        date: date.unwrap(),
        distance: distance.unwrap(),
    })
}

#[cfg(test)]
mod test {
    use std::fs;
    use super::*;

    #[test]
    fn parses_workout_summaries() -> Result<(), Box<dyn std::error::Error>> {
        let raw_workout = fs::read_to_string("test/workouts.html")?;
        let workouts = parse_workout_summaries(raw_workout)?;
        assert_eq!(workouts, vec![
            WorkoutSummary { id: "602826f48212a500463b6afa".to_string(), title: "Manual Workout".to_string(), date: "Feb 13, 2021".to_string(), distance: "3.6".to_string() },
            WorkoutSummary { id: "6027231fdb64d3003e21828b".to_string(), title: "Manual Workout".to_string(), date: "Feb 12, 2021".to_string(), distance: "3.7".to_string() },
            WorkoutSummary { id: "6018eb584ac635187c8f39fc".to_string(), title: "Manual Workout".to_string(), date: "Feb 1, 2021".to_string(), distance: "5.0".to_string() },
            WorkoutSummary { id: "601261c6306dcb03b1734a2e".to_string(), title: "Manual Workout".to_string(), date: "Jan 27, 2021".to_string(), distance: "3.0".to_string() },
            WorkoutSummary { id: "60110a08d4350f123459ef7b".to_string(), title: "Manual Workout".to_string(), date: "Jan 26, 2021".to_string(), distance: "5.2".to_string() },
            WorkoutSummary { id: "600fa69556513b21cc3e8eb9".to_string(), title: "Manual Workout".to_string(), date: "Jan 25, 2021".to_string(), distance: "5.2".to_string() },
            WorkoutSummary { id: "600a5aae708e8c0ea38ab118".to_string(), title: "Manual Workout".to_string(), date: "Jan 21, 2021".to_string(), distance: "6.0".to_string() },
            WorkoutSummary { id: "60027edbfee5a7094749f0ef".to_string(), title: "Manual Workout".to_string(), date: "Jan 15, 2021".to_string(), distance: "3.6".to_string() },
            WorkoutSummary { id: "5ffe3614ecbe470db5058230".to_string(), title: "Manual Workout".to_string(), date: "Jan 12, 2021".to_string(), distance: "3.6".to_string() },
            WorkoutSummary { id: "5ffd3af551f6c80f81709d82".to_string(), title: "Manual Workout".to_string(), date: "Jan 11, 2021".to_string(), distance: "6.4".to_string() }
        ]);
        Ok(())
    }
}
