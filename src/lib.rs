use crate::allpass::AllPassFilter;
use nice_plug::prelude::*;
use std::sync::Arc;

//use crate::allpass;

mod allpass;

pub struct PassoidPlugin {
    params: Arc<PassoidPluginParams>,
    filters: Vec<Vec<AllPassFilter>>,
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
        }
    }
}

impl Plugin for PassoidPlugin {
    const NAME: &'static str = "Passoid";
    const VENDOR: &'static str = "maxie";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "maxie.maxie";
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

    type Editor = ();
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
        let num_of_samples = buffer.samples();

        let cutoff = self.params.cutoff.value() * -1.0;
        let num_of_channels = buffer.channels();
        let buffer_slice = buffer.as_slice();

        let filters = self.params.filter_count.value();

        for channel in 0..num_of_channels {
            let filter_channel = &mut self.filters[channel];
            for samp in buffer_slice[channel].into_iter() {
                for allpass_i in 1..filters {
                    let allpass_filter_instance = &mut filter_channel[(allpass_i - 1) as usize];

                    *samp = allpass_filter_instance.process(*samp, cutoff);
                }
                //for allpass_filter_instance in filter_channel.into_iter() {}
            }
        }

        return ProcessStatus::Normal;

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for PassoidPlugin {
    const CLAP_ID: &'static str = "net.mzxie.passoid";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("maxieplug");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for PassoidPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"MyMaxiePassin001";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nice_export_clap!(PassoidPlugin);
nice_export_vst3!(PassoidPlugin);
