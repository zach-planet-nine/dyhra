use macroquad::color::{ORANGE, RED};
use macroquad_particles::{ColorCurve, Emitter, EmitterConfig};

#[derive(Default, Clone, Copy)]
pub struct PES {}

impl PES {
    pub fn explosion(&self) -> Emitter {
      Emitter::new(self.particle_explosion())
    }

    fn particle_explosion(&self) -> EmitterConfig {
		EmitterConfig {
			local_coords: false,
			one_shot: true,
			emitting: true,
			lifetime: 0.6,
			lifetime_randomness: 0.3,
			explosiveness: 0.65,
			initial_direction_spread: 2.0 * std::f32::consts::PI,
			initial_velocity: 300.0,
			initial_velocity_randomness: 0.8,
			size: 3.0,
			size_randomness: 0.3,
			colors_curve: ColorCurve {
			start: RED,
			mid: ORANGE,
			end: RED,
			},
			..Default::default()
		}
    }
}
