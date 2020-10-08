use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Image {
    pub id: usize,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Images(pub Vec<Image>);

impl Image {
    pub fn from_id(id: usize) -> Self {
        Self { id }
    }

    pub fn preview_path(&self, db: &Database) -> PathBuf {
        let filename = format!("dog.{}.jpg", self.id);
        let mut path = PathBuf::from(&db.root);
        path.push(filename);
        path
    }
}

#[cfg(feature = "rocket_param")]
impl<'r> rocket::request::FromParam<'r> for Image {
    type Error = &'r rocket::http::RawStr;

    fn from_param(param: &'r rocket::http::RawStr) -> Result<Self, Self::Error> {
        let id = usize::from_param(param)?;
        Ok(Image::from_id(id))
    }
}

fn parse_id(filename: &str) -> Option<usize> {
    let name: String = filename.chars().skip(4).take_while(|&x| x != '.').collect();
    name.parse().ok()
}

pub struct Database {
    root: PathBuf,
    images: Images,
}

impl Database {
    pub fn new(root: PathBuf) -> Option<Self> {
        let entries = std::fs::read_dir(&root).ok()?;

        let images = entries
            .take(100)
            .filter_map(|res| res.ok())
            .filter_map(|res| {
                let name = res.file_name();
                parse_id(name.to_str()?)
            })
            .map(|id| Image::from_id(id))
            .collect();

        let images = Images(images);

        log::info!("Loaded {} images", images.0.len());

        Some(Self { root, images })
    }

    pub fn list_images(&self) -> &Images {
        &self.images
    }

    pub fn delete_image(&mut self, image: &Image) {
        &self.images.0.retain(|x| x.id != image.id);

    }
}
