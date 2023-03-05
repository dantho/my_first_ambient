use ambient_api::{
    prelude::*,
    rand,
    concepts::{
        make_perspective_infinite_reverse_camera,
        make_transformable, make_sphere
    },
    components::core::{
        game_objects::player_camera,
        physics::{box_collider, dynamic, physics_controlled},
        player::player,
        primitives::cube,
        rendering::color,
        transform::{lookat_center, translation},
    },
};

#[main]
/// My First Ambient Program
pub async fn main() -> EventResult {
    const BLOCK_COUNT: u16 = 100;
    let c = rcg(); // random color generator

    let _main_ball = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    spawn_query(player()).bind(move |players| {
        for _ in players {
            make_sphere()
                .with_merge(make_transformable())
                .with(translation(), rand::random())
                .with(color(), c)
                .spawn();
        }
    });

    sleep(3.0).await;

    let cube_template = Entity::new()
        .with_default(cube())
        .with_merge(make_transformable())
        .with(box_collider(), Vec3::ONE * 0.5)
        .with(dynamic(), true)
        .with_default(physics_controlled());
    for _ in 0..BLOCK_COUNT {
        let c = rcg();
        cube_template.clone()
            .with(translation(), rand::random())
            .with(color(), c)
            .spawn();
    }
    on(event::COLLISION, |_c| {
        println!("Collision");
        EventOk
    });
    EventOk
}

/// Random Color Generator
fn rcg() -> Vec4 {
        let mut c: Vec4 = rand::random();
        c.w = c.w*0.5+0.5;
        c
}