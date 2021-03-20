use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub height: i32,
    pub width: i32,
    pub layers: Vec<Layer>,
    pub tilesets: Vec<TileSet>,
    #[serde(rename = "tileheight")]
    pub tile_height: i32,
    #[serde(rename = "tilewidth")]
    pub tile_width: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    pub data: Vec<i32>,
    pub height: i32,
    pub width: i32,
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSet {
    pub columns: i32,
    pub image: String,
    #[serde(rename = "imageheight")]
    pub image_height: i32,
    #[serde(rename = "imagewidth")]
    pub image_width: i32,
    pub name: String,
    #[serde(rename = "tilecount")]
    pub tile_count: i32,
    #[serde(rename = "tileheight")]
    pub tile_height: i32,
    #[serde(rename = "tilewidth")]
    pub tile_width: i32,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pub id: i32,
    pub properties: Vec<TileProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileProperty {
    pub name: String,
    #[serde(rename = "type")]
    pub property_type: String,
    pub value: PropertyValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Bool(bool),
    String(String),
}

impl Map {
    pub fn from_json_file(path: &str) -> Result<Map> {
        let data = std::fs::read_to_string(path).expect("Unable to read path file");
        let map: Map = serde_json::from_str(data.as_str())?;
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_json_file() {
        let map = Map::from_json_file("assets/basic_map.json").expect("Failed to load map");
        assert_eq!(50, map.height);
    }
}
