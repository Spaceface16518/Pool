use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::{
    common::physics::{
        current::{drag::DragSystem, CurrentSystem},
        velocity::MovementSystem,
    },
    particles::{collider::ParticleCollisionSystem, tint_shift::TintShiftSystem},
};

mod common;
mod particles;
mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(MovementSystem, "movement_system", &[])
        .with(
            ParticleCollisionSystem,
            "particle_collision_system",
            &["movement_system"],
        )
        .with(CurrentSystem, "current_system", &["movement_system"])
        .with(DragSystem, "drag_system", &["movement_system"])
        .with(TintShiftSystem, "tint_shift_system", &[]);

    let mut game = Application::new(resources, state::GameState, game_data)?;
    game.run();

    Ok(())
}
