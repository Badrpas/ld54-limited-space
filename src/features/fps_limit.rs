use bevy::{prelude::*, input::common_conditions::input_toggle_active};

#[cfg(not(debug_assertions))]
const MAX_FPS: f64 = 120.;
#[cfg(debug_assertions)]
const MAX_FPS: f64 = 35.;
const DELAY: f64 = 1. / MAX_FPS;

pub struct FpsLimitPlugin;

impl Plugin for FpsLimitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            fps_limiter.run_if(input_toggle_active(true, KeyCode::F)),
        );
    }
}

pub fn fps_limiter(_time: Res<Time>) {
    let delay = DELAY; // - time.delta_seconds_f64();
    if delay > 0. {
        std::thread::sleep(std::time::Duration::from_secs_f64(delay));
    }
}
