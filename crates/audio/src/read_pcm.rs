//! The WAV and AIFF PCM readers of `read` without its decoders (feature `pcm-decode`).

#[path = "read/aiff.rs"]
mod aiff;
#[path = "read/raw.rs"]
pub(crate) mod raw;
#[path = "read/source.rs"]
mod source;
#[path = "read/wav.rs"]
mod wav;
