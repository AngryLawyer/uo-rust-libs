#![crate_id="uorustlibs#0.0.1"]
#![crate_type = "lib"]

#![desc = "UO data file libraries"]
#![license = "MIT"]

#[cfg(test)]
extern crate debug;


//pub mod byte_helpers;
pub mod mul_reader;
//pub mod utils;

//pub mod art;
pub mod skills;
pub mod hues;
//pub mod map;
//pub mod tiledata;

#[cfg(test)]
mod tests {
    mod mul_reader;
    mod skills;
    mod hues;
}
