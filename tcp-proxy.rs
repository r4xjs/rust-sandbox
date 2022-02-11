use std::net::{TcpStream, TcpListener};
use std::thread;
use std::io::{Write, Read};

/*
sudo sysctl net.ipv4.ip_forward=1
sudo iptables -t nat -A PREROUTING -p tcp -d <dest-addr> --dport <dest-port> -j DNAT --to-destination <redirect-to:5587>
sudo iptables -t nat -A POSTROUTING -j MASQUERADE
*/

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const RELAYTO: &str = "mail.gmx.net:587";

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:5587")?;
    while let Ok((client_socket, client_addr)) = listener.accept() {
	println!("Connection from: {:#}", &client_addr);

	handle_socket(client_socket).unwrap();
    }
    Ok(())
}

fn handle_socket(mut inbound: TcpStream) -> Result<()> {
    let mut inbound2  = inbound.try_clone()?;

    let mut outbound  = TcpStream::connect(RELAYTO)?;
    let mut outbound2 = outbound.try_clone()?;

    // read answer of outbound connection and forward to inbound connection
    thread::spawn(move || {
	loop {
	    let mut buf = [0;1024];
	    match outbound2.read(&mut buf) {
		Ok(0) | Err(_) => break,
		Ok(num) => {
		    //dbg!(String::from_utf8_lossy(&buf[..num]));
		    let mut message = String::from_utf8_lossy(&buf[..num]).into_owned();
		    if message.starts_with("250-gmx.net Hello") {
			dbg!("inject PIPELINING support into server response");
			message.push_str("250-PIPELINING\r\n"); 
			inbound2.write(&message.bytes().collect::<Vec<u8>>()).unwrap();
		    } else {
			inbound2.write(&buf[..num]).unwrap();
		    }
		},
	    };
	}
    });

    // read answer from inbound connetion and forward to outbound connection
    loop {
	let mut buf = [0;1024];
	match inbound.read(&mut buf) {
	    Ok(0) | Err(_) => break,
	    Ok(num) => {
		//dbg!(String::from_utf8_lossy(&buf[..num]));
		outbound.write(&buf[..num]).unwrap();
	    },
	};
    }
    Ok(())
}
