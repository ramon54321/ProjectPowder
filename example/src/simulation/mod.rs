use crate::RenderableState;
use std::{sync::mpsc::Sender, thread, time::Duration};

///
/// Runs in own thread. Responsible for simulation.
///
pub fn simulate(tx: Sender<RenderableState>) {
    // Setup Simulation

    // Run Simulation
    loop {
        // Update Simulation

        // Build renderable state
        let renderable_state = RenderableState {};

        // Send renderable state through channel to be rendered
        tx.send(renderable_state).unwrap();

        // Wait for next tick
        thread::sleep(Duration::from_millis(1000));
    }
}
