use brotli_dict_gen::generate_dict_from_files;
use clap::Parser;
use std::fs;
use std::io::Write;

type BoxError = Box<dyn std::error::Error>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    /// Path of the generated dictionary
    output: String,

    #[arg(trailing_var_arg = true)]
    files: Vec<String>,
}

fn main() -> Result<(), BoxError> {
    let args = Args::parse();

    let mut files = vec![];
    for file in &args.files {
        let data = fs::read(file).map_err(|err| format!("failed to read file '{file}': {err}"))?;
        files.push(data);
    }

    let dictionary = generate_dict_from_files(files)
        .map_err(|err| format!("failed to generate dictionary: {err}"))?;

    let mut file = fs::File::create(&args.output)?;
    file.write_all(dictionary.as_bytes())?;

    eprintln!("wrote dictionary {}", args.output);
    Ok(())
}
