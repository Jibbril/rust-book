mod front_of_house;

// When bringing functions into scope it is recommended to only import the
// parent module and then specify in code.
use crate::front_of_house::hosting;

// We can also re-export items so that modules calling this module can 
// utilize the exported item as if it was imported there
// pub use crate::front_of_house::hosting;

// For using structs and enums the best practice is to import them fully
use std::collections::HashMap;

pub fn eat_at_resturant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    // After use statement
    hosting::add_to_waitlist();
}

// If items with identical names are imported they need to be specified, either 
// like this
use std::fmt;
use std::io;

fn function1(format: fmt::Result) {}
fn function2(format: io::Result<u8>) {}

// Or like this
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn function3(format: FmtResult) {}
fn function4(format: IoResult<u8>) {}


// To reduce the lines needed for use statements we can combine inputs from
// similar paths like below

// use std::io;
// use std::io::Write;

// equivalent:
// use std::io::{self, Write};