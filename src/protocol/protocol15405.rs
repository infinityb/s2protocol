use std::io::{self};

use phf::Map as PhfMap;

use super::{
    Protocol,
    ReplayHeader,
    ReplayInitData,
    ReplayDetails,
    ReplayGameEvents,
    ReplayMessageEvents,
    ReplayTrackerEvents,
    ReplayAttributesEvents,
    TypeInfo,
    VersionedDecoder,
    BitPackedDecoder,
};

use super::protocol15405_def::{
    REPLAY_HEADER_TYPEID,
    GAME_EVENTID_TYPEID,
    GAME_EVENT_TYPES,
    MESSAGE_EVENTID_TYPEID,
    MESSAGE_EVENT_TYPES,
    TYPEINFOS,
};

pub struct Protocol15405;

impl Protocol for Protocol15405 {
    fn protocol_num(&self) -> u32 { 15405 }

    fn decode_replay_header(&self, rdr: &mut io::Read) -> io::Result<ReplayHeader> {
        unimplemented!();
        // let mut decoder = VersionedDecoder::new(rdr, TYPEINFOS);
        // let x = decoder.read_instance(REPLAY_HEADER_TYPEID);
    }

    fn decode_replay_initdata(&self, rdr: &mut io::Read) -> io::Result<ReplayInitData> {
        unimplemented!();
    }

    fn decode_replay_details(&self, rdr: &mut io::Read) -> io::Result<ReplayDetails> {
        unimplemented!();
    }

    fn decode_replay_game_events(&self, rdr: &mut io::Read) -> io::Result<ReplayGameEvents> {
        unimplemented!();
    }

    fn decode_replay_message_events(&self, rdr: &mut io::Read) -> io::Result<ReplayMessageEvents> {
        unimplemented!();
    }

    fn decode_replay_tracker_events(&self, rdr: &mut io::Read) -> io::Result<ReplayTrackerEvents> {
        unimplemented!();
    }

    fn decode_replay_attributes_events(&self, rdr: &mut io::Read) -> io::Result<ReplayAttributesEvents> {
        unimplemented!();
    }
}


// def decode_replay_header(contents):
//     """Decodes and return the replay header from the contents byte string."""
//     decoder = VersionedDecoder(contents, typeinfos)
//     return decoder.instance(replay_header_typeid)