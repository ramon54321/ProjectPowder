use femtovg::{Color, LineCap, LineJoin, Paint, Path};
use powder::Powder;

fn main() {
    // Init Powder
    let powder = Powder::new();
    if powder.is_err() {
        eprintln!("Powder error: {}", powder.err().unwrap());
        return;
    }
    let mut powder = powder.unwrap();

    // Push a render layer
    powder.push(Box::new(|canvas| {
        let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
        paint.set_line_cap(LineCap::Butt);
        paint.set_line_join(LineJoin::Bevel);
        paint.set_line_width(1.0);

        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.line_to(200.0, 200.0);
        canvas.stroke_path(&mut path, paint);
    }));

    // Start Powder
    powder.start();
}
