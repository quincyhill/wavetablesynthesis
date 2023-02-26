struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WaveTableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveTableOscillator {
        return WaveTableOscillator { 
            sample_rate: sample_rate ,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    fn set_frequency(&mut self, frequncy: f32) {
        self.index_increment = frequncy * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();
        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;
        return truncated_index_weight * self.wave_table[truncated_index] + next_index_weight * self.wave_table[next_index];
    }
}

fn main() {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        let value = (2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin();
        wave_table.push(value);
        println!("{}", value);
    }

    let mut oscillator = WaveTableOscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0)
}