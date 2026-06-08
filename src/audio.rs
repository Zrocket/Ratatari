use fundsp::{
    hacker::{ hammond_hz, multipass, reverb_stereo, sine, sine_hz, soft_saw_hz, square_hz, wavech, Wave,},
    prelude::AudioUnit
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device,
    FromSample,
    SampleFormat,
    SizedSample,
    StreamConfig
};
use std::sync::Arc;
use tokio;

pub fn sound_test() {
    let audio_graph = create_sine_440();
    let chord_graph = create_c_major();
    //let sample_graph = create_sample();
    //let sample_fm_graph = create_sample_fm();

    //run_output(audio_graph);
    //run_output(chord_graph);
    //run_output(sample_graph);
    tokio::spawn(async move {
        run_output(chord_graph);
        let duration = 5;
        std::thread::sleep(std::time::Duration::from_secs(duration));
    });
}

fn run_output(audio_graph: Box<dyn AudioUnit>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run_synth::<f32>(audio_graph, device, config.into()),
        SampleFormat::I16 => run_synth::<i16>(audio_graph, device, config.into()),
        SampleFormat::U16 => run_synth::<u16>(audio_graph, device, config.into()),

        _ => panic!("unsupported sample format"),
    }
}

fn run_synth<T: SizedSample + FromSample<f64>>(
    mut audio_graph: Box<dyn AudioUnit>,
    device: Device,
    config: StreamConfig,
    ) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        audio_graph.set_sample_rate(sample_rate);

        let mut next_value = move || audio_graph.get_stereo();

        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, &mut next_value)
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
}

fn write_data<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f32, f32),
    ) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = T::from_sample(sample.0.into());
        let right: T = T::from_sample(sample.1.into());

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}

fn create_sine_440() -> Box<dyn AudioUnit> {
    let synth = sine_hz(440.0);

    Box::new(synth)
}

fn create_c_major() -> Box<dyn AudioUnit> {
    let synth = sine_hz(261.6) + sine_hz(329.628) + sine_hz(391.995);
    // let synth = square_hz(261.6) + square_hz(329.628) + square_hz(391.995);
    // let synth = soft_saw_hz(261.6) + soft_saw_hz(329.628) + soft_saw_hz(391.995);
    // let synth = hammond_hz(261.6) + hammond_hz(329.628) + hammond_hz(391.995);

    Box::new(synth)
}

fn create_sample() -> Box<dyn AudioUnit> {
    let wave = 
        Arc::new(Wave::load("samples/sample.wav").expect("failed to load sample"));
    let left = wavech(&wave, 0, None);
    let right = wavech(&wave, 1, None);
    let synth = (left | right) >> (multipass() & (0.2 * reverb_stereo(10.0, 3.0, 0.5)));

    Box::new(synth)
}

fn create_sample_fm() -> Box<dyn AudioUnit> {
    let f = 440.0;
    let m = 5.0;
    let synth = (sine_hz(f) * f * m + f) >> sine();

    Box::new(synth)
}
