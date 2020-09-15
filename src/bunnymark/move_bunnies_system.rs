use amethyst::ecs::ReadExpect;
use bunnymark::BunnyResource;
use amethyst::{
	core::{Time, Transform},
	ecs::prelude::{Join, Read, System, WriteStorage},
};
use bunnymark::Bunny;
use rand::random;

const GRAVITY: f32 = 500.0;

pub struct MoveBunniesSystem {}

impl MoveBunniesSystem {
	pub fn new() -> MoveBunniesSystem {
		MoveBunniesSystem {}
	}
}

impl<'s> System<'s> for MoveBunniesSystem {
	type SystemData = (
		WriteStorage<'s, Bunny>,
		WriteStorage<'s, Transform>,
		Read<'s, Time>,
		ReadExpect<'s, BunnyResource>,
	);

	fn run(&mut self, (mut bunnies, mut transforms, time, bunny_resource): Self::SystemData) {

		for (bunny, transform) in (&mut bunnies, &mut transforms).join() {
			let translation = transform.translation();
			let mut new_translation = transform.translation().clone();
			bunny.velocity.y += GRAVITY * time.delta_seconds();

			if translation.x > bunny_resource.bounds.x {
				bunny.velocity.x *= -1.0;
			}

			if translation.x < 0.0 {
				bunny.velocity.x *= -1.0;
			}

			if translation.y > bunny_resource.bounds.y {
				new_translation.y = bunny_resource.bounds.y;
				if random::<f32>() > 0.5 {
					bunny.velocity.y = (random::<u32>() % 1100 + 50) as f32;
				} else {
					bunny.velocity.y *= -0.85;
				}
			}

			if translation.y < 0.0 {
				new_translation[1] = 0.0;
				bunny.velocity[1] = 0.0;
			}

			transform.set_translation(new_translation + bunny.velocity * time.delta_seconds());
		}
	}
}
