# Logger Editor

## Purpose
Checking the security of built libraries by ensuring authority is non-transitive in designs.

## Background

![Logger Editor](/images/logger.jpg)

In figure above, the `Main` application has a `Logger` module that it can trust. The logger module has access to `FileIO`, providing it the authority to read/write to files. However, there also exists an extension named `wordCloud` module, which needs to utilize the `Logger` library to read/write to a place that only `Logger` can allow. Here, `Main` passes `Logger` as a when creating `wordCloud`. The goal of the user is to design `Logger` in such a way that `WordExt` does not have access to the underlying module `fileIO` and, in the process, escalate its privilege.

### Step 1 (~30 mins)

The following requirements need to be satisfied when designing the logger architecture for specific languages:


1. The name of the log file should be `log.txt`
2. The directory containing the file would depend on the language as follows:
   - **Rust** - should be in `$DATA_DIR` (since Rust's capability library provides inbuilt support for writing directly to that folder)
   - **Wyvern** - should be in the same folder as the program

The logger should contain the functionalities for the following (note that one can change the function names:

- `create_logger(logFile : String)` - A constructor which returns a new logger object with the name logFile
- `append_to_log(entry : String)` - Append a new entry to the logFile

**Rust**
Given a template `extension.rs`, `main.rs`  file [1] - design the corresponding Logger module with capability library in Rust ([2], [3])

A potential template is given in `logger.rs`:

```rust
/* Some imports the user may / may not need */
// use cap_directories::{ambient_authority, ProjectDirs};
// use cap_std::fs::{Dir, OpenOptions};
// use std::ffi::OsString;
// use std::io::Write;
// use std::path::PathBuf;

pub struct Logger {
    // TODO: Define the fields for logger structure
}

impl Logger {
    pub fn create_logger(rel_file_name: &str) -> anyhow::Result<Self> {
        Ok(Logger {})
    }
    pub fn append_to_log(&self, entry: String) -> anyhow::Result<()> {
        Ok(())
    }
}
```

The following documentation may be useful

[1] https://docs.rs/cap-std/0.26.1/cap_std/

[2] https://docs.rs/cap-directories/0.26.1/cap_directories/

[3] https://doc.rust-lang.org/std/

**Wyvern**

The extension library is `wordCloud`, and the users need to design the logger library from scratch with the `appendLog` function. Since capability is built hierarchically here, the users were given a more open-ended specification to the parameters of the logger library (the answer here is that using the underlying system resource is exclusively dependent that Main passes `fileIO` from the main function. 

```rust
import fileSystem
import logger
import wordFactory
import wordCloud

val fs = fileSystem(java)
val logFile = fs.fileFor("log.txt")

// val logger = ??
val word = wordFactory.create("temp")
val wordCloud = wordCloud(logger, wordFactory, word)
wordCloud.updateCloud()
```

The documentation for Wyvern's fileSystem was provided as standard IO library, hoping that it will act as self-documenting code since no developer-friendly documentation exists for the library. The required method definitions for `fileIO` is provided as `fileIO.wyv` in wyvern's stdlib.

### Step 2 (~20 mins)

Upon completing the corresponding functions, now try to break the security of the filesystem in the corresponding programs only by modifying `extension.rs` (for Rust) and `wordCloud.wyv` (for Wyvern).

### Step 3 (~10 mins)

Please provide your ratings out of 5 on the following:

1. How useful do you think capabilities are?
2. How much did you like working on Wyvern?
3. How much did you like working on Rust?
4. How much do you think you understand the concept of capabilities?

**Subjective questions**:
Is there a part of the language / task design which the participant would want to be improved?
