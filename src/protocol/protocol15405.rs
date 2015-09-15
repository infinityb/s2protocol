use std::io::{self};

use phf::Map as PhfMap;

use super::{
    Protocol,
    InitData,
    ReplayDetails,
    ReplayGameEvents,
    ReplayMessageEvents,
    ReplayTrackerEvents,
    ReplayAttributesEvents,
    TypeInfo,
};

use super::protocol15405_def::{
    GAME_EVENTID_TYPEID,
    GAME_EVENT_TYPES,
    MESSAGE_EVENTID_TYPEID,
    MESSAGE_EVENT_TYPES,
    TYPEINFOS,
};

pub struct Protocol15405;

impl Protocol for Protocol15405 {
    fn protocol_num(&self) -> u32 { 15405 }

    fn decode_replay_initdata(&self, rdr: &mut io::Read) -> io::Result<InitData> {
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
