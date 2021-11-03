use std::error::Error;
use std::fs;
use std::io::{stdout, BufWriter, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dump", about = "an minimal hexdump")]
struct Opt {
    file: String,
}

fn to_hex(byte: u8) -> String {
    format!("{:02x}", byte)
}

fn hex_view(bytes: &[u8]) {
    let mut writer = BufWriter::new(stdout());

    let mut column = 0;
    for byte in bytes {
        if column % 16 != 0 {
            write!(writer, "{} ", to_hex(*byte));
        } else {
            write!(writer, "{}\n", to_hex(*byte));
        }

        column += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();

    let content = fs::read(args.file)?;

    hex_view(&content);

    Ok(())
}
