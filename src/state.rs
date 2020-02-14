use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use rand::{thread_rng, Rng};

use crate::common::arena::ArenaBounds;
use crate::particles::ParticlesConfig;

pub struct GameState;

impl SimpleState for GameState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut rng = thread_rng();

        // Define the bounds of the arena
        init_arena(world);

        // Place the camera
        init_camera(world);

        // Load our sprites and display them
        let sprites = load_sprite_visuals(world);
        init_particles(world, &mut rng, sprites);
        // more game object initializations go here
    }

    //    fn handle_event(
    //        &mut self,
    //        mut _data: StateData<'_, GameData<'_, '_>>,
    //        event: StateEvent,
    //    ) -> SimpleTrans {
    //        if let StateEvent::Window(event) = &event {
    //            // Check if the window should be closed
    //            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
    //                return Trans::Quit;
    //            }
    //
    //            // Listen to any key events
    //            if let Some(event) = get_key(&event) {
    //                info!("handling key event: {:?}", event);
    //            }
    //
    //            // If you're looking for a more sophisticated event handling solution,
    //            // including key bindings and gamepad support, please have a look at
    //            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
    //        }
    //
    //        // Keep going
    //        Trans::None
    //    }
}

fn init_arena(world: &mut World) {
    const DEFAULT_WIDTH: f32 = 1000.0;
    const DEFAULT_HEIGHT: f32 = 1000.0;
    let arena = ArenaBounds::new(DEFAULT_WIDTH, DEFAULT_HEIGHT).unwrap();
    world.insert(arena)
}

fn init_camera(world: &mut World) {
    let (viewport_width, viewport_height) = {
        let screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let h = 200.0 * screen_dimensions.hidpi_factor() as f32;
        let w = h * screen_dimensions.aspect_ratio();
        (w, h)
    };

    let transform = {
        let arena_bounds = *world.read_resource::<ArenaBounds>();
        let midpoint = arena_bounds / 2.0;
        let mut transform = Transform::default();
        transform
            .set_translation_x(midpoint.width())
            .set_translation_y(midpoint.height());
        transform
    };

    world
        .create_entity()
        .with(Camera::standard_2d(viewport_width, viewport_height))
        .with(transform)
        .build();
}

fn load_sprite_visuals(world: &mut World) -> Handle<SpriteSheet> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprite_sheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let loader = world.read_resource::<Loader>();
    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprite_sheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sheet_storage,
    )
}

fn init_particles(world: &mut World, rng: &mut impl Rng, sheet_handle: Handle<SpriteSheet>) {
    let particles = ParticlesConfig {
        rng,
        bounds: *world.read_resource::<ArenaBounds>(),
        velocity_middle: 10.0,
        velocity_maximum_percent_variation: 0.5,
        sprite_render: SpriteRender {
            sprite_sheet: sheet_handle,
            sprite_number: 0,
        },
        count: 10,
    };

    particles.add_to_world(world);
}
