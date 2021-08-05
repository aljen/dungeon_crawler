use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();

    draw_batch.target(1);

    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).next().unwrap();

    let offset = Point::new(camera.left_x, camera.top_y);

    renderables
        .iter(ecs)
        .filter(|(position, _)| player_fov.visible_tiles.contains(&position))
        .for_each(|(position, render)| {
            draw_batch.set(*position - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
