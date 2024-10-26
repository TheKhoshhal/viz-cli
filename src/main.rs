use std::sync::mpsc::*;

use std::io::{self, Write};
use std::{thread, time};

use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::{
    cursor::MoveTo,
    style::{PrintStyledContent, Stylize},
    terminal::{self, Clear},
    ExecutableCommand, QueueableCommand,
};

mod calculations;

fn main() -> io::Result<()> {
    let pa = portaudio::PortAudio::new().expect("Unable to init PortAudio");
    let mic_index = pa
        .default_input_device()
        .expect("Unable to get default device");
    let mic = pa.device_info(mic_index).expect("unable to get mic info");

    let input_params =
        portaudio::StreamParameters::<f32>::new(mic_index, 1, true, mic.default_low_input_latency);

    let input_settings =
        portaudio::InputStreamSettings::new(input_params, mic.default_sample_rate, 256);

    let (sender, receiver) = channel();

    let callback =
        move |portaudio::InputStreamCallbackArgs { buffer, .. }| match sender.send(buffer) {
            Ok(_) => portaudio::Continue,
            Err(_) => portaudio::Complete,
        };

    let mut stream = pa
        .open_non_blocking_stream(input_settings, callback)
        .expect("Unable to create stream");
    stream.start().expect("Unable to start stream");

    // while stream.is_active().unwrap() {
    //     while let Ok(buffer) = receiver.try_recv() {
    //         println!("{:?}", buffer);
    //     }
    // }

    let mut sc = io::stdout();

    while stream.is_active().unwrap() {
        while let Ok(buffer) = receiver.try_recv() {
            let num_vec = buffer.to_vec();
            let (x, y) = crossterm::terminal::size().unwrap();

            if y <= num_vec.len() as u16 {
                let new_vec = calculations::shrink_vec(&num_vec, &(x as usize));
                let percentage = calculations::calculate_percentage(&new_vec, &(y as f32));

                sc.execute(Clear(terminal::ClearType::All))?;

                for i in 0..new_vec.len() {
                    if percentage[i] != y && percentage[i] != 0 {
                        for j in (y - percentage[i])..y {
                            sc.queue(MoveTo(i as u16, j))?;
                            sc.queue(PrintStyledContent("█".cyan()))?;
                        }
                    } else if percentage[i] == y {
                        for j in 0..y {
                            sc.queue(MoveTo(i as u16, j))?;
                            sc.queue(PrintStyledContent("█".cyan()))?;
                        }
                    } else {
                        sc.queue(MoveTo(i as u16, y - 1))?;
                        sc.queue(PrintStyledContent("▂".cyan()))?;
                    }
                }

                sc.flush()?;

                let sleep_time = time::Duration::from_millis(100);
                thread::sleep(sleep_time);
            } else {
                panic!("sorry doesn't work yet");
            }
        }
    }

    Ok(())
}
