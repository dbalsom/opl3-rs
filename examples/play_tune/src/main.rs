//! This is an example program that plays a simple, 3-channel tune via opl3-rs and the `rodio` audio
//! library. It can optionally save the output to a wav file via `hound`.
//! This library uses a multithreaded timer callback via the `timer` crate to play the music and
//! generate audio samples which are sent to the main thread via `crossbeam` channels.
//!
//! Original code (C) Maarten Janssen (maarten@cheerful.nl) 2016-04-13
//! https://github.com/DhrBaksteen/ArduinoOPL2
//! Hacked for a OPL2LPT test program Peter De Wachter (pdewacht@gmail.com).
//! https://github.com/pdewacht/adlipt
//! Rewritten in Rust by Daniel Balsom for opl3-rs
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of this software
//! and associated documentation files (the “Software”), to deal in the Software without
//! restriction, including without limitation the rights to use, copy, modify, merge, publish,
//! distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
//! Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use bpaf::*;
use chrono::Duration;
use crossbeam_channel::unbounded;
use rodio::cpal::traits::HostTrait;
use rodio::DeviceTrait;
use timer::Timer;

use crate::music_player::CallbackMessage;
use crate::music_player::MusicPlayer;
use crate::opl::*;
use crate::opl_instruments::*;

mod music_player;
mod opl;
mod opl_instruments;

const TIMER_FREQ: i64 = 100; // We will set a timer callback at 100Hz

#[derive(Debug, Clone)]
struct Out {
    debug: Option<bool>,
    test_note: Option<bool>,
    output_wav: Option<PathBuf>,
}

fn opts() -> OptionParser<Out> {
    // Set up bpaf argument parsing.
    let debug = short('d')
        .long("debug")
        .help("Activate debug mode")
        .switch()
        .fallback(false)
        .optional();

    let test_note = short('t')
        .long("test_note")
        .help("Play a single test note for 1 second.")
        .switch()
        .fallback(false)
        .optional();

    let output_wav = short('w')
        .long("wav_file")
        .help("WAV File to write")
        .argument::<PathBuf>("FILE")
        .optional();

    construct!(Out {
        debug,
        test_note,
        output_wav
    })
    .to_options()
    .descr("opl3-rs: play_tune example")
}

fn main() {
    // This example uses rodio for playback.
    // rodio is built on top of the lower level 'cpal' audio library. If there is something we
    // cannot accomplish in rodio, we can fall back to the underlying cpal implementation.

    // We want to retrieve the system's sample rate in order to be able to provide it to opl3-rs.
    // However, I haven't found a way to retrieve the sample rate from a rodio stream, so we
    // will open the default device via cpal first, query the sample rate, then open the
    // rodio output stream from the cpal device.

    // Get the command line options.
    let opts = opts().run();

    // Possible improvement here: enumerate devices and let user select - or - attempt to open
    // other devices if the default device fails.
    let audio_device = rodio::cpal::default_host()
        .default_output_device()
        .expect("No audio device found.");

    let device_name = audio_device.name().expect("Couldn't get adapter name.");

    let config = audio_device
        .default_output_config()
        .expect("Couldn't get device configuration.");

    // We can now retrieve the sample rate. We don't really need the number of channels or sample
    // format, but it is displayed for informational purposes.
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;
    let sample_format = config.sample_format().to_string();

    let (_stream, stream_handle) =
        rodio::OutputStream::try_from_device(&audio_device).expect("Couldn't open rodio stream.");

    println!(
        "Opened audio device: {}, sample rate: {}, channels: {}, format: {:?}",
        device_name, sample_rate, channels, sample_format
    );

    let mut wav_out = None;

    if let Some(filename) = opts.output_wav {
        // If we've specified a wave file for output, open it now and create a BufWriter for it.
        let file = File::create(filename).expect("Couldn't create output file.");
        wav_out = Some(BufWriter::new(file));
    }

    // Play the test note if option -t was specified, otherwise play music.
    if opts.test_note.is_some_and(|b| b) {
        play_note::<BufWriter<File>>(sample_rate, stream_handle, wav_out.as_mut());
    } else {
        play_music::<BufWriter<File>>(
            sample_rate,
            stream_handle,
            wav_out.as_mut(),
            opts.debug.unwrap_or(false),
        );
    }
}

/// Initialize the MusicPlayer but only play a single, sustained note for 1 second.
/// This is useful for debugging and testing the audio output if the music player isn't working.
/// The generated samples are saved to the specified writer Option, if provided.
#[allow(dead_code)]
fn play_note<W: Write>(
    sample_rate: u32,
    stream_handle: rodio::OutputStreamHandle,
    _wav_out: Option<&mut W>,
) {
    // Create a stereo buffer one second long. (Length = Sample rate * 2 channels)
    let mut samples = vec![0; 2 * sample_rate as usize];

    // Create the music player. We don't use this channel in this example.
    let (s, _r) = unbounded();
    let mut player = MusicPlayer::new(sample_rate, s, false);

    // Start the player and play a single note, leaving it sustained.
    player.setup();
    player.play_test_note(&OPL_INSTRUMENT_ORGAN1, NOTE_C, 2);

    // Generate 1 second of samples.
    player.generate_direct(&mut samples);

    // Write buf to file
    let mut file = std::fs::File::create("test.raw").unwrap();
    for sample in &samples {
        file.write(&sample.to_le_bytes()).unwrap();
    }

    // Convert samples to f32.
    let channel_samples: Vec<f32> = samples
        .iter_mut()
        .map(|c| *c as f32 / i16::MAX as f32)
        .collect();

    for sample in &channel_samples[1000..2000] {
        println!("Sample: {}", sample);
    }

    println!("Got {} samples", channel_samples.len());
    let samples_buf = rodio::buffer::SamplesBuffer::new(2, sample_rate, channel_samples);

    stream_handle
        .play_raw(samples_buf)
        .expect("Couldn't play sound");

    std::thread::sleep(std::time::Duration::from_secs(1));
}

/// Play music using the MusicPlayer. This function sets up a timer callback to execute OPL3
/// commands and generate audio samples. The callback is fired at a fixed rate (100Hz), in a
/// separate thread. Crossbeam channels are used to send the generated samples to the main thread.
/// The message type is an enum of type CallbackMessage, and can incorporate either instructions
/// for the main thread or encapsulate audio samples.
fn play_music<W: Write + std::io::Seek>(
    sample_rate: u32,
    stream_handle: rodio::OutputStreamHandle,
    wav_out: Option<&mut W>,
    debug: bool,
) {
    // Create a channel to receive the audio samples as they are generated by the timer callback.
    // The channel here is unbounded, but you could calculate the number of samples you expect to
    // receive and use a bounded channel. I am not sure of the performance differences.
    let (s, r) = unbounded();

    // Create and initialize the music player.
    let mut player = MusicPlayer::new(sample_rate, s, debug);
    player.setup();

    // Wrap the player in an Arc<Mutex<>> so we can share it with the timer callback.
    let player_arc = Arc::new(Mutex::new(player));

    // Create a rodio 'sink' to play the audio samples. Since there is only one stream, we could
    // use the stream_handle.play_raw() method directly, but this is a more general approach.
    let sink = rodio::Sink::try_new(&stream_handle).expect("Couldn't create sink!");

    // Create and set up the timer. The 'Timer' crate is a bit old and unmaintained, but it seems
    // to still work well. You could use a different timer crate such as tokio::time.
    let freq_duration = Duration::milliseconds(1000 / TIMER_FREQ);
    let timer = Timer::new();

    // Set up the timer callback. The result of schedule_repeating() is stored in a guard variable
    // to determine the callback's lifetime. The callback thread will end when the guard is dropped.
    let _guard = {
        let player_arc_clone = Arc::clone(&player_arc);
        timer.schedule_repeating(freq_duration, move || {
            // Lock the player and call the player's timer callback.
            let mut player_lock = player_arc_clone
                .lock()
                .unwrap_or_else(|e| panic!("Error locking player: {:?}", e));
            player_lock.timer_callback();
        })
    };

    // If a writer was provided, create a Hound wav writer wrapped in Some, otherwise None.
    let mut wav_writer = if let Some(w) = wav_out {
        // Use our converted format, using the specified sample rate and 32-bit float samples.
        let wav_writer = hound::WavWriter::new(
            w,
            hound::WavSpec {
                channels: 2,
                sample_rate,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            },
        )
        .expect("Couldn't create wav writer.");

        Some(wav_writer)
    } else {
        None
    };

    // Start playing the rodio audio sink. This may not be strictly necessary as we never paused it.
    sink.play();

    // Loop and receive messages/samples from the callback. We use an outer flag to determine when
    // we are done. This flag is set by a CallbackMessage::EndPlayback message.
    let mut end_playback = false;
    while !end_playback {
        // Block until we receive a message. If we receive a read error, we treat it like we
        // received an error message from the channel.
        let message = r.recv().unwrap_or_else(|e| {
            eprintln!("Error receiving channel message: {:?}", e);
            // exit to os
            CallbackMessage::Error
        });
        match message {
            CallbackMessage::Error | CallbackMessage::EndPlayback => {
                // Either an error occurred, or we were instructed to end playback.
                end_playback = true;
            }
            CallbackMessage::HaveSamples(samples) => {
                // We received some audio samples. opl3-rs generates samples in i16 format, so we
                // need to convert them to f32 in the range -1.0 to 1.0 for rodio to play them back.
                let channel_samples: Vec<f32> = samples
                    .iter()
                    .map(|c| *c as f32 / i16::MAX as f32)
                    .collect();

                // If we have a wav writer, write the samples to the wav file.
                if let Some(wav_writer) = &mut wav_writer {
                    for sample in &channel_samples {
                        wav_writer.write_sample(*sample).unwrap();
                    }
                }

                // Create a SamplesBuffer out of our received samples and append them to the sink
                // to be played.
                let buf = rodio::buffer::SamplesBuffer::new(2, sample_rate, channel_samples);
                sink.append(buf);
            }
        }
    }

    // If we have a wav writer, finalize the wav file.
    if let Some(wav_writer) = wav_writer {
        wav_writer.finalize().expect("Couldn't finalize wav file.");
    }
}
