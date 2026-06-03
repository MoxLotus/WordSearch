use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Args {
    #[arg(long, value_parser = parse_size, default_value = "10x10")]
    size: (u32, u32),
}

fn parse_size(s: &str) -> Result<(u32, u32), String> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return Err("Expected format WIDTHxHEIGHT".into());
    }
    let width = parts[0].parse::<u32>().map_err(|_| "Invalid width")?;
    let height = parts[1].parse::<u32>().map_err(|_| "Invalid height")?;
    Ok((width, height))
}

fn main() {
    let args = Args::parse();
    let w = args.size.0;
    let h = args.size.1;

    let grid: Vec<Vec<char>> = (0..h)
        .map(|_| {
            (0..w)
                .map(|_| rand::thread_rng().gen_range(b'A'..=b'Z') as char)
                .collect()
        })
        .collect();

    //TODO: Add word placement logic here

    for row in &grid {
        for &ch in row {
            print!("{}  ", ch);
        }
        println!();
    }

    //TODO: Print the list of words to find here
}
