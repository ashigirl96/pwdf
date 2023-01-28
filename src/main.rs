use anyhow::ensure;
use std::env;
use std::path::{Path, PathBuf};

fn validate(s: &Vec<String>) -> anyhow::Result<()> {
    ensure!(
        s.len() == 2,
        "pwdf takes 1 positional argument but 2 were given"
    );
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", current_dir.to_string_lossy());
        return Ok(());
    }
    validate(&args)?;
    let target_relative_path = Path::new(&args[1]);
    let file_name = current_dir.join(target_relative_path);
    let canonicalize = file_name.canonicalize()?;
    let file_path = canonicalize.to_string_lossy();
    println!("{file_path}");
    Ok(())
}
