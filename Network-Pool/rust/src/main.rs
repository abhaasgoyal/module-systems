mod extension;
mod pool_auth;

use std::io;
use extension::pool_ext;

fn main() -> io::Result<()> {
    // Create the capability pool instances
    let tcp_pool = pool_auth::create_tcp_port();
    let net_pool = pool_auth::create_net_port();

    // TODO: Perform operations that requires authorization from the respective pools

    Ok(())
}
