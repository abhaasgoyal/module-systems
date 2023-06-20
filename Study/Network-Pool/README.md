# Network Pool

![](/static/network.jpg)

## Rust and Wyvern
### Step 1 (~30 mins)

The user has to make two types of network pools for interfacing using inbuilt capability functions in Rust/Wyvern, for connecting with the website example.com.

There are two types of pools used in the extension. The pools should have the following capability guards when being connected by the extension:

Net-Port -> Only allow in IP address of 93.184.216.14, however the connection should be within the range of a TCP port
TCP-Port -> Allow connections on a small range of IP addresses. Here, IP addresses should be in the range of 93.184.216.<0-255>

**Rust**

Within pool_auth.rs , create the respective network pools by looking at the necessary documentation [1]

```rust
pub fn create_tcp_port() -> Pool {
}

pub fn create_net_port() -> Pool {
}
```

2. Then, call in the extension by passing in the Pools with the required IP address and HTTP port in both cases as the input

[1] https://docs.rs/cap-std/1.0.15/cap_std/net/struct.Pool.html


**Wyvern**
The makePool module should have 4 input parameters - startIp, endIp, startPort, and endPort.
Then, come up with an abstraction of functions for Net-Port and TCP-Port which call makePool. This should be within the main function.
Finally, the connect(addr, port) function should consist of a guard which checks whether the addr and port are within the acceptable range, and provides the response of specific privileges.



### Step 2 (~20 mins)

Upon completing the corresponding functions, now try to break the security of the filesystem in the corresponding programs only by modifying extension.rs (for Rust) and cloud.wyv (for Wyvern).

### Step 3 (~10 mins)

Please provide your ratings out of 5 on the following:

1. How useful do you think capabilities are?
2. How much did you like working on Wyvern?  
3. How much did you like working on Rust?
4. How much do you think you understand the concept of capabilities?

**Subjective questions:**
Is there a part of the language / task design which the participant would want to be improved?  

