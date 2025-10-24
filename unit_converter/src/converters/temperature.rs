use anyhow::Result;

pub fn convert(from: &String, to: &String, value: &f64) -> Result<f64> {
    println!("Converting temperature from {} to {}", from, to);
    Ok(*value)
}
