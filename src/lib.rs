extern crate byteorder;
#[cfg(feature = "image")]
extern crate image;

pub mod mul_reader;
pub mod utils;

pub mod color;

pub mod anim;
pub mod art;
pub mod fonts;
pub mod gump;
pub mod hues;
pub mod map;
pub mod skills;
pub mod texmaps;
pub mod tiledata;

#[cfg(test)]
mod tests {
    mod test_art;
    mod test_color;
    mod test_gump;
    mod test_hues;
    mod test_mul_reader;
    mod test_skills;
    //    mod test_tiledata;
}
