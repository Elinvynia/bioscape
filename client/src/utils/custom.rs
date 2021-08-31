use bioscape_common::component::TextureFile;
use std::convert::TryFrom;

// Safety wrapper for egui User textures.
// Egui stores custom textures as u64.
#[derive(Debug)]
pub enum CustomTexture {
    Unimplemented,
}

impl TryFrom<u64> for CustomTexture {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        use CustomTexture::*;
        match value {
            0 => Ok(Unimplemented),
            _ => Err(()),
        }
    }
}

impl From<CustomTexture> for u64 {
    fn from(ct: CustomTexture) -> Self {
        use CustomTexture::*;
        match ct {
            Unimplemented => 0,
        }
    }
}

impl From<CustomTexture> for TextureFile {
    fn from(ct: CustomTexture) -> Self {
        use CustomTexture::*;
        match ct {
            Unimplemented => TextureFile::Unimplemented,
        }
    }
}
