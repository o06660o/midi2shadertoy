use std::fs;

use midly::Smf;

// const MIDI_FILE: &str = "届かない恋.mid";
const MIDI_FILE: &str = "Karma.mid";

fn main() {
    let bytes =
        fs::read(format!("../../test_assets/{}", MIDI_FILE)).expect("failed to read MIDI file");
    let smf = Smf::parse(&bytes).expect("failed to parse MIDI file");

    println!("MIDI file header: {:?}", smf.header);
    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
        for event in track {
            println!("{:?} (delta = {:?})", event.kind, event.delta);
        }
    }
}
