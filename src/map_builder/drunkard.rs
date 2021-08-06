use super::MapArchitect;
use crate::prelude::*;

pub struct DrunkardsWalkArchitect {}

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Wall);

        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        self.drunkard(&center, rng, &mut mb.map);

        while mb
            .map
            .tiles
            .iter()
            .filter(|tile| **tile == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            self.drunkard(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );

            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );

            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(index, _)| mb.map.tiles[index] = TileType::Wall);
        }

        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_position = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunkard_index = map.point2d_to_index(drunkard_position);

            map.tiles[drunkard_index] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_position.x -= 1,
                1 => drunkard_position.x += 1,
                2 => drunkard_position.y -= 1,
                _ => drunkard_position.y += 1,
            }

            if !map.in_bounds(drunkard_position) {
                break;
            }

            distance_staggered += 1;

            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
