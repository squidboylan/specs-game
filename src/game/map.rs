use std::collections::HashMap;
use std::sync::Arc;
use crate::components::Rect;

/// A map contains the background info to render the map, worth noting that we
/// currently only support maps made of one tileset, this simplifies rendering,
/// I'll probably change this in the future
#[derive(Clone)]
pub struct Map {
    pub layers: Vec<MapLayer>,
    pub image: String,
    tiles: Vec<Tile>,
}

#[derive(Clone)]
pub struct MapLayer {
    pub map_tiles: Vec<MapTile>,
}

#[derive(Clone)]
pub struct MapTile {
    pub tile_num: usize,
    pub loc: [f32; 2]
}

#[derive(Clone)]
pub struct Tile {
    pub rect: Rect,
}

impl Map {
    pub fn from_tiled(map: &tiled::Map) -> Result<Self, String> {
        let mut layers = Vec::new();
        let mut tiles = Vec::new();

        let mut path: Option<String> = None;

        for ts in &map.tilesets {
            // I'm not actually sure if there can be multiple images in a tileset but the format
            // suggests it so we'll do this just to be safe.
            for image_data in &ts.images {
                if path.is_none() {
                    let mut tmp = "resources/".to_string();
                    tmp.push_str(&image_data.source);
                    path = Some(tmp);
                } else {
                    let mut tmp = "resources/".to_string();
                    tmp.push_str(&image_data.source);
                    // ugly but whatever
                    if path != Some(tmp) {
                        return Err("Maps with multiple tilesets are not supported".to_string());
                    }
                }
                let columns = image_data.width as u32/ts.tile_width;
                for curr_tile in 0..ts.tilecount.unwrap() {
                    let w = ts.tile_width as f32/image_data.width as f32;
                    let h = ts.tile_height as f32/image_data.height as f32;
                    let rect = Rect::new(
                        ((ts.margin + (curr_tile % columns) * (ts.tile_width + ts.spacing)) as f32)/image_data.width as f32 + w/2.0,
                        ((ts.margin + (curr_tile / columns) * (ts.tile_height + ts.spacing)) as f32)/image_data.height as f32 + h/2.0,
                        w,
                        h,
                        );
                    tiles.push(Tile { rect });
                }
            }
        }

        for layer in &map.layers {
            let mut map_tiles = Vec::new();
            for (row_num, tile_row) in layer.tiles.iter().enumerate() {
                for (col_num, tile) in tile_row.iter().enumerate() {
                    let w = 32.0;
                    let h = 32.0;
                    let x = col_num as f32 * w + w/2.0;
                    let y = row_num as f32 * h + h/2.0;

                    map_tiles.push(MapTile{ tile_num: tile.gid as usize, loc: [x, y] });
                }
            }
            layers.push(MapLayer{ map_tiles });
        }

        Ok(
            Map {
                layers,
                tiles,
                image: path.unwrap(),
            }
        )
    }

    pub fn get_tile(&self, gid: usize) -> Option<&Tile> {
        if gid == 0 {
            return None
        }
        Some(&self.tiles[gid - 1])
    }
}
