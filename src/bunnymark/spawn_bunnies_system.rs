use amethyst::{
    core::{math::Vector3, Transform, Time},
    ecs::{
        prelude::{Read, System},
        Entities, ReadExpect, WriteStorage,
    },
    renderer::{Sprite, SpriteRender, sprite::TextureCoordinates},
    utils::fps_counter::FpsCounter,
};
use bunnymark::{BunnyResource, Bunny};
use rand::random;
use std::fs::File;
use std::io::Write;
use std::process::exit;

const SPAWN_WAIT_TIME_IN_SECONDS: f32 = 0.5;
const BUNNIES_TO_SPAWN: i32 = 100;

pub struct SpawnBunniesSystem {
    pub elapsed: f32,
    pub print_elapsed: f32,
    pub count: usize,
    pub sprite: Sprite,
    file: File,
}

impl SpawnBunniesSystem {
    pub fn new() -> SpawnBunniesSystem {
        SpawnBunniesSystem {
            elapsed: 0.0,
            print_elapsed: 0.0,
            count: 0,
            sprite: Sprite {
                height: 604.0,
                width: 604.0,
                offsets: [0.0, 0.0],
                tex_coords: TextureCoordinates {
                    left: 0.0,
                    right: 1.0,
                    top: 1.0,
                    bottom: 0.0,
                },
            },
            file: File::create("output.txt").unwrap(),
        }
    }

    fn spawn_bunny<'s>(
        &mut self,
        entities: &Entities<'s>,
        bunnies: &mut WriteStorage<'s, Bunny>,
        transforms: &mut WriteStorage<'s, Transform>,
        sprite_renders: &mut WriteStorage<'s, SpriteRender>,
        bunny_resource: &ReadExpect<'s, BunnyResource>,
    ) {
        let mut transform = Transform::default();
        transform.set_translation(Vector3::new(
            bunny_resource.bounds.x / 2.0 - bunny_resource.sprite_size.x / 2.0,
            bunny_resource.bounds.y / 2.0 - bunny_resource.sprite_size.y / 2.0,
            0.0,
        ));

        let sprite_render = bunny_resource.sprite_render.clone();

        entities
            .build_entity()
            .with(sprite_render, sprite_renders)
            .with(
                Bunny {
                    velocity: Vector3::new(
                        random::<f32>() * 200.0 + 50.0,
                        random::<f32>() * 200.0 + 50.0,
                        0.0,
                    ),
                },
                bunnies,
            )
            .with(transform, transforms)
            .build();

        self.count += 1;
    }
}

impl<'s> System<'s> for SpawnBunniesSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Bunny>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, BunnyResource>,
        Read<'s, Time>,
        Read<'s, FpsCounter>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut bunnies,
            mut transforms,
            mut sprite_renders,
            bunny_resources,
            time,
            fps_counter,
        ) = data;

        self.elapsed += time.delta_seconds();
        self.print_elapsed += time.delta_seconds();
        
        if self.elapsed > SPAWN_WAIT_TIME_IN_SECONDS {
            for _ in 0..BUNNIES_TO_SPAWN {
                self.spawn_bunny(
                    &entities,
                    &mut bunnies,
                    &mut transforms,
                    &mut sprite_renders,
                    &bunny_resources,
                );
            }
            self.elapsed = 0.0;
        }

        if self.print_elapsed > 2.0 {
            let fps = fps_counter.sampled_fps();
            self.file.write_all(format!("{},{}\n", self.count, fps).as_bytes()).unwrap();
            if fps < 20.0 {exit(0);}
            self.print_elapsed = 0.0;
        }
    }
}
