pub struct Decoder {
    coefficient_1: f64,
    coefficient_2: f64,
    previous_sample_1: i32,
    previous_sample_2: i32,
}

impl Decoder {
    pub fn new(highpass_frequency: u32, sample_rate: u32) -> Self {
        let factor: f64 = highpass_frequency as f64 / sample_rate as f64;
        let a: f64 = 2.0f64.sqrt() - (2.0f64 * (-1.0f64).acos() * factor).cos();
        let b: f64 = 2.0f64.sqrt() - 1.0f64;
        let c: f64 = (a - ((a + b) * (a - b)).sqrt()) / b;

        let coefficient_1: f64 = c * 2.0f64;
        let coefficient_2: f64 = -(c * c);


        Self {
            coefficient_1,
            coefficient_2,
            previous_sample_1: 0,
            previous_sample_2: 0,
        }
    }

    pub fn decode_sample(&mut self, sample: i32, scale: i32) -> i32 {
        let sample = if (sample & 8) != 0 { sample - 16 } else { sample };

        let prediction_value = self.coefficient_1 * self.previous_sample_1 as f64 + self.coefficient_2 * self.previous_sample_2 as f64;
        let mut decoded_sample = sample * scale + prediction_value as i32;


        if decoded_sample > 32767 {
            decoded_sample = 32767;
        } else if decoded_sample < -32768 {
            decoded_sample = -32767;
        }
        self.previous_sample_2 = self.previous_sample_1;
        self.previous_sample_1 = decoded_sample;
        decoded_sample
    }
}