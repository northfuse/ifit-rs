use std::collections::HashMap;
use reqwest::Client;
use crate::parser::{WorkoutSummary, parse_workout_summaries};
use thiserror::Error;
use anyhow::{Context, Result};

pub struct IfitClient {
    client: Client
}

#[derive(Error, Debug)]
pub enum IfitError {
    #[error("invalid username/password")]
    InvalidUsernamePassword,
}

impl IfitClient {
    pub fn new() -> Result<Self >  {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .context("unable to build reqwest client")?;
        Ok(IfitClient {
            client: client,
        })
    }

    pub async fn login(&self, username: String, password: String) -> Result<()> {
        let mut data = HashMap::new();
        data.insert("email", username);
        data.insert("password", password);

        let response = self.client.post("https://www.ifit.com/web-api/login")
            .json(&data)
            .send()
            .await
            .context("unable to send login post")?;

        println!("{:?}", response);

        if response.status() == 401 {
            Err(IfitError::InvalidUsernamePassword)?
        }
        Ok(())
    }

    pub async fn list_workout_summaries(&self) -> Result<Vec<WorkoutSummary>> {
        let raw_workouts = self.client.get("https://www.ifit.com/me/workouts")
            .send()
            .await
            .context("unable to load workouts")?
            .text()
            .await
            .context("unable to load workout response body")?;

        let workouts = parse_workout_summaries(raw_workouts)?;
        Ok(workouts)
    }

    pub async fn download_tcx(&self, workout_id : String) -> Result<String > {
        let tcx_url = format!("https://www.ifit.com/workout/export/tcx/{}", workout_id);
        let tcx = self.client.get(tcx_url.as_str())
            .send()
            .await
            .context("unable to load tcs file")?
            .text()
            .await
            .context("unable to parse tcx response body")?;
        Ok(tcx)
    }

}
