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
        primitives::{cube, sphere},
        rendering::color,
        transform::{lookat_center, translation},
    },
};

#[main]
/// My First Ambient Program
pub async fn main() -> EventResult {
    let mut c = Vec4::new(0.0, 0.0, 0.0, 0.0);
    let mut rng = rand::thread_rng();

    for x in c.as_mut() {
        *x = rng.gen();
    }
    c.w = c.w.max(0.5);

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
            // Entity::new()
            //     .with_default(cube())
            //     .with_merge(make_transformable())
            //     .with(translation(), rand::random())
            //     .with(color(), c)
            //     .spawn();
        }
    });

    sleep(3.0).await;

    for _ in 0..100 {
        for x in c.as_mut() {
            *x = rng.gen();
        }
        c.w = c.w.max(0.5);
        Entity::new()
            .with_default(cube())
            .with_merge(make_transformable())
            .with(box_collider(), Vec3::ONE * 0.5)
            .with(dynamic(), true)
            .with_default(physics_controlled())
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
