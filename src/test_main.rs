use basexx::*;
use std::io::Read;

fn eprintln_and_exit(prog: &str) -> ! {
    eprintln!("[usage] {} -b {{56|58|32|64}} file", prog);
    std::process::exit(0);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (fnm, bxx) = if args.len() > 1 {
        match args[1].as_str() {
            "-b" => {
                let bxx = match args[2].as_str() {
                    "56" => 56,
                    "58" => 58,
                    "32" => 32,
                    "64" => 64,
                    _ => {
                        eprintln_and_exit(args[0].as_str());
                    }
                };
                (args[3].as_str(), bxx)
            }
            _ => {
                eprintln_and_exit(args[0].as_str());
            }
        }
    } else {
        eprintln_and_exit(args[0].as_str());
    };
    let mut file = std::fs::File::open(fnm)?;
    let mut inp = vec![];
    file.read_to_end(&mut inp)?;
    let oup = match bxx {
        56 => Base56::default().encode(&inp).unwrap(),
        58 => Base58::default().encode(&inp).unwrap(),
        32 => Base32::default().encode(&inp).unwrap(),
        64 => Base64::default().encode(&inp).unwrap(),
        _ => {
            eprintln_and_exit(args[0].as_str());
        }
    };
    println!("{}", oup);
    Ok(())
}
