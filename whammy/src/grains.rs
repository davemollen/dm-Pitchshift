mod grain;
use grain::Grain;
mod phasor;
use phasor::Phasor;

use crate::{shared::{delay_line::DelayLine, delta::Delta}, MIN_PITCH};

const VOICES: usize = 4;
const GRAIN_FREQ_MULTIPLIER: f32 = 16.;

pub struct Grains {
  grain_delay_line: DelayLine,
  grains: Vec<Grain>,
  phasor: Phasor,
  delta: Delta,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    let min_grain_freq = GRAIN_FREQ_MULTIPLIER * Self::pitch_to_speed(MIN_PITCH).abs();

    Self {
      grain_delay_line: DelayLine::new((sample_rate * min_grain_freq.recip().ceil()) as usize, sample_rate),
      grains: vec![Grain::new(sample_rate); VOICES * 2],
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
    }
  }

  pub fn process(&mut self, input: f32, pitch: f32, freq: Option<f32>) -> f32 {
    let speed = Self::pitch_to_speed(pitch);
    match freq {
      Some(freq) => {
        let grain_freq = Self::get_grain_freq(freq, speed);
        let phasor = self.phasor.process(grain_freq * VOICES as f32);
        let trigger = self.delta.process(phasor) < 0.;

        if trigger {
          self.set_grain_parameters(grain_freq, speed);
        }
      }
      None => (),
    };

    let grain_delay_line = &mut self.grain_delay_line;
    let output = self
      .grains
      .iter_mut()
      .filter(|grain| !grain.is_free())
      .map(|grain| grain.process(grain_delay_line, speed))
      .sum::<f32>() * (VOICES as f32 / 2.).recip();

    self.grain_delay_line.write(input);

    output
  }
  
  fn set_grain_parameters(&mut self, freq: f32, speed: f32) {
    let window_size = 1000. / freq;

    let grain = self.grains.iter_mut().find(|grain | grain.is_free());
    match grain {
      Some(grain) => {
        grain.set_parameters(freq, window_size, speed);
      }
      None => {}
    }
  }

  fn pitch_to_speed(pitch: f32) -> f32 {
    1. - 2_f32.powf(pitch / 12.)
  }

  fn get_grain_freq(freq: f32, speed: f32) -> f32 {
    freq * speed.abs() / (freq / GRAIN_FREQ_MULTIPLIER).trunc()
  }
}
