use std::error::Error;
use std::fs;
use std::io;
use std::io::{stdout, BufReader, BufWriter, Read, Seek, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dump", about = "an minimal hexdump")]
struct Opt {
    file: String,

    #[structopt(short, long)]
    seek: usize,

    #[structopt(short, long)]
    length: usize,
}

struct ViewInfo {
    filename: String,
    seek: usize,
    length: usize,
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

fn buffered_writer() -> BufWriter<io::Stdout> {
    let writer = BufWriter::new(stdout());
    writer
}

fn buffered_reader(filename: &str) -> Result<BufReader<fs::File>, Box<dyn Error>> {
    let handle = fs::File::open(filename)?;
    let reader = BufReader::new(handle);
    Ok(reader)
}

fn hex_view(info: ViewInfo) -> Result<(), Box<dyn Error>> {
    let mut writer = buffered_writer();
    let mut reader = buffered_reader(&info.filename)?;

    let mut buffer;
    if info.length < 16 {
        buffer = Vec::with_capacity(info.length);
        buffer.resize(info.length, 0);
    } else {
        buffer = Vec::with_capacity(16);
        buffer.resize(16, 0);
    }
    reader.seek(io::SeekFrom::Current(info.seek as i64))?;
    let mut total = 0;
    let mut bytecount;
    while total < info.length {
        bytecount = reader.read(&mut buffer)?;
        if bytecount == 0 {
            break;
        };
        let row = line_view(&buffer, total);
        writer.write_all(row.as_bytes())?;
        total += bytecount;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();

    let hexinfo = ViewInfo {
        filename: args.file,
        length: args.length,
        seek: args.seek,
    };

    hex_view(hexinfo)?;

    Ok(())
}
