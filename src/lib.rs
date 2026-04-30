//! UORustLibs provides methods for reading and writing data files from the classic video game
//! Ultima Online. Most code is tested on data files from the Age of Shadows client, but it should
//! work for earlier clients, and clients up until the switch from `.mul` to `.uop`

extern crate byteorder;
#[cfg(feature = "image")]
extern crate image;

pub mod mul;
pub mod errors;

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
    mod test_mul;
    mod test_skills;
    //    mod test_tiledata;
}
