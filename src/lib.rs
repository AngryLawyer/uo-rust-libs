extern crate byteorder;
extern crate image;

pub mod mul_reader;
pub mod utils;

pub mod color;

pub mod art;
pub mod skills;
pub mod hues;
pub mod map;
pub mod tiledata;
pub mod gump;
pub mod anim;

#[cfg(test)]
mod tests {
    mod test_mul_reader;
    mod test_color;
    mod test_skills;
    mod test_hues;
    mod test_art;
    mod test_gump;
//    mod test_tiledata;
}
