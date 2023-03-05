use ambient_api::{
    components::core::{
        game_objects::player_camera,
        physics::{box_collider, dynamic, physics_controlled},
        player::player,
        prefab::prefab_from_url,
        primitives::{cube, quad},
        rendering::color,
        transform::{lookat_center, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    entity::{AnimationAction, AnimationController},
    prelude::*,
    rand,
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

    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    spawn_query(player()).bind(move |players| {
        for _ in players {
            Entity::new()
                .with_default(cube())
                .with_merge(make_transformable())
                .with(translation(), rand::random())
                .with(color(), c)
                .spawn();
        }
    });

    for _ in 0..100 {
        for x in c.as_mut() {
            *x = rng.gen();
        }
        c.w = c.w.max(0.5);
        Entity::new()
            .with_default(cube())
            .with_merge(make_transformable())
            .with(box_collider(), Vec3::ONE * 2.)
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
