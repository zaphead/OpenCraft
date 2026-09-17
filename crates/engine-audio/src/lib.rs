//! Mixer: positional voices + categories. cpal stays private.

use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use engine_core::Vec3;
use parking_lot::Mutex;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Category {
    Master,
    Music,
    Players,
    Weather,
}

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("audio device: {0}")]
    Device(String),
    #[error("decode: {0}")]
    Decode(String),
}

#[derive(Clone)]
pub struct Clip {
    pub samples: Arc<Vec<f32>>,
    pub rate: u32,
}

struct Voice {
    clip: Clip,
    cursor: f32,
    step: f32,
    gain_l: f32,
    gain_r: f32,
    cat: Category,
}

struct State {
    voices: Vec<Voice>,
    vols: [f32; 4],
    listener: Vec3,
    listener_yaw: f32,
    out_rate: u32,
}

pub struct Mixer {
    state: Arc<Mutex<State>>,
    clips: HashMap<u16, Clip>,
    _stream: Option<cpal::Stream>,
}

impl Mixer {
    pub fn start() -> Result<Self, AudioError> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| AudioError::Device("no output".into()))?;
        let cfg = device
            .default_output_config()
            .map_err(|e| AudioError::Device(e.to_string()))?;
        let out_rate = cfg.sample_rate().0;
        let channels = cfg.channels() as usize;
        let state = Arc::new(Mutex::new(State {
            voices: Vec::new(),
            vols: [1.0; 4],
            listener: Vec3::ZERO,
            listener_yaw: 0.0,
            out_rate,
        }));
        let st = Arc::clone(&state);
        let stream = device
            .build_output_stream(
                &cfg.config(),
                move |data: &mut [f32], _| mix(data, channels, &st),
                |e| eprintln!("audio: {e}"),
                None,
            )
            .map_err(|e| AudioError::Device(e.to_string()))?;
        stream.play().map_err(|e| AudioError::Device(e.to_string()))?;
        Ok(Self {
            state,
            clips: HashMap::new(),
            _stream: Some(stream),
        })
    }

    pub fn silent() -> Self {
        Self {
            state: Arc::new(Mutex::new(State {
                voices: Vec::new(),
                vols: [1.0; 4],
                listener: Vec3::ZERO,
                listener_yaw: 0.0,
                out_rate: 48000,
            })),
            clips: HashMap::new(),
            _stream: None,
        }
    }

    pub fn set_listener(&self, pos: Vec3, yaw: f32) {
        let mut s = self.state.lock();
        s.listener = pos;
        s.listener_yaw = yaw;
    }

    pub fn register(&mut self, id: u16, clip: Clip) {
        self.clips.insert(id, clip);
    }

    pub fn play(&self, id: u16, pos: Vec3, cat: Category) {
        let Some(clip) = self.clips.get(&id) else {
            return;
        };
        let mut s = self.state.lock();
        if s.voices.len() >= 32 {
            s.voices.remove(0);
        }
        let d = (pos - s.listener).length().max(0.1);
        let att = (1.0 / (1.0 + d * 0.15)).clamp(0.0, 1.0);
        let rel = pos - s.listener;
        let right = Vec3::new(s.listener_yaw.cos(), 0.0, s.listener_yaw.sin());
        let pan = right.dot(rel).clamp(-1.0, 1.0) * 0.5;
        let step = clip.rate as f32 / s.out_rate as f32;
        s.voices.push(Voice {
            clip: clip.clone(),
            cursor: 0.0,
            step,
            gain_l: att * (0.5 - pan),
            gain_r: att * (0.5 + pan),
            cat,
        });
    }
}

fn mix(out: &mut [f32], channels: usize, st: &Arc<Mutex<State>>) {
    let mut s = st.lock();
    for f in out.iter_mut() {
        *f = 0.0;
    }
    let master = s.vols[0];
    let mut i = 0;
    while i < s.voices.len() {
        let cat_v = s.vols[cat_i(s.voices[i].cat)];
        let g = master * cat_v;
        let frames = out.len() / channels;
        let mut done = false;
        for f in 0..frames {
            let v = &s.voices[i];
            let idx = v.cursor as usize;
            if idx >= v.clip.samples.len() {
                done = true;
                break;
            }
            let smp = v.clip.samples[idx];
            if channels >= 2 {
                out[f * channels] += smp * v.gain_l * g;
                out[f * channels + 1] += smp * v.gain_r * g;
            } else {
                out[f * channels] += smp * ((v.gain_l + v.gain_r) * 0.5) * g;
            }
            s.voices[i].cursor += s.voices[i].step;
        }
        if done {
            s.voices.remove(i);
        } else {
            i += 1;
        }
    }
}

fn cat_i(c: Category) -> usize {
    match c {
        Category::Master => 0,
        Category::Music => 1,
        Category::Players => 2,
        Category::Weather => 3,
    }
}

pub fn decode_ogg<R: Read + std::io::Seek>(r: R) -> Result<Clip, AudioError> {
    let mut ogg = lewton::inside_ogg::OggStreamReader::new(r)
        .map_err(|e| AudioError::Decode(e.to_string()))?;
    let rate = ogg.ident_hdr.audio_sample_rate;
    let ch = ogg.ident_hdr.audio_channels.max(1);
    let mut samples = Vec::new();
    while let Some(pkt) = ogg.read_dec_packet_itl().map_err(|e| AudioError::Decode(e.to_string()))? {
        if ch == 1 {
            for s in pkt {
                samples.push(s as f32 / 32768.0);
            }
        } else {
            for frame in pkt.chunks(ch as usize) {
                let mut acc = 0.0;
                for s in frame {
                    acc += *s as f32 / 32768.0;
                }
                samples.push(acc / ch as f32);
            }
        }
    }
    Ok(Clip {
        samples: Arc::new(samples),
        rate,
    })
}
