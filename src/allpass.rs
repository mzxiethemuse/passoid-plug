/// Single order allpass filter.
pub struct AllPassFilter {
    last_in: f32,
    last_out: f32,
}

impl Default for AllPassFilter {
    fn default() -> Self {
        Self {
            last_in: 0.0,
            last_out: 0.0,
        }
    }
}

impl Clone for AllPassFilter {
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }

    fn clone(&self) -> Self {
        Self {
            last_in: self.last_in.clone(),
            last_out: self.last_out.clone(),
        }
    }
}

impl AllPassFilter {
    /// Processes a sample into the allpass filter with the given cutoff.
    pub fn process(&mut self, sample: f32, cutoff: f32) -> f32 {
        /*
         * (cutoff * *sample) + (self.prev_sample_l - (cutoff * self.prev_out_l));
         */
        let out: f32 = (cutoff * sample) + self.last_in - (cutoff * self.last_out);
        self.last_in = sample;

        self.last_out = out;
        return out;
    }
}
