use bunnymark::bunny_resource::BunnyResource;
use amethyst::{
    core::{
        Transform,
    },
    ecs::Entity,
    prelude::*,
    renderer::{Camera},
};

pub struct BunnyMark {
    pub camera: Option<Entity>,
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for BunnyMark {
    fn on_start(&mut self, data: StateData<GameData>) {
        BunnyResource::initialize(data.world);

        let bounds = data.world.read_resource::<BunnyResource>().bounds;
        let mut transform = Transform::default();
        transform.set_translation_xyz(bounds.x * 0.5, bounds.y * 0.5, 1.0);

        self.camera = Some(
            data.world
                .create_entity()
                .with(Camera::standard_2d(bounds.x, bounds.y))
                .with(transform)
                .build(),
        );
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}
