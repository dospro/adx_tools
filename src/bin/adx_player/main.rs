use std::env;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use adx_tools::adx_file::AdxFile;
use cpal::Devices;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file to play");
        return;
    }

    let filename = &args[1];

    let adx_file = AdxFile::new(filename).unwrap();
    adx_file.print_info();

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = cpal::StreamConfig {
        channels: adx_file.channel_count as u16,
        sample_rate: cpal::SampleRate(adx_file.sample_rate),
        buffer_size: cpal::BufferSize::Default,
    };

    let mut buffer_iterator = adx_file.get_into_iterator().unwrap();


    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for elem in data.iter_mut() {
                *elem = buffer_iterator.next().unwrap() as f32 / 32768.0f32;
            }
        },
        move |err| { println!("err: {:?}", err) },
    ).unwrap();
    println!("Starting to play");
    stream.play().expect("Unable to play");
    let mut option: String = String::new();
    println!("Press q to exit player");
    loop {
        std::io::stdin().read_line(&mut option).unwrap();
        match option.trim() {
            "q" => {
                println!("Command {} stop music", option);
                println!("Thread finished");
                break;
            }
            _ => {
                println!("Command {} no action", option);
            }
        }
    }
}
