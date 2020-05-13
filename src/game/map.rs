use ggez::{self, *};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Map {
    pub layers: Vec<MapLayer>,
    tiles: Vec<Tile>,
    image_cache: HashMap<String, Arc<graphics::Image>>,
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
    pub image: Arc<graphics::Image>,
    pub rect: graphics::Rect,
}

impl Map {
    pub fn new(ctx: &mut ggez::Context, map: &tiled::Map) -> Self {
        let mut layers = Vec::new();
        let mut tiles = Vec::new();
        let mut image_cache = HashMap::new();

        for ts in &map.tilesets {
            for image_data in &ts.images {
                let mut path = "/".to_string();
                path.push_str(&image_data.source);
                image_cache.entry(image_data.source.to_string()).or_insert(Arc::new(graphics::Image::new(ctx, &path).unwrap()));
                let columns = image_data.width as u32/ts.tile_width;
                for curr_tile in 0..ts.tilecount.unwrap() {
                    let rect = graphics::Rect::new(
                        (ts.margin + (curr_tile % columns) * (ts.tile_width + ts.spacing)) as f32/image_data.width as f32,
                        (ts.margin + (curr_tile / columns) * (ts.tile_height + ts.spacing)) as f32/image_data.height as f32,
                        ts.tile_width as f32/image_data.width as f32,
                        ts.tile_height as f32/image_data.height as f32,
                        );
                    let image = Arc::clone(image_cache.get(&image_data.source).unwrap());
                    tiles.push(Tile { rect, image });
                }
            }
        }

        for layer in &map.layers {
            let mut map_tiles = Vec::new();
            for (row_num, tile_row) in layer.tiles.iter().enumerate() {
                for (col_num, tile) in tile_row.iter().enumerate() {
                    let w = 32.0;
                    let h = 32.0;
                    let x = col_num as f32 * w;
                    let y = row_num as f32 * h;

                    map_tiles.push(MapTile{ tile_num: tile.gid as usize, loc: [x, y] });
                }
            }
            layers.push(MapLayer{ map_tiles });
        }

        Map {
            layers,
            tiles,
            image_cache
        }
    }

    pub fn get_tile(&self, gid: usize) -> Option<&Tile> {
        if gid == 0 {
            return None
        }
        Some(&self.tiles[gid - 1])
    }
}
