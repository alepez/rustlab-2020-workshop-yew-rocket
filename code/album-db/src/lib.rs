use serde::Serialize;

const PREVIEWS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../dogs");

#[derive(Debug, Serialize)]
pub struct Image {
    id: usize,
}

#[derive(Debug, Default, Serialize)]
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

pub fn list_images() -> Result<Images, std::io::Error> {
    let entries = std::fs::read_dir(PREVIEWS_DIR)?;

    let images = entries
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
