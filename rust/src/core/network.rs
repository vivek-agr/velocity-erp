// We use the std::net::UdpSocket. This logic should run in a background thread when the app starts.
use std::net::UdpSocket;
use std::time::Duration;
use std::thread;

const DISCOVERY_PORT: u16 = 5522; // Velocity-ERP reserved port
const DISCOVERY_MSG: &[u8] = b"VELOCITY_SERVER_DISCOVERY";

/// THE SERVER: Broadcaster (The machine holding the Data)
pub fn start_broadcaster() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to any local port
    socket.set_broadcast(true)?;

    println!("Server: Discovery broadcaster started...");

    thread::spawn(move || {
        loop {
            // Broadcast the server's presence to the entire subnet
            let broadcast_address = "255.255.255.255:5522";
            let _ = socket.send_to(DISCOVERY_MSG, broadcast_address);
            
            // Wait 5 seconds before next pulse (Tally-style heartbeat)
            thread::sleep(Duration::from_secs(5));
        }
    });

    Ok(())
}

/// THE CLIENT: Listener (The machine wanting to connect)
pub fn discover_server() -> std::io::Result<String> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", DISCOVERY_PORT))?;
    socket.set_read_timeout(Some(Duration::from_secs(10)))?;

    let mut buf = [0u8; 64];
    println!("Client: Searching for Velocity Server on network...");

    match socket.recv_from(&mut buf) {
        Ok((amt, src)) => {
            let msg = std::str::from_utf8(&buf[..amt]).unwrap_or("");
            if msg == "VELOCITY_SERVER_DISCOVERY" {
                println!("Client: Found server at {}", src.ip());
                return Ok(src.ip().to_string());
            }
        }
        Err(e) => return Err(e),
    }
    
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No server found"))
}