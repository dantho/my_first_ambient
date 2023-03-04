use ambient_api::{
    components::core::{
        game_objects::player_camera,
        primitives::{quad,cube},
        transform::{lookat_center, translation}, rendering::color,
        player::player, physics::{box_collider, dynamic, physics_controlled}, prefab::prefab_from_url,
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*, rand, entity::{AnimationController, AnimationAction},
};

#[main]
/// My First Ambient Program
pub async fn main() -> EventResult {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    spawn_query(player()).bind(move |players| {
        for _ in players {
            Entity::new()
                .with_merge(make_transformable())
                .with_default(cube())
                .with(translation(), rand::random())
                .with(color(), rand::random())
                .spawn();
        }
    });

    Entity::new()
        .with_merge(make_transformable())
        .with_default(cube())
        .with(box_collider(), Vec3::ONE * 2.)
        .with(dynamic(), true)
        .with_default(physics_controlled())
        .spawn();

    on(event::COLLISION, |c| {
        println!("Collision");
        EventOk
    });

    println!("Hello, Ambient!");
    println!("Vec3::ZERO is '{:?}'", Vec3::ZERO);
    println!("Vec3::ONE is '{:?}'", Vec3::ONE);
    println!("Vec3::ONE * 5 is '{:?}'", Vec3::ONE * 5.);
    EventOk
}
