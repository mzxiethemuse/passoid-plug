use crate::allpass::AllPassFilter;
use crate::gui::*;

use nice_plug::{editor::dpi::LogicalSize, prelude::*};
use nice_plug_iced::{
    IcedEditor, IcedEditorState, IcedNiceSettings, application, create_iced_editor,
    iced::PollSubNotifier,
};
use std::sync::Arc;

const MIN_WINDOW_SIZE: LogicalSize<f32> = LogicalSize::new(400.0, 170.0);
const RESIZE_HINT: ResizeHint = ResizeHint::non_resizable().with_min_logical_size(MIN_WINDOW_SIZE);
const INITIAL_SCALE_FACTOR: f32 = 1.0;

mod allpass;
mod gui;

pub struct PassoidPlugin {
    params: Arc<PassoidPluginParams>,
    filters: Vec<Vec<AllPassFilter>>,
    editor_state: Arc<IcedEditorState>,
    notifier: PollSubNotifier,
}

pub fn lerp(a: f32, b: f32, delta: f32) -> f32 {
    return a + ((b - a) * delta);
}

#[derive(Params)]
struct PassoidPluginParams {
    #[id = "cutoff"]
    pub cutoff: FloatParam,
    #[id = "num_of_filters"]
    pub filter_count: IntParam,
}

impl Default for PassoidPluginParams {
    fn default() -> Self {
        Self {
            cutoff: FloatParam::new("cutoff", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            filter_count: IntParam::new("filters", 0, IntRange::Linear { min: 1, max: 200 }),
        }
    }
}

impl Default for PassoidPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(PassoidPluginParams::default()),
            filters: vec![vec![AllPassFilter::default(); 200]; 2],
            editor_state: IcedEditorState::from_size(MIN_WINDOW_SIZE, INITIAL_SCALE_FACTOR),
            notifier: PollSubNotifier::new(),
        }
    }
}

impl Plugin for PassoidPlugin {
    const NAME: &'static str = "passoid";
    const VENDOR: &'static str = "maxie";
    const URL: &'static str = "https://x.com/helloiammaxie";
    const EMAIL: &'static str = "maxie@maxie";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type Editor = IcedEditor;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        //let num_of_samples = buffer.samples();

        let cutoff = self.params.cutoff.value() * -1.0;
        let num_of_channels = buffer.channels();
        let buffer_slice = buffer.as_slice();

        let filters = self.params.filter_count.value();

        // manually iterate samples
        for channel in 0..num_of_channels {
            let filter_channel = &mut self.filters[channel];
            for samp in buffer_slice[channel].into_iter() {
                for allpass_i in 1..filters {
                    // go thru every filter in series
                    let allpass_filter_instance = &mut filter_channel[(allpass_i - 1) as usize];

                    *samp = allpass_filter_instance.process(*samp, cutoff);
                }
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Self::Editor> {
        create_iced_editor(
            self.editor_state.clone(),
            EditorState {
                params: self.params.clone(),
            },
            self.notifier.clone(),
            IcedNiceSettings::new().with_resize_hint(RESIZE_HINT),
            |editor_state, nice_ctx| {
                Ok(application(
                    editor_state,
                    nice_ctx,
                    PassoidGui::new,
                    PassoidGui::update,
                    PassoidGui::view,
                )
                .theme(PassoidGui::theme)
                .scale_factor(PassoidGui::scale_factor)
                .subscription(PassoidGui::subscription)
                .run())
            },
        )
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for PassoidPlugin {
    const CLAP_ID: &'static str = "net.mzxie.passoid";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("many allpass filters");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Filter,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for PassoidPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"MaxiePassoid0123";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
        Vst3SubCategory::Filter,
        Vst3SubCategory::Modulation,
    ];
}

nice_export_clap!(PassoidPlugin);
nice_export_vst3!(PassoidPlugin);
