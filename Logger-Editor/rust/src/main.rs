//! A simple illustration of the `cap-std` API to stores data in a project's directory

use cap_directories::{ambient_authority, ProjectDirs};
use std::path::PathBuf;

mod logging;
mod extension;


fn main() -> anyhow::Result<()> {
    // Parse command-line arguments.
    // TODO: Show how using relative path names can lead to problems
    let key: PathBuf = PathBuf::from("../key");
    let value = "new_value4";

    // Obtain the `data_dir` for this program.
    let project_dirs = ProjectDirs::from(
        "com.example",
        "Example Organization",
        "Cap-std Key-Value CLI Example",
        ambient_authority(),
    ).unwrap();
    let mut data_dir = project_dirs.data_dir()?;

    // Create new directory and set data_dir to that path
    let parent = key.parent();
    let file_name = key.file_name().unwrap();
    if let Some(parent) = parent {
        if !parent.as_os_str().is_empty() {
            data_dir.create_dir_all(parent)?;
            data_dir = data_dir.open_dir(parent)?;
        }
    }

    // Write the value in the new file.
    data_dir.write(file_name, value)?;

    // TODO: Show create a logging instance and be able to write to it

    Ok(())
}













// fn function() {
//     println!("called `function()`");
// }

// mod file_io;

// // pub use file_io;

// // pub use file_io::Write;

// mod cool {
//     pub fn function() {
//         println!("called `cool::function()`");
//         crate::file_io::write();
//     }
// }

// mod my {
//     fn function() {
//         println!("called `my::function()`");
//     }

//     mod cool {
//         pub fn function() {
//             println!("called `my::cool::function()`");
//         }
//     }

//     pub fn indirect_call() {
//         // Let's access all the functions named `function` from this scope!
//         print!("called `my::indirect_call()`, that\n> ");

//         // The `self` keyword refers to the current module scope - in this case `my`.
//         // Calling `self::function()` and calling `function()` directly both give
//         // the same result, because they refer to the same function.
//         self::function();
//         function();

//         // We can also use `self` to access another module inside `my`:
//         self::cool::function();

//         // The `super` keyword refers to the parent scope (outside the `my` module).
//         super::function();

//         // This will bind to the `cool::function` in the *crate* scope.
//         // In this case the crate scope is the outermost scope.
//         {
//             use crate::cool::function as root_function;
//             root_function();
//         }
//     }
// }

// fn main() {
//     my::indirect_call();
// }
