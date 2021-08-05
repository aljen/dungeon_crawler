use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    fn valid_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let destination = location + delta;

        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let index = self.point2d_to_index(destination);
                Some(index)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(index);

        if let Some(index) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((index, 1.0))
        }
        if let Some(index) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((index, 1.0))
        }
        if let Some(index) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((index, 1.0))
        }
        if let Some(index) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((index, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, index1: usize, index2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(self.index_to_point2d(index1), self.index_to_point2d(index2))
    }
}
