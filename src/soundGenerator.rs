use std::{f32::consts::PI};

use macroquad::{audio::{Sound, load_sound_from_bytes, play_sound_once}, window::{next_frame, screen_width, screen_height, clear_background}, text::draw_text, prelude::{BLACK, WHITE}};


const CHUNK_ID:&str = "RIFF";
const CHUNK_SIZE:&str = "----";
const FORMAT:&str = "WAVE";

//FMT sub-chunk
const SUBCHUNK_1_ID:&str = "fmt ";
const SUBCHUNK_1_SIZE:i32 = 16;
const AUDIO_FORMAT:i16 = 1;
const NUM_CHANNELS:i16 = 1;
const SAMPLE_RATE:i32 = 44100;
const BYTE_RATE:i32 = SAMPLE_RATE * NUM_CHANNELS as i32 * (SUBCHUNK_1_SIZE / 8);
const BLOACK_ALIGN:i16 = NUM_CHANNELS * (SUBCHUNK_1_SIZE / 8) as i16;
const BITS_PR_SAMPLE:i16 = 16;

//Data sub-chunk
const SUBCHUNK_2_ID:&str = "data";
const SUBCHUNK_2_SIZE:&str = "----";


const MAX_AMPLITUDE:usize = 32760;


fn write_as_bytes(vec:&mut Vec<u8>, value:usize, byte_size:u8){
    let mut bytes = value.to_le_bytes();
    vec.append(&mut bytes[0..byte_size as usize].to_vec());
}

pub async fn generateTone(frequency: f32, duration:f32) -> Sound{

    // const CHUNK_ID:&str = "RIFF";
    // const CHUNK_SIZE:&str = "----";
    // const FORMAT:&str = "WAVE";

    // //FMT sub-chunk
    // const SUBCHUNK_1_ID:&str = "fmt ";
    // const SUBCHUNK_1_SIZE:u32 = 16;
    // const AUDIO_FORMAT:u16 = 1;
    // const NUM_CHANNELS:u16 = 1;
    // const SAMPLE_RATE:u32 = 44100;
    // const BYTE_RATE:u32 = SAMPLE_RATE * NUM_CHANNELS as u32 * (SUBCHUNK_1_SIZE / 8);
    // const BLOACK_ALIGN:u16 = NUM_CHANNELS * (SUBCHUNK_1_SIZE / 8) as u16;
    // const BITS_PR_SAMPLE:u16 = 16;

    // //Data sub-chunk
    // const SUBCHUNK_2_ID:&str = "data";
    // const SUBCHUNK_2_SIZE:&str = "----";


    // const MAX_AMPLITUDE:usize = 32760;

 
    let mut soundFileBytes = [
        CHUNK_ID.as_bytes(),
        CHUNK_SIZE.as_bytes(),
        FORMAT.as_bytes(),
        SUBCHUNK_1_ID.as_bytes(),
        &SUBCHUNK_1_SIZE.to_le_bytes(),
        &AUDIO_FORMAT.to_le_bytes(),
        &NUM_CHANNELS.to_le_bytes(),
        &SAMPLE_RATE.to_le_bytes(),
        &BYTE_RATE.to_le_bytes(),
        &BLOACK_ALIGN.to_le_bytes(),
        &BITS_PR_SAMPLE.to_le_bytes(),
        SUBCHUNK_2_ID.as_bytes(),
        SUBCHUNK_2_SIZE.as_bytes()
    ].concat();




    let startAudio = soundFileBytes.len();

    let mut collect = Vec::new();
    let lim = ((SAMPLE_RATE as f32 * duration) as usize);
    for i in 0..((SAMPLE_RATE as f32 * duration) as usize){
        let amplitude = 500. * f32::sin((i as f32 - 300.) / 1200.);
        let value = f32::sin((2. * PI * (i as f32) *  (frequency as f32)) / SAMPLE_RATE as f32);
        let channel = (amplitude  * if i+100 > lim {0.} else {value});

        collect.push(channel);
        soundFileBytes.append(&mut (channel as i16).to_le_bytes().to_vec());


    }
    let endAudio = soundFileBytes.len();


    let mut holder = Vec::new();
    write_as_bytes(&mut holder, endAudio-startAudio, 4);
    for i in 0..4{
        soundFileBytes[(startAudio-4)+i] = holder[i];
    }
    holder.clear();
    write_as_bytes(&mut holder, 36+endAudio-startAudio, 4);

    for i in 0..4{
        soundFileBytes[4+i] = holder[i];
    }



    let sound = load_sound_from_bytes(&soundFileBytes).await.expect("Failed to load");
 
    sound
}
