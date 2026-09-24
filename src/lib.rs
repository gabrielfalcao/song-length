#![allow(unused)]
pub(crate) mod errors;
pub use errors::{Error, Exit, Result};
pub mod dispatch;
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub fn calculate_length_in_bars(song_length_in_minutes: f32, bpm: f32, beats_per_bar: f32) -> f32 {
    let total_beats = song_length_in_minutes * bpm;
    total_beats / beats_per_bar
}

pub fn calculate_bars_to_length(bars: f32, bpm: f32, beats_per_bar: f32) -> f32 {
    let total_beats = bars * beats_per_bar;
    total_beats / bpm
}

#[test]
pub fn test_calculate_length_in_bars() {
    assert_eq!(calculate_length_in_bars(5.0, 128.0, 4.0), 160.0);
}

#[test]
pub fn test_calculate_bars_to_length() {
    assert_eq!(calculate_bars_to_length(160.0, 128.0, 4.0), 5.0);
}
