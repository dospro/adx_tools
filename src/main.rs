use std::env;
use std::thread;
use std::sync::mpsc;

use cpal::{StreamData, UnknownTypeOutputBuffer};
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

use adx_tools::adx_file::AdxFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file to play");
        return;
    }

    let filename = &args[1];

    let adx_file = AdxFile::new(filename).unwrap();
    adx_file.print_info();


//    for dev in devices() {
//        println!("Device: {}", dev.name());
//        for format in dev.supported_output_formats().unwrap() {
//            println!("...Format {:?}", format.with_max_sample_rate());
//        }
//    }

    let host = cpal::default_host();
    let event_loop = host.event_loop();

    let device = host.default_output_device().unwrap();

    let format = cpal::Format {
        channels: adx_file.channel_count as u16,
        sample_rate: cpal::SampleRate(adx_file.sample_rate),
        data_type: cpal::SampleFormat::F32,
    };


    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

    event_loop.play_stream(stream_id);
    let (tx, rx) = mpsc::channel();

    let mut thread_handle = thread::spawn(move || {
        let mut buffer_iterator = adx_file.get_decoder().unwrap();
        let el = &event_loop;
        el.run(move |stream_id, stream_result| {
            let stream_data = match stream_result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("An error ocurred with the stream: {}", err);
                    return;
                }
            };
            let value = rx.try_recv();
            if value == Ok(0) {
                el.destroy_stream(stream_id);
                println!("Quit signal");
                return;
            }
            match stream_data {
                StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        *elem = buffer_iterator.next().unwrap() as i16;
                    }
                }
                StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        *elem = buffer_iterator.next().unwrap() as f32 / 32768.0f32;
                    }
                }
                _ => {}
            }
        });
        println!("Finished");
    });

    let mut option: String = String::new();
    loop {
        std::io::stdin().read_line(&mut option);
        match option.trim() {
            "q" => {
                println!("Command {} stop music", option);
                tx.send(0);
                println!("Thread finished");
                break;
            }
            _ => {
                println!("Command {} no action", option);
            }
        }
    }
}




