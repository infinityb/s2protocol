#![allow(dead_code)]
#![feature(slice_bytes, plugin)]
#![plugin(phf_macros)]

extern crate byteorder;
extern crate phf;
extern crate phf_macros;


mod mpq;
pub mod protocol;
pub use self::protocol::get_protocol;

#[test]
fn it_works() {
	assert_eq!(get_protocol(15405).unwrap().protocol_num(), 15405);

	let proto = get_protocol(15405).unwrap();
	proto.decode_replay_header()
}

