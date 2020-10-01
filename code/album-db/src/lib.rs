use serde::{Serialize, Deserialize};

const PREVIEWS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../dogs");

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Image {
    pub id: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Images(pub Vec<Image>);

impl Image {
    pub fn from_id(id: usize) -> Self {
        Self { id }
    }

    pub fn preview_path(&self) -> std::path::PathBuf {
        let filename = format!("dog.{}.jpg", self.id);
        let mut path = std::path::PathBuf::from(PREVIEWS_DIR);
        path.push(filename);
        path
    }
}

#[cfg(feature="rocket_param")]
impl<'r> rocket::request::FromParam<'r> for Image {
    type Error = &'r rocket::http::RawStr;

    fn from_param(param: &'r rocket::http::RawStr) -> Result<Self, Self::Error> {
        let id = usize::from_param(param)?;
        Ok(Image::from_id(id))
    }
}

pub fn list_images() -> Result<Images, std::io::Error> {
    let entries = std::fs::read_dir(PREVIEWS_DIR)?;

    let images = entries
        .take(100)
        .filter_map(|res| res.ok())
        .filter_map(|res| {
            let name = res.file_name();
            parse_id(name.to_str()?)
        })
        .map(|id| Image::from_id(id))
        .collect();

    Ok(Images(images))
}

fn parse_id(filename: &str) -> Option<usize> {
    let name: String = filename.chars().skip(4).take_while(|&x| x != '.').collect();
    name.parse().ok()
}
