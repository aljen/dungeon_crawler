use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_position = player.iter(ecs).nth(0).unwrap().0;
    let player_index = map_idx(player_position.x, player_position.y);

    let search_targets = vec![player_index];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(entity, position, _)| {
        let index = map_idx(position.x, position.y);

        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, index, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*position, *player_position);

            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_position
            };

            let mut attacked = false;

            positions
                .iter(ecs)
                .filter(|(_, target_position, _)| **target_position == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }

                    attacked = true;
                });

            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    })
}
