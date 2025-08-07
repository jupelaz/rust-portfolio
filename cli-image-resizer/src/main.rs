use clap::Parser;
use image::imageops::FilterType;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
    #[arg(long)]
    width: u32,
    #[arg(long)]
    height: u32,
}

fn main() {
    let args = Args::parse();
    let img = image::open(&args.input).expect("cannot open input");
    let resized = img.resize(args.width, args.height, FilterType::Nearest);
    resized.save(&args.output).expect("cannot save image");
}
