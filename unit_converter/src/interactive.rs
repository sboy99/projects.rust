use crate::converters;
use anyhow::Result;
use inquire::{Select, Text};

pub fn run() -> Result<()> {
    let category =
        Select::new("Select a category", vec!["Temperature", "Length", "Weight"]).prompt()?;
    let from = Text::new("From unit").prompt()?;
    let to = Text::new("To unit").prompt()?;
    let value = Text::new("Value").prompt()?.parse::<f64>()?;
    match category.to_string().as_str() {
        "Temperature" => {
            let result = converters::convert_temperature(&from, &to, &value)?;
            println!("{} {} = {} {}", value, from, result, to);
        }
        "Length" => {
            let result = converters::convert_length(&from, &to, &value)?;
            println!("{} {} = {} {}", value, from, result, to);
        }
        "Weight" => {
            let result = converters::convert_weight(&from, &to, &value)?;
            println!("{} {} = {} {}", value, from, result, to);
        }
        _ => return Err(anyhow::anyhow!("Unknown category.")),
    }
    Ok(())
}
