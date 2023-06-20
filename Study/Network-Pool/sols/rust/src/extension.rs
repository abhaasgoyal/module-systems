use std::io;
use cap_std::net::Pool;

pub fn pool_ext(pool: &Pool) -> io::Result<()> {
    // Connect to example.com
    match pool.connect_tcp_stream("93.184.216.34:80") {
        Ok(mut _stream) => {
            println!("Connected to example.com");
            // REVIEW: Add HTTP GET Request
        }
        Err(e) => {
            // Handle unauthorized access
            eprintln!("Error connecting: {}", e);
        }
    }

    Ok(())
}
