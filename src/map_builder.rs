use rand::Rng;
use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,

}

impl MapBuilder {
    pub fn new() -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms();
        mb.build_corridors();
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile_type: TileType) {
        self.map.tiles.iter_mut().for_each(|tile| *tile = tile_type);

        self.rooms.clear();
        self.player_start = Point::zero();
    }

    fn build_random_rooms(&mut self) {
        let mut rng = rand::thread_rng();
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.gen_range(1..SCREEN_WIDTH - 10),
                rng.gen_range(1..SCREEN_HEIGHT - 10),
                rng.gen_range(2..10),
                rng.gen_range(2..10),
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if room.intersect(r) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|point| {
                    if point.x > 0 && point.x < SCREEN_WIDTH && point.y > 0 && point.y < SCREEN_HEIGHT {
                        let idx = map_idx(point.x, point.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self) {
        let mut rng = rand::thread_rng();
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.gen_range(0..2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}