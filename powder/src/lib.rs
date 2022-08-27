use femtovg::{renderer::OpenGl, Canvas, Color};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};

pub type RenderLayerFn = Box<dyn FnMut(&mut Canvas<OpenGl>) -> ()>;

pub struct Powder {
    event_loop: EventLoop<()>,
    context: ContextWrapper<PossiblyCurrent, Window>,
    canvas: Canvas<OpenGl>,
    render_layers: Vec<RenderLayerFn>,
}
impl Powder {
    pub fn new() -> Result<Self, String> {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new().with_title("Jimmy");

        // Build window context
        let context = ContextBuilder::new().build_windowed(window_builder, &event_loop);
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
            event_loop,
            context,
            canvas,
            render_layers: Vec::new(),
        })
    }
    pub fn start(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            let window = self.context.window();

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    // Before Rendering
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

                    // Rendering
                    for render_layer in self.render_layers.iter_mut() {
                        (render_layer)(&mut self.canvas);
                    }

                    // After Rendering
                    self.canvas.flush();
                    self.context.swap_buffers().expect("Could not swap buffers");
                }
                Event::MainEventsCleared => window.request_redraw(),
                _ => (),
            };
        });
    }
    pub fn push(&mut self, render_layer: RenderLayerFn) {
        self.render_layers.push(render_layer);
    }
}
