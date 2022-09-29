use self::components::draw_horizontal_slider;
use crate::RenderableState;
use femtovg::{renderer::OpenGl, Canvas, FontId};
use powder::{Meta, Powder, RenderLayerFn};
use std::{collections::HashMap, sync::mpsc::Receiver};

mod components;

pub struct PowderState {
    renderable_state: RenderableState,
    dynamic_state: HashMap<String, String>,
    font: Option<FontId>,
}
impl Default for PowderState {
    fn default() -> Self {
        Self {
            renderable_state: Default::default(),
            dynamic_state: Default::default(),
            font: None,
        }
    }
}

pub fn render(rx: Receiver<RenderableState>) {
    // Define layers for powder renderer
    let layers: Vec<RenderLayerFn<PowderState>> = vec![Box::new(render_game)];

    // Setup powder instance with initial state
    let mut powder =
        Powder::new(PowderState::default(), 1200, 800).expect("Could not start powder");

    // Load after-init assets
    powder.state.font = Some(powder.load_font("assets/Roboto-Regular.ttf"));

    // Push thread receiver layer
    powder.push(Box::new(move |_canvas, _meta, state| {
        match rx.try_recv() {
            Ok(new_renderable_state) => state.renderable_state = new_renderable_state,
            _ => (),
        };
    }));

    // Push custom layers
    for layer in layers {
        powder.push(layer);
    }

    // Start graphics
    powder.start();
}

fn render_game(canvas: &mut Canvas<OpenGl>, meta: &mut Meta, state: &mut PowderState) {
    draw_horizontal_slider(canvas, 50.0, 50.0, 400.0, 0.0, 100.0, 25.0);
}
