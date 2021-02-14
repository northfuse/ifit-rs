extern crate serde;
extern crate scraper;

mod parser;
mod client;

pub use parser::WorkoutSummary;
pub use client::IfitClient;
