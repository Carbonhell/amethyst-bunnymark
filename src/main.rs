extern crate amethyst;
extern crate rand;

mod bunnymark;

use amethyst::{
    core::TransformBundle,
    prelude::*,
    utils::fps_counter::FpsCounterBundle, renderer::{types::DefaultBackend, RenderingBundle, RenderToWindow, RenderFlat2D},
};

use bunnymark::{BunnyMark, BunnyMarkBundle};

fn main() -> Result<(), amethyst::Error> {
    // logger disabled because we don't want it to affect the benchmark
    //amethyst::start_logger(Default::default());

    let display_config_path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let assets_dir = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));

    let game_data = GameDataBuilder::default()
        .with_bundle(BunnyMarkBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.1, 0.1, 0.1, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
        )?;

    let mut game = Application::build(assets_dir, BunnyMark { camera: None })?.build(game_data)?;

    game.run();
    Ok(())
}
