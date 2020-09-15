use amethyst::{
    core::bundle::SystemBundle,
    ecs::DispatcherBuilder, shred::World,
};
use bunnymark::{MoveBunniesSystem, SpawnBunniesSystem};

pub struct BunnyMarkBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for BunnyMarkBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> amethyst::Result<()> {
        builder.add(MoveBunniesSystem::new(), "move_bunnies", &[]);
        builder.add(SpawnBunniesSystem::new(), "spawn_bunnies", &[]);
        Ok(())
    }
}
