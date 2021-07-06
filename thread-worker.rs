use std::thread;
use std::io::{Read};
use std::fs;
use std::sync::{Mutex, Arc, mpsc::channel};
use std::time::{Duration, Instant};

fn rand() -> u32 {
    let mut fd = fs::File::open("/dev/urandom").unwrap();
    let mut buf = [0; 4];
    fd.read(&mut buf).unwrap();
    return u32::from_le_bytes(buf);
}

const NTHREAD: u32 = 10;

fn main() {
    let start_time = Instant::now();
    let input: Vec<u32> = (0..20).collect();
    let input = Arc::new(Mutex::new(input));
    let (tx, rx) = channel();

    // start worker threads
    for id in 0..NTHREAD {
	let input = Arc::clone(&input);
	let tx = tx.clone();
	thread::spawn(move || {
	    println!("Worker {} started", id);
	    loop {
		let val = match input.lock().unwrap().pop() {
		    Some(v) => v,
		    None => break,
		};
		thread::sleep(Duration::from_secs((rand() % 9).into()));
		tx.send(val+1).unwrap();
	    }
	    drop(tx);
	    println!("Worker {} stopped", id);
	});
    }
    // drop tx for main thread
    drop(tx);

    while let Ok(ret) = rx.recv() {
	println!("recv: {}", ret);
    }

    println!("Finished in {:?}", start_time.elapsed());
}

