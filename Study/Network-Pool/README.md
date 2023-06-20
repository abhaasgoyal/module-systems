# Network Pool



## Purpose
Ease of developing appropriate implementations from the specification. A high degree of freedom should be provided in how to design programs.

## Goal

Firewalls are essential to control the authority of incoming/outgoing network traffic. The user has to implement a basic functionality by making two types of network pools for interfacing using inbuilt capability functions in Rust/Wyvern with an untrusted extension. The extension promises us to only connect with the website \texttt{example.com}. The architecture can be represented with figure below

![Network Pool](/images/network.jpg)

### Step 1 (~30 mins)

The user has to make two types of network pools for interfacing using inbuilt capability functions in Rust/Wyvern, for connecting with the website `example.com`.

There are two types of pools used in the extension. The pools should have the following capability guards when being connected by the extension:

- **Net-Port** - Only allow in IP address of `93.184.216.14`; however, the connection should be within the range of a TCP port (0-65535)
- **TCP-Port** - Only allow connections on a small range of IP addresses for any port. The last 8 bits of the IP addresses should be in the range `93.184.216.<0-255>`.

**Rust**

1. Within `pool_auth.rs` , create the respective network pools by looking at the necessary documentation [1]

```rust
pub fn create_tcp_port() -> Pool {
}

pub fn create_net_port() -> Pool {
}
```

2. Then, call in the extension by passing in the Pools with the required IP address and HTTP port in both cases as the input

[1] https://docs.rs/cap-std/1.0.15/cap_std/net/struct.Pool.html


**Wyvern**
1. The `makePool` module should have 4 input parameters - `startIp`, `endIp`, `startPort`, and `endPort`.
2. Then, come up with an abstraction of functions for `Net-Port` and `TCP-Port` which call `makePool`. This should be within the `main` function.
3. Finally, the `connect(addr, port)` function should consist of a guard which checks whether the `addr` and `port` are within the acceptable range, and provides the response of specific privileges.



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

