use anyhow::Result;

pub fn convert(from: &String, to: &String, value: &f64) -> Result<f64> {
    let result = match (from.as_str(), to.as_str()) {
        ("c", "f") => *value * 1.8 + 32.0,
        ("f", "c") => (*value - 32.0) / 1.8,
        ("c", "k") => *value + 273.15,
        ("k", "c") => *value - 273.15,
        ("f", "k") => (*value + 459.67) * 5.0 / 9.0,
        ("k", "f") => *value * 9.0 / 5.0 - 459.67,
        _ if from == to => *value,
        _ => return Err(anyhow::anyhow!("Invalid temperature unit")),
    };
    Ok(result)
}
