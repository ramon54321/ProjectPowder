use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use femtovg::{renderer::OpenGl, Canvas, Color, FontId};
use glutin::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use nalgebra_glm::Vec2;

pub use glutin::event::VirtualKeyCode;

pub type RenderLayerFn<T> = Box<dyn FnMut(&mut Canvas<OpenGl>, &mut Meta, &mut T) -> ()>;

pub struct Powder<T> {
    pub state: T,
    event_loop: EventLoop<()>,
    context: ContextWrapper<PossiblyCurrent, Window>,
    canvas: Canvas<OpenGl>,
    render_layers: Vec<RenderLayerFn<T>>,
}
impl<T> Powder<T>
where
    T: 'static,
{
    pub fn new(state: T, width: u16, height: u16, title: &str) -> Result<Self, String> {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_min_inner_size(LogicalSize::new(width, height))
            .with_position(PhysicalPosition::new(0, 0))
            .with_title(title);

        // Build window context
        let context = ContextBuilder::new()
            .with_multisampling(8)
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop);
        if context.is_err() {
            return Err("Could not build window".to_string());
        }
        let window_context = context.unwrap();

        // Make context current
        let context = unsafe {
            let current_option = window_context.make_current();
            if current_option.is_err() {
                return Err("Could not make context current".to_string());
            }
            current_option.unwrap()
        };

        // Create renderer
        let renderer = OpenGl::new_from_glutin_context(&context);
        if renderer.is_err() {
            return Err("Could not create renderer".to_string());
        }
        let renderer = renderer.unwrap();

        let canvas = Canvas::new(renderer);
        if canvas.is_err() {
            return Err("Could not create canvas".to_string());
        }
        let canvas = canvas.unwrap();

        Ok(Self {
            state,
            event_loop,
            context,
            canvas,
            render_layers: Vec::new(),
        })
    }
    pub fn start(mut self) {
        // Framerate management
        let mut frame_timer = Instant::now();
        let mut frame_second_timer = Instant::now();
        let mut frame_counter = 0;
        let mut frames_per_second = 0;

        let mut mouse_position = Vec2::new(0.0, 0.0);
        let mut is_mouse_down = false;
        let mut is_mouse_released = false;
        let mut keys_hold = HashSet::new();
        let mut keys_down = HashSet::new();
        let mut keys_up = HashSet::new();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            let window = self.context.window();

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _,
                    } => {
                        if input.virtual_keycode.is_some() {
                            let key = input.virtual_keycode.unwrap();
                            let value = input.state == ElementState::Pressed;
                            if value {
                                keys_down.insert(key);
                                keys_hold.insert(key);
                            } else {
                                keys_up.insert(key);
                                keys_hold.remove(&key);
                            }
                        }
                    }
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        ..
                    } => {
                        mouse_position.x = position.x as f32;
                        mouse_position.y = position.y as f32;
                    }
                    WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button,
                        ..
                    } => match state {
                        ElementState::Pressed => match button {
                            MouseButton::Left => {
                                is_mouse_down = true;
                            }
                            _ => (),
                        },
                        ElementState::Released => match button {
                            MouseButton::Left => {
                                is_mouse_down = false;
                                is_mouse_released = true;
                            }
                            _ => (),
                        },
                    },
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    // Before Rendering
                    frame_counter = frame_counter + 1;
                    let time_since_frame_counter_reset = Instant::now() - frame_second_timer;
                    if time_since_frame_counter_reset.as_millis() >= 1000 {
                        frame_second_timer = Instant::now();
                        frames_per_second = frame_counter;
                        frame_counter = 0;
                    }
                    let delta_time = Instant::now() - frame_timer;
                    frame_timer = Instant::now();

                    let size = window.inner_size();
                    self.canvas
                        .set_size(size.width, size.height, window.scale_factor() as f32);
                    self.canvas.clear_rect(
                        0,
                        0,
                        size.width,
                        size.height,
                        Color::rgbf(0.0, 0.0, 0.0),
                    );

                    // Construct state for current frame
                    let mut meta = Meta {
                        frames_per_second,
                        delta_time,
                        mouse_position,
                        is_mouse_down,
                        is_mouse_released,
                        keys_hold: &keys_hold,
                        keys_down: &keys_down,
                        keys_up: &keys_up,
                    };

                    // Rendering
                    for render_layer in self.render_layers.iter_mut() {
                        (render_layer)(&mut self.canvas, &mut meta, &mut self.state);
                    }

                    // After Rendering
                    self.canvas.flush();
                    self.context.swap_buffers().expect("Could not swap buffers");

                    // Reset Meta
                    is_mouse_released = false;
                    keys_up.clear();
                    keys_down.clear();
                }
                Event::MainEventsCleared => window.request_redraw(),
                _ => (),
            };
        });
    }
    pub fn push(&mut self, render_layer: RenderLayerFn<T>) {
        self.render_layers.push(render_layer);
    }
    pub fn load_font(&mut self, path: &str) -> FontId {
        self.canvas.add_font(path).expect("Could not load font")
    }
}

pub struct Meta<'a> {
    pub frames_per_second: usize,
    pub delta_time: Duration,
    pub mouse_position: Vec2,
    pub is_mouse_down: bool,
    pub is_mouse_released: bool,
    pub keys_hold: &'a HashSet<VirtualKeyCode>,
    pub keys_up: &'a HashSet<VirtualKeyCode>,
    pub keys_down: &'a HashSet<VirtualKeyCode>,
}
