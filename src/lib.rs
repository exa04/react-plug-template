mod params;

use crate::params::*;

use nih_plug::prelude::*;
use react_plug::prelude::*;

use include_dir::{include_dir, Dir};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub struct {{struct_name}} {
    params: Arc<{{struct_name}}Params>,
}

impl Default for {{struct_name}} {
    fn default() -> Self {
        Self {
            params: Arc::new({{struct_name}}Params::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../gui/src/bindings/GuiMessage.ts")]
enum GuiMessage { }

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../gui/src/bindings/PluginMessage.ts")]
enum PluginMessage { }

static EDITOR_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/gui/dist");

impl Plugin for {{struct_name}} {
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        ReactPlugEditor::<PluginMessage, GuiMessage>::new(
                self.params.clone(),
                &EDITOR_DIR,
                (800, 600),
            )
            .with_developer_mode(true)
            .into()
    }

    const NAME: &'static str = "{{ plugin_name }}";
    const VENDOR: &'static str = "{{ author }}";
    const URL: &'static str = "{{ url }}";
    const EMAIL: &'static str = "{{ email }}";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
}

impl ClapPlugin for {{ struct_name }} {
    const CLAP_ID: &'static str = "{{ clap_id }}";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("{{ description }}");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for {{ struct_name }} {
    const VST3_CLASS_ID: [u8; 16] = *b"{{ vst3_id }}";

    // Don't forget to change these features
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_vst3!({{struct_name}});
nih_export_clap!({{struct_name}});
