// use prost::encode_length_delimiter()

pub mod proto {
    pub mod alstromeria {
        include!(concat!(env!("OUT_DIR"), "/als.rs"));
    }
}