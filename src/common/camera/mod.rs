use amethyst::{
    ecs::Entity,
    prelude::{StateEvent, World, WorldExt},
    renderer::{camera::Projection, Camera},
    winit::{dpi::LogicalSize, Event, WindowEvent},
};

fn projection(width: f32, height: f32) -> Projection {
    // ripped from Camera::standard_2d
    Projection::orthographic(
        -width / 2.0,
        width / 2.0,
        -height / 2.0,
        height / 2.0,
        0.1,
        2000.0,
    )
}

pub fn camera_resize(world: &mut World, size: &LogicalSize) {
    let active_camera = *world.read_resource::<ActiveCamera>();
    let mut cameras = world.write_storage::<Camera>();
    let camera = cameras
        .get_mut(active_camera.id)
        .expect("error getting active camera");
    camera.set_projection(projection(size.width as f32, size.height as f32))
}

pub fn handle_camera_resize(world: &mut World, event: &StateEvent) {
    if let StateEvent::Window(event) = event {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::Resized(logical_size) = event {
                camera_resize(world, logical_size)
            }
        }
    }
}

/// Inserted into the world with camera initialization. Used to grab the correct camera to handle resizing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveCamera {
    pub id: Entity,
}

impl ActiveCamera {
    #[cold]
    pub const fn new(id: Entity) -> Self {
        ActiveCamera { id }
    }

    #[cold]
    pub fn add_to_world(self, world: &mut World) {
        world.insert(self)
    }
}
