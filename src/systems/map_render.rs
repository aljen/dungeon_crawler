use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();

    draw_batch.target(0);

    let mut player = <(&Point, &Player)>::query();

    let player_position = player.iter(ecs).nth(0).unwrap().0;

    let player_fov = fov.iter(ecs).next().unwrap();

    let white = RGB::named(WHITE);
    let gray = RGB::named(GRAY47);
    let dark_gray = RGB::named(DARKSLATEGRAY);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let index = map_idx(x, y);

            let in_fov = player_fov.visible_tiles.contains(&point);

            if map.in_bounds(point) && (in_fov | map.revealed_tiles[index]) {
                let distance = map.get_pathing_distance(
                    map_idx(point.x, point.y),
                    map_idx(player_position.x, player_position.y),
                );
                let distance = distance / player_fov.radius as f32;
                let distance = distance;

                let tint = if in_fov {
                    white.lerp(gray, distance)
                } else {
                    dark_gray
                };

                let index = map_idx(x, y);
                let glyph = match map.tiles[index] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                draw_batch.set(point - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
