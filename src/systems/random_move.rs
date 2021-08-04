use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &MovingRandomly)>::query();

    let mut rng = RandomNumberGenerator::new();

    movers.iter_mut(ecs).for_each(|(position, _)| {
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *position;

        if map.can_enter_tile(destination) {
            *position = destination;
        }
    });
}
