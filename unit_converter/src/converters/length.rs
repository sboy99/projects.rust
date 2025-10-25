use anyhow::Result;

pub fn convert(from: &String, to: &String, value: &f64) -> Result<f64> {
    let result = match (from.as_str(), to.as_str()) {
        ("mm", "cm") => *value / 10.0,
        ("cm", "mm") => *value * 10.0,
        ("mm", "m") => *value / 1000.0,
        ("m", "mm") => *value * 1000.0,
        ("cm", "m") => *value / 100.0,
        ("m", "cm") => *value * 100.0,
        _ if from == to => *value,
        _ => return Err(anyhow::anyhow!("Invalid length unit")),
    };
    Ok(result)
}
