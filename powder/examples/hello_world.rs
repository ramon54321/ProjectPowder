use femtovg::{Color, LineCap, LineJoin, Paint, Path};
use nalgebra_glm::Vec2;
use powder::Powder;

struct State {
    line_start: Vec2,
}

fn main() {
    // Init State
    let state = State {
        line_start: Vec2::new(0.0, 0.0),
    };

    // Init Powder
    let powder = Powder::new(state, 800, 600, "Example");
    if powder.is_err() {
        eprintln!("Powder error: {}", powder.err().unwrap());
        return;
    }
    let mut powder = powder.unwrap();

    // Push a render layer
    powder.push(Box::new(|canvas, meta, state| {
        if meta.is_mouse_down {
            state.line_start.x = meta.mouse_position.x;
            state.line_start.y = meta.mouse_position.y;
        }

        let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
        paint.set_line_cap(LineCap::Butt);
        paint.set_line_join(LineJoin::Bevel);
        paint.set_line_width(1.0);

        let mut path = Path::new();
        path.move_to(state.line_start.x, state.line_start.y);
        path.line_to(meta.mouse_position.x, meta.mouse_position.y);
        canvas.stroke_path(&mut path, paint);
    }));

    // Start Powder
    powder.start();
}
