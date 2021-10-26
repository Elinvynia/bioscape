use std::fmt;

pub fn get_assets_folder() -> String {
    "assets/".into()
}

// A sprite can be made from multiple textures (images in our case).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sprite {
    pub textures: Vec<TextureFile>,
}

impl Sprite {
    pub fn new(textures: Vec<TextureFile>) -> Self {
        Sprite { textures }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFile {
    Unimplemented,
}

// Displaying it into a string gives us the path to it.
impl fmt::Display for TextureFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let folder = get_assets_folder();

        use TextureFile::*;
        let file = match self {
            Unimplemented => "unimplemented.png",
        };

        write!(f, "{}{}", folder, file)
    }
}
