use anyhow::Result;
use std::io::Write;
use std::net::TcpStream;

// protobuf format, but we do not use protobuf in this project.
// instead we use a custom format
static MAGIC_DELIMITER: [u8; 3] = [0x00, 0x82, 0x01];
static KEEP_ALIVE_MAGIC: [u8; 3] = [0x00, 0x48, 0x01];
pub enum AlsPacket {
    AuthenticateRequest {
        /// Token
        token: String,
    },
    JoinRequest {
        room_id: String,
    },
    KeepAliveRequest {},
}

impl AlsPacket {
    pub fn get_varint(length: u16) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut len = length;
        while len > 0 {
            let part = (len & 0x7f) as u8;
            len >>= 7;
            if len == 0 {
                bytes.push(part);
                break;
            } else {
                bytes.push(part | 0x80);
            }
        }
        bytes
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // 2 bytes length from the start of the packet
        bytes.extend_from_slice(&[0x00, 0x00]);
        bytes.extend_from_slice(&MAGIC_DELIMITER);
        match self {
            AlsPacket::AuthenticateRequest { token } => {
                let varint = Self::get_varint(token.len() as u16);
                bytes.extend_from_slice([varint[0] + 7, varint[1]].as_slice());
                bytes.extend_from_slice(&[0x82, 0x01]);
                bytes.extend_from_slice([varint[0] + 3, varint[1]].as_slice());
                bytes.extend_from_slice(&[0x0a]);
                bytes.extend_from_slice(&varint);
                bytes.extend_from_slice(token.as_bytes());
                // big endian header length
            }
            AlsPacket::JoinRequest { room_id } => {
                let varint = Self::get_varint(room_id.len() as u16);
                // magic
                bytes.extend_from_slice(&[0x31, 0x9a, 0x01, 0x2e]);
                bytes.extend_from_slice(&[0x0a]);
                bytes.extend_from_slice(&varint);
                bytes.extend_from_slice(room_id.as_bytes());
            }
            AlsPacket::KeepAliveRequest {} => {
                bytes.extend_from_slice(&KEEP_ALIVE_MAGIC);
            }
        }
        let length = ((bytes.len() - 2) as u16).to_be_bytes();
        bytes[0] = length[0];
        bytes[1] = length[1];
        bytes
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<()> {
        let bytes = self.to_bytes();
        // send two bytes first, then the rest of the packet
        stream.write_all(&bytes[0..2])?;
        stream.write_all(&bytes[2..])?;
        stream.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protobuf_length() {
        assert_eq!(AlsPacket::get_varint(614), vec![0xe6, 0x04]);
    }

    #[test]
    fn test_packet() {
        let auth_packet = AlsPacket::AuthenticateRequest {
            token: "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJzZXJ2aWNlX2RvbWFpbiI6Imh0dHBzOi8vYXBpLmxpbmstbGlrZS1sb3ZlbGl2ZS5hcHAiLCJsaW5rX2xpa2VfaWQiOiJBQUFBQUFBQUEiLCJyb29tX2lkIjoiZGVmYXVsdC1mYWNiZGE1MS1iYjlkLTQyNjctYjRhYi01ZWYzYzg3OGJhZWMiLCJyb2xlIjoiYXVkaWVuY2UiLCJwb2QiOnsicm9sZSI6ImF1ZGllbmNlIiwic2NoZW1lIjoidGNwIiwiYWRkcmVzcyI6IjEwLjExNC41MTQuMTkxIiwicG9ydCI6OTgxMH0sImlzcyI6Imh0dHBzOi8vYXBpLmxpbmstbGlrZS1sb3ZlbGl2ZS5hcHAiLCJzdWIiOiJBQUFBQUFBQUEiLCJhdWQiOlsiQUFBQUFBQUFBIl0sImV4cCI6MTc0ODUxODU3NSwibmJmIjoxNzQ4NTE4NTYwLCJpYXQiOjE3NDg1MTg1NjB9.eddiZjzEH_I88w9lmOVBr2Z4BWShIv6yeM9TPZvKIts5rmPFwvBbJEKffkobXglOuUBp80svLoufyzOM_YSmDg".to_string(),
        };
        let bytes = auth_packet.to_bytes();
        assert_eq!(bytes, hex::decode("0274008201ef048201eb040ae80465794a68624763694f694a49557a55784d694973496e523563434936496b705856434a392e65794a7a5a584a3261574e6c583252766257467062694936496d68306448427a4f693876595842704c6d7870626d737462476c725a53317362335a6c62476c325a533568634841694c434a73615735725832787061325666615751694f694a425155464251554642515545694c434a796232397458326c6b496a6f695a47566d595856736443316d59574e695a4745314d533169596a6c6b4c5451794e6a6374596a5268596930315a57597a597a67334f474a685a574d694c434a796232786c496a6f695958566b61575675593255694c434a77623251694f6e7369636d39735a534936496d46315a476c6c626d4e6c4969776963324e6f5a57316c496a6f6964474e77496977695957526b636d567a63794936496a45774c6a45784e4334314d5451754d546b784969776963473979644349364f5467784d483073496d6c7a63794936496d68306448427a4f693876595842704c6d7870626d737462476c725a53317362335a6c62476c325a533568634841694c434a7a645749694f694a425155464251554642515545694c434a68645751694f6c7369515546425155464251554642496c3073496d5634634349364d5463304f4455784f4455334e537769626d4a6d496a6f784e7a51344e5445344e5459774c434a70595851694f6a45334e4467314d5467314e6a42392e656464695a6a7a45485f49383877396c6d4f564272325a344257536849763679654d3954505a764b49747335726d5046777642624a454b66666b6f6258676c4f75554270383073764c6f7566797a4f4d5f59536d4467").unwrap());

        let join_packet = AlsPacket::JoinRequest {
            room_id: "default-facbda51-bb9d-4267-b4ab-5ef3c878baec".to_string(),
        };
        let bytes = join_packet.to_bytes();
        assert_eq!(bytes, hex::decode("0035008201319a012e0a2c64656661756c742d66616362646135312d626239642d343236372d623461622d356566336338373862616563").unwrap());
    }
}
