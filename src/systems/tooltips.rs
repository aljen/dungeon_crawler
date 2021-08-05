use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_postion: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).next().unwrap();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_position = *mouse_postion + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    positions
        .iter(ecs)
        .filter(|(_, position, _)| {
            **position == map_position && player_fov.visible_tiles.contains(&position)
        })
        .for_each(|(entity, _, name)| {
            let screen_position = *mouse_postion * 4;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{}: {}hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_position, &display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
