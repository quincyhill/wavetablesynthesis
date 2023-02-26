fn main() {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        let value = (2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin();
        wave_table.push(value);
        println!("{}", value);
    }
}