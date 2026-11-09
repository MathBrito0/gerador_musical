mod instruments;

use rodio::{OutputStream, Sink};
use std::{thread, time::Duration};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink_teclado = Sink::try_new(&stream_handle).unwrap();
    let sink_kick = Sink::try_new(&stream_handle).unwrap();
    let sink_snare = Sink::try_new(&stream_handle).unwrap();

    let pattern_kick = [true, false, true, false];
    let pattern_snare = [false, true, false, true];

    for _ciclo in 0..4 {
        for passo in 0..4 {

            if pattern_kick[passo] {
                println!("Bumbo no passo {}", passo);
                instruments::bateria(&sink_kick, 0.2, 0.05);
            }

            if pattern_snare[passo] {

                println!("Caixa no passo {}", passo);
                instruments::bateria(&sink_snare, 0.2, 0.05);
            }

            thread::sleep(Duration::from_millis(250));
        }
    }

    instruments::teclado(&sink_teclado, 261.63, 1, 0.1);

    sink_teclado.sleep_until_end();
    sink_kick.sleep_until_end();
    sink_snare.sleep_until_end();
}
