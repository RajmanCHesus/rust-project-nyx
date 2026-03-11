use clap::Parser;
use nyx::error::NyxResult;

#[derive(Parser, Debug)]
#[command(name = "nyx")]
#[command(about = "Cross-modal transformation system", long_about = None)]
struct Args {
    /// Input file path
    #[arg(value_name = "FILE")]
    input: String,

    
    /// Output file path
    #[arg(value_name = "FILE")]
    output: String,

    /// Transformation mode: audio-to-image, image-to-audio, text-to-audio
    #[arg(short, long, default_value = "audio-to-image")]
    mode: String,
}

fn main() -> NyxResult<()> {
    let args = Args::parse();
    
    match args.mode.as_str() {
        "audio-to-image" => {
            println!("Transforming audio {} → {}", args.input, args.output);
            // TODO: Implement audio-to-image transformation
            println!("Not yet implemented");
        }
        "image-to-audio" => {
            println!("Transforming image {} → {}", args.input, args.output);
            // TODO: Implement image-to-audio transformation
            println!("Not yet implemented");
        }
        _ => {
            eprintln!("Unknown mode: {}", args.mode);
        }
    }

    Ok(())
}
