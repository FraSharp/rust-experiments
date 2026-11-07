use std::env;

const BYTES_PER_LINE: usize = 16;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let Some(path) = args.get(1) else {
        eprintln!("usage: fxxd <file>");
        std::process::exit(1);
    };

    let bytes = std::fs::read(path)?;

    for (i, chunk) in bytes.chunks(BYTES_PER_LINE).enumerate() {
        print!("{:08x}: ", i * BYTES_PER_LINE);

        for (j, byte) in chunk.iter().enumerate() {
            print!("{:02x}", byte);

            if j % 2 == 1 {
                print!(" ");
            }
        }

        for j in chunk.len()..BYTES_PER_LINE {
            print!("  ");

            if j % 2 == 1 {
                print!(" ");
            }
        }

        print!(" ");

        for byte in chunk {
            if matches!(byte, 0x20..=0x7e) {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }

        println!();
    }

    Ok(())
}
