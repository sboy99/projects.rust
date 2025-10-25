use anyhow::Result;

pub fn convert(from: &String, to: &String, value: &f64) -> Result<f64> {
    let result = match (from.as_str(), to.as_str()) {
        ("kg", "g") => *value * 1000.0,
        ("g", "kg") => *value / 1000.0,
        ("kg", "mg") => *value * 1000000.0,
        ("mg", "kg") => *value / 1000000.0,
        ("g", "mg") => *value * 1000.0,
        ("mg", "g") => *value / 1000.0,
        _ if from == to => *value,
        _ => return Err(anyhow::anyhow!("Invalid weight unit")),
    };
    Ok(result)
}
