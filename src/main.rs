use anyhow::ensure;
use std::env;

fn validate(s: &Vec<String>) -> anyhow::Result<()> {
    ensure!(s.len() >= 2, "Give the argument file name");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    validate(&args)?;
    let current_dir = env::current_dir()?;
    let file_name = current_dir.join(&args[1]);
    let file_name_str = file_name.into_os_string().into_string().unwrap();
    println!("{file_name_str}");
    Ok(())
}
