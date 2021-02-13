use std::collections::HashMap;
use reqwest::Client;
use crate::parser::{WorkoutSummary, parse_workout_summaries};

pub struct IfitClient {
    client: Client
}

impl IfitClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>>  {
        let client = Client::builder()
            .cookie_store(true)
            .build()?;
        Ok(IfitClient {
            client: client,
        })
    }

    pub async fn login(&self, username: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = HashMap::new();
        data.insert("email", username);
        data.insert("password", password);

        self.client.post("https://www.ifit.com/web-api/login")
            .json(&data)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_workout_summaries(&self) -> Result<Vec<WorkoutSummary>, Box<dyn std::error::Error>> {
        let raw_workouts = self.client.get("https://www.ifit.com/me/workouts")
            .send()
            .await?
            .text()
            .await?;

        let workouts = parse_workout_summaries(raw_workouts)?;
        Ok(workouts)
    }

    pub async fn download_tcx(&self, workout_id : String) -> Result<String, Box<dyn std::error::Error>> {
        let tcx_url = format!("https://www.ifit.com/workout/export/tcx/{}", workout_id);
        let tcx = self.client.get(tcx_url.as_str())
            .send()
            .await?
            .text()
            .await?;
        Ok(tcx)
    }

}
