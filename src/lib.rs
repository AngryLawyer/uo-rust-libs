extern crate byteorder;
#[cfg(feature = "use-sdl2")]
extern crate sdl2;

pub mod mul_reader;
pub mod utils;

pub mod color;

pub mod art;
pub mod skills;
pub mod hues;
//pub mod map;
//pub mod tiledata;

#[cfg(test)]
mod tests {
    mod test_mul_reader;
    mod test_color;
    mod test_skills;
    mod test_hues;
    mod test_art;
//    mod test_tiledata;
}
