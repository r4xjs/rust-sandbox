use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn gen_name(len: u8) -> Result<String> {
    let mut rnd = File::open("/dev/urandom")?;
    let mut buf: [u8; 8] = [0; 8];

    let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz135679"
	.chars()
	.collect();
    let name: String = (0..len)
	.map(|_| {
	    rnd.read(&mut buf).unwrap();
	    let idx = usize::from_le_bytes(buf);
	    alpha[idx % (alpha.len()-1)]
	})
	.collect();
    
    Ok(name)
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let len = if args.len() > 1 {
	args[1].parse()?
    } else {
	5
    };

    for _ in 0..100 {
	println!("{}", gen_name(len)?);
    }

    Ok(())
}
