use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use ::messages::{MessageDeserializationError, MessageSerializationError};
use ::messages::{RtmpMessage};

pub fn serialize(stream_id: u32) -> Result<Vec<u8>, MessageSerializationError> {
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_u32::<BigEndian>(stream_id)?;

    Ok(cursor.into_inner())
}

pub fn deserialize(data: &[u8]) -> Result<RtmpMessage, MessageDeserializationError> {
    let mut cursor = Cursor::new(data);

    Ok(RtmpMessage::Abort {
        stream_id: cursor.read_u32::<BigEndian>()?
    })
}

#[cfg(test)]
mod tests {
    use super::{serialize, deserialize};
    use std::io::Cursor;
    use byteorder::{BigEndian, WriteBytesExt};

    use ::messages::{RtmpMessage};

    #[test]
    fn can_serialize_message() {
        let id = 523;
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();
        let expected = cursor.into_inner();

        let raw_message = serialize(id).unwrap();

        assert_eq!(raw_message, expected);
    }

    #[test]
    fn can_deserialize_message() {
        let id = 532;
        let expected = RtmpMessage::Abort{stream_id: id};

        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<BigEndian>(id).unwrap();

        let result = deserialize(&cursor.into_inner()[..]).unwrap();
        assert_eq!(result, expected);
    }
}