use rand::Rng;
use rodio::buffer::SamplesBuffer;
use rodio::source::SineWave;
use rodio::{Sink, source::Source};
use std::time::Duration;

// Função do teclado / Synth

pub fn teclado(sink: &Sink, frequencia: f32, segundos: u64, volume: f32) {
    let nota = SineWave::new(frequencia)
        .take_duration(Duration::from_secs(segundos))
        .amplify(volume);

    sink.append(nota);
}

// Função da bateria (ruído aleatório com o rand)

pub fn bateria(sink: &Sink, duracao_segundos: f32, volume: f32) {
    let sample_rate = 44100; // 44.1 kHz (padrão de CD/áudio)
    let num_samples = (sample_rate as f32 * duracao_segundos) as usize;
    let mut rng = rand::thread_rng();

    // Cria um vetor de números aleatórios entre -1.0 e +1.0 (amostras de áudio)
    let mut buffer: Vec<f32> = (0..num_samples)
        .map(|_| rng.gen_range(-1.0..1.0) * volume)
        .collect();



    let total_amostras = buffer.len();
    for (i, sample) in buffer.iter_mut().enumerate() {
        let progresso = i as f32 / total_amostras as f32;

        let envelope = 1.0 - progresso;     // Envelope linear simples: o volume diminui à medida que o tempo passa
        *sample *= envelope;

    }



    // Junta o buffer em uma fonte de áuido que o Rodio entende (1 canal, 44100 Hz)

    let source = SamplesBuffer::new(1, sample_rate, buffer);
    sink.append(source);
}
