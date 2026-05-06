//! UORustLibs provides methods for reading and writing data files from the classic video game
//! Ultima Online. Most code is tested on data files from the Age of Shadows client, but it should
//! work for earlier clients, and clients up until the switch from `.mul` to `.uop`

extern crate byteorder;
#[cfg(feature = "image")]
extern crate image;

pub mod error;
pub mod mul;

pub mod color;

pub mod anim;
pub mod art;
pub mod font;
pub mod gump;
pub mod hue;
pub mod map;
pub mod skill;
pub mod texmap;
pub mod tiledata;

#[cfg(test)]
mod tests {
    mod test_anim;
    mod test_art;
    mod test_color;
    mod test_font;
    mod test_gump;
    mod test_hue;
    mod test_mul;
    mod test_skill;
    mod test_texmap;
    mod test_tiledata;
}
