use std::env;

use cpal::{EventLoop, StreamData, UnknownTypeOutputBuffer};

use adx_decoder::adx_file::AdxFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file to play");
        return;
    }

    let filename = &args[1];

    let adx_file = AdxFile::new(filename).unwrap();
    adx_file.print_info();
    let mut buffer_iterator = adx_file.get_decoder().unwrap();

    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().unwrap();

    let format = cpal::Format {
        channels: adx_file.channel_count as u16,
        sample_rate: cpal::SampleRate(adx_file.sample_rate),
        data_type: cpal::SampleFormat::I16,
    };


    let stream_id = match event_loop.build_output_stream(&device, &format) {
        Ok(id) => id,
        Err(e) => {
            println!("{:?}", e);
            panic!("Panic");
        }
    };
    event_loop.play_stream(stream_id);


    event_loop.run(move |_stream_id, stream_data| {
        match stream_data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    *elem = buffer_iterator.next().unwrap() as i16;
                }
            }
            _ => {}
        }
    });
}




