use powder_example::graphics::render;
use powder_example::simulation::simulate;
use powder_example::RenderableState;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // Set up channel between simulation and graphics
    let (tx, rx) = channel::<RenderableState>();

    // Simulation
    let simulation_thread = thread::spawn(move || simulate(tx));

    // Graphics
    render(rx);

    // Graceful shutdown
    simulation_thread
        .join()
        .expect("Could not join thread on exit");
}
