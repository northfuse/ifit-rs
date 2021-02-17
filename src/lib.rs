extern crate serde;
extern crate scraper;
extern crate thiserror;
extern crate anyhow;

mod parser;
mod client;

pub use parser::WorkoutSummary;
pub use client::IfitClient;
