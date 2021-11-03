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

fn line_view(bytes: &[u8], byte_count: usize) -> String {
    let hexes: String = bytes.iter().map(|b| format!("{} ", to_hex(*b))).collect();
    let address = format!("0x{:08x?}", byte_count);
    let asciis: String = bytes
        .iter()
        .map(|b| {
            if !b.is_ascii_control() {
                *b as char
            } else {
                '.'
            }
        })
        .collect();
    format!("{} | {} | {} \n", address, hexes, asciis)
}

fn hex_view(bytes: &[u8]) {
    let mut writer = BufWriter::new(stdout());
    let elem_per_row = 16;

    let mut start = 0;
    while start + elem_per_row < bytes.len() {
        let line = line_view(&bytes[start..start + elem_per_row], start);
        write!(writer, "{}", line);
        start += elem_per_row;
    }
    write!(writer, "{}", line_view(&bytes[start..], start));
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();

    let content = fs::read(args.file)?;

    hex_view(&content);

    Ok(())
}
