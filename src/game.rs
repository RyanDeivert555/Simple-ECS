use crate::prelude::*;
use raylib::prelude::*;

struct Counter(usize);
struct Pos(Vector2);
struct Dir(Vector2);
struct Speed(f32);
struct Size(Vector2);
struct Render(Color);

register_components!(Pos, Dir, Speed, Size, Render);
register_resources!(RaylibHandle, RaylibThread, Counter);

pub struct Setup;
impl StartUpSystem for Setup {
    fn run(&mut self, world: &mut World) -> bool {
        set_trace_log(TraceLogLevel::LOG_NONE);
        let (rl, thread) = raylib::init()
            .size(800, 600)
            .title("Benchmark Test")
            .build();

        world.add_resource(rl);
        world.add_resource(thread);
        world.add_resource(Counter(0));

        true
    }
}

pub struct CloseSystem;
impl System for CloseSystem {
    fn run(&mut self, world: &mut World) -> bool {
        let rl = world.get_resource::<RaylibHandle>().unwrap();
        if rl.window_should_close() {
            false
        } else {
            true
        }
    }
}

pub struct RenderSystem;
impl System for RenderSystem {
    fn run(&mut self, world: &mut World) -> bool {
        let thread = world.get_resource::<RaylibThread>().unwrap();
        let mut rl = world.get_resource_mut::<RaylibHandle>().unwrap();
        let counter = world.get_resource::<Counter>().unwrap();
        let fps = rl.get_fps();
        let mut draw_handle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);
        let query = world.query_components::<(Render, Pos, Size)>();
        for (renderable, position, size) in query {
            draw_handle.draw_rectangle_v(position.0, size.0, renderable.0);
        }
        draw_handle.draw_text(&fps.to_string(), 10, 0, 50, Color::LIME);
        draw_handle.draw_text(&counter.0.to_string(), 10, 50, 50, Color::LIME);

        true
    }
}

pub struct AddSquaresSystem;
impl System for AddSquaresSystem {
    fn run(&mut self, world: &mut World) -> bool {
        let (left_mouse_down, _right_mouse_down) = {
            let rl = world.get_resource::<RaylibHandle>().unwrap();
            (
                rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
                rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON),
            )
        };
        let square_count = 100usize;

        if left_mouse_down {
            for _ in 0..square_count {
                add_square(world);
            }
            let mut counter = world.get_resource_mut::<Counter>().unwrap();
            counter.0 += square_count;
        }

        // if right_mouse_down {
        //     let entities = world.query_components::<(Entity,)>();
        //     for (entity,) in entities {
        //         world.remove_entity(entity.id());
        //     }
        // }

        true
    }
}

fn add_square(world: &mut World) {
    let square = world.new_entity();
    let color = Color::new(
        get_random_value::<i32>(0, 255) as u8,
        get_random_value::<i32>(0, 255) as u8,
        get_random_value::<i32>(0, 255) as u8,
        255,
    );
    let dir = Vector2::new(
        if get_random_value::<i32>(0, 1) == 0 {
            -1.0
        } else {
            1.0
        },
        if get_random_value::<i32>(0, 1) == 0 {
            -1.0
        } else {
            1.0
        },
    )
    .normalized();
    let speed = get_random_value::<i32>(100, 800) as f32;
    let size = Vector2::new(
        get_random_value::<i32>(10, 100) as f32,
        get_random_value::<i32>(10, 100) as f32,
    );
    let pos = Vector2::new(
        get_random_value::<i32>(0, 800) as f32 - size.x,
        get_random_value::<i32>(0, 600) as f32 - size.y,
    );
    world.add_component(square, Render(color));
    world.add_component(square, Pos(pos));
    world.add_component(square, Dir(dir));
    world.add_component(square, Speed(speed));
    world.add_component(square, Size(size));
}
