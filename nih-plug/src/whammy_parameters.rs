use nih_plug::prelude::{FloatParam, FloatRange, Params};
use nih_plug_vizia::ViziaState;
use whammy::{MAX_PITCH, MIN_PITCH};
use std::sync::Arc;
mod custom_formatters;
use custom_formatters::v2s_f32_digits;

use crate::editor;

#[derive(Params)]
pub struct WhammyParameters {
  /// The editor state, saved together with the parameter state so the custom scaling can be
  /// restored.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,
  
  #[id = "dry"]
  pub dry: FloatParam,
  
  #[id = "wet"]
  pub wet: FloatParam,

  #[id = "pitch"]
  pub pitch: FloatParam,
}

impl Default for WhammyParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      dry: FloatParam::new(
        "Dry",
        0.,
        FloatRange::SymmetricalSkewed {
          min: -70.,
          max: 6.,
          factor: 0.333333,
          center: 0.
        },
      )
      .with_unit(" dB")
      .with_value_to_string(Arc::new(move |value| {
        if value == -70. {
          "-inf".to_string()
        } else {
          format!("{:.2}", value)
        }
      })),
      
      wet: FloatParam::new(
        "Wet",
        0.,
        FloatRange::SymmetricalSkewed {
          min: -70.,
          max: 6.,
          factor: 0.333333,
          center: 0.
        },
      )
      .with_unit(" dB")
      .with_value_to_string(Arc::new(move |value| {
        if value == -70. {
          "-inf".to_string()
        } else {
          format!("{:.2}", value)
        }
      })),

      pitch: FloatParam::new(
        "Pitch",
        12.,
        FloatRange::Linear {
          min: MIN_PITCH,
          max: MAX_PITCH,
        },
      )
      .with_value_to_string(v2s_f32_digits(2)),
    }
  }
}
