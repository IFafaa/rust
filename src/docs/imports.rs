use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
use std::io::{self, Write};
use std::time::Duration;

// Importing specific items from a module

// Renaming an imported item

// Importing multiple items from a module

// Importing all items from a module (generally not recommended)
use std::collections::*;

// Importing with public visibility (for re-exporting)
pub use std::fs::File;

// Importing nested modules

// Importing with alias to avoid conflicts

// Importing only in a specific scope
fn example() {
    let duration = Duration::new(5, 0);
    println!("{:?}", duration);
}
