use std::f32::consts::PI;

use macroquad::audio::{Sound, load_sound_from_bytes};


const CHUNK_ID:&str = "RIFF";
const CHUNK_SIZE:&str = "----";
const FORMAT:&str = "WAVE";

//FMT sub-chunk
const SUBCHUNK_1_ID:&str = "fmt ";
const SUBCHUNK_1_SIZE:usize = 16;
const AUDIO_FORMAT:usize = 1;
const NUM_CHANNELS:usize = 1;
const SAMPLE_RATE:usize = 44100;
const BYTE_RATE:usize = SAMPLE_RATE * NUM_CHANNELS * (SUBCHUNK_1_SIZE / 8);
const BLOACK_ALIGN:usize = NUM_CHANNELS * (SUBCHUNK_1_SIZE / 8);
const BITS_PR_SAMPLE:usize = 16;

//Data sub-chunk
const SUBCHUNK_2_ID:&str = "data";
const SUBCHUNK_2_SIZE:&str = "----";


const MAX_AMPLITUDE:usize = 32760;


fn write_as_bytes(vec:&mut Vec<u8>, value:usize, byte_size:u8){
    let mut bytes = value.to_le_bytes();
    vec.append(&mut bytes[0..byte_size as usize].to_vec());
}

pub async fn generateTone(frequency: f32, duration:f32) -> Sound{

    let mut soundFileBytes = Vec::new();

    soundFileBytes.append(&mut CHUNK_ID.clone().as_bytes().to_vec());

    soundFileBytes.append(&mut CHUNK_SIZE.clone().as_bytes().to_vec());

    soundFileBytes.append(&mut FORMAT.clone().as_bytes().to_vec());

    soundFileBytes.append(&mut SUBCHUNK_1_ID.clone().as_bytes().to_vec());

    write_as_bytes(&mut soundFileBytes, SUBCHUNK_1_SIZE, 4);

    write_as_bytes(&mut soundFileBytes, AUDIO_FORMAT, 2) ;

    write_as_bytes(&mut soundFileBytes, NUM_CHANNELS, 2);

    write_as_bytes(&mut soundFileBytes, SAMPLE_RATE, 4);
    
    write_as_bytes(&mut soundFileBytes, BYTE_RATE, 4);

    write_as_bytes(&mut soundFileBytes, BLOACK_ALIGN, 2);
    write_as_bytes(&mut soundFileBytes, BITS_PR_SAMPLE, 2);

    soundFileBytes.append(&mut SUBCHUNK_2_ID.clone().as_bytes().to_vec());

    soundFileBytes.append(&mut SUBCHUNK_2_SIZE.clone().as_bytes().to_vec());


    let startAudio = soundFileBytes.len();

    let mut collect = Vec::new();
    for i in 0..((SAMPLE_RATE as f32 * duration) as usize){
        let amplitude = 1500. / SAMPLE_RATE as f32 * MAX_AMPLITUDE as f32;
        let value = f32::sin((2. * PI * (i as f32) *  (frequency as f32)) / SAMPLE_RATE as f32);
        let channel = (amplitude * value);

        collect.push(channel);
        soundFileBytes.append(&mut (channel as i16).to_le_bytes().to_vec())
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
