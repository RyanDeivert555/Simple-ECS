use crate::prelude::*;
use raylib::prelude::*;

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
        //let counter = world.get_resource::<SquareCounter>().unwrap();
        let fps = rl.get_fps();
        let mut draw_handle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);
        let query = world.query_components::<(Renderable, Position, Size)>();
        for (renderable, position, size) in query {
            draw_handle.draw_rectangle_v(position.0, size.0, renderable.0);
        }
        draw_handle.draw_text(&fps.to_string(), 10, 0, 50, Color::LIME);
        //draw_handle.draw_text(&counter.0.to_string(), 10, 50, 50, Color::LIME);

        true
    }
}
