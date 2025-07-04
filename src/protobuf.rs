/// A wire type as seen on the wire.
enum WireType {
    /// The Varint WireType indicates the value is a single VARINT.
    Varint,
    // The I64 WireType indicates that the value is precisely 8 bytes in
    // little-endian order containing a 64-bit signed integer or double type.
    //I64,  -- not needed for this exercise
    /// The Len WireType indicates that the value is a length represented as a
    /// VARINT followed by exactly that number of bytes.
    Len,
    // The I32 WireType indicates that the value is precisely 4 bytes in
    // little-endian order containing a 32-bit signed integer or float type.
    //I32,  -- not needed for this exercise
}

#[derive(Debug)]
/// A field's value, typed based on the wire type.
enum FieldValue<'a> {
    Varint(u64),
    //I64(i64),  -- not needed for this exercise
    Len(&'a [u8]),
    //I32(i32),  -- not needed for this exercise
}

#[derive(Debug)]
/// A field, containing the field number and its value.
pub struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

pub trait ProtoMessage<'a>: Default {
    fn add_field(&mut self, field: Field<'a>);
}

impl From<u64> for WireType {
    fn from(value: u64) -> Self {
        match value {
            0 => WireType::Varint,
            //1 => WireType::I64,  -- not needed for this exercise
            2 => WireType::Len,
            //5 => WireType::I32,  -- not needed for this exercise
            _ => panic!("Invalid wire type: {value}"),
        }
    }
}

impl<'a> FieldValue<'a> {
    fn as_str(&self) -> &'a str {
        let FieldValue::Len(data) = self else {
            panic!("Expected string to be a `Len` field");
        };
        std::str::from_utf8(data).expect("Invalid string")
    }

    fn as_bytes(&self) -> &'a [u8] {
        let FieldValue::Len(data) = self else {
            panic!("Expected bytes to be a `Len` field");
        };
        data
    }

    fn as_u64(&self) -> u64 {
        let FieldValue::Varint(value) = self else {
            panic!("Expected `u64` to be a `Varint` field");
        };
        *value
    }
}

/// Parse a VARINT, returning the parsed value and the remaining bytes.
fn parse_varint(data: &[u8]) -> (u64, &[u8]) {
    for i in 0..7 {
        let Some(b) = data.get(i) else {
            panic!("Not enough bytes for varint");
        };
        if b & 0x80 == 0 {
            // This is the last byte of the VARINT, so convert it to
            // a u64 and return it.
            let mut value = 0u64;
            for b in data[..=i].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return (value, &data[i + 1..]);
        }
    }

    // More than 7 bytes is invalid.
    panic!("Too many bytes for varint");
}

/// Convert a tag into a field number and a WireType.
fn unpack_tag(tag: u64) -> (u64, WireType) {
    let field_num = tag >> 3;
    let wire_type = WireType::from(tag & 0x7);
    (field_num, wire_type)
}

/// Parse a field, returning the remaining bytes
fn parse_field(data: &[u8]) -> (Field, &[u8]) {
    let (tag, remainder) = parse_varint(data);
    let (field_num, wire_type) = unpack_tag(tag);
    let (fieldvalue, remainder) = match wire_type {
        WireType::Varint => {
            let (value, rest) = parse_varint(remainder);
            (FieldValue::Varint(value), rest)
        }
        WireType::Len => {
            let (len, rest) = parse_varint(remainder);
            let len = len as usize;
            (FieldValue::Len(&rest[..len]), &rest[len..])
        }
    };
    (
        Field {
            field_num,
            value: fieldvalue,
        },
        remainder,
    )
}

/// Parse a message in the given data, calling `T::add_field` for each field in
/// the message.
///
/// The entire input is consumed.
pub fn parse_message<'a, T: ProtoMessage<'a>>(mut data: &'a [u8]) -> T {
    let mut result = T::default();
    while !data.is_empty() {
        let parsed = parse_field(data);
        result.add_field(parsed.0);
        data = parsed.1;
    }
    result
}

#[derive(Debug, Default, PartialEq)]
pub struct PhoneNumber<'a> {
    number: &'a str,
    type_: &'a str,
}

#[derive(Debug, Default, PartialEq)]
pub struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}

impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        let Field { field_num, value } = field;
        match field_num {
            1 => self.number = value.as_str(),
            2 => self.type_ = value.as_str(),
            _ => unreachable!(),
        }
    }
}

impl<'a> ProtoMessage<'a> for Person<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        let Field { field_num, value } = field;
        match field_num {
            1 => self.name = value.as_str(),
            2 => self.id = value.as_u64(),
            3 => self.phone.push(parse_message(value.as_bytes())),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let person_id: Person = parse_message(&[0x10, 0x2a]);
        assert_eq!(
            person_id,
            Person {
                name: "",
                id: 42,
                phone: vec![]
            }
        );
    }

    #[test]
    fn test_name() {
        let person_name: Person = parse_message(&[
            0x0a, 0x0e, 0x62, 0x65, 0x61, 0x75, 0x74, 0x69, 0x66, 0x75, 0x6c, 0x20, 0x6e, 0x61,
            0x6d, 0x65,
        ]);
        assert_eq!(
            person_name,
            Person {
                name: "beautiful name",
                id: 0,
                phone: vec![]
            }
        );
    }

    #[test]
    fn test_just_person() {
        let person_name_id: Person =
            parse_message(&[0x0a, 0x04, 0x45, 0x76, 0x61, 0x6e, 0x10, 0x16]);
        assert_eq!(
            person_name_id,
            Person {
                name: "Evan",
                id: 22,
                phone: vec![]
            }
        );
    }

    #[test]
    fn test_phone() {
        let phone: Person = parse_message(&[
            0x0a, 0x00, 0x10, 0x00, 0x1a, 0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x33, 0x34, 0x2d,
            0x37, 0x37, 0x37, 0x2d, 0x39, 0x30, 0x39, 0x30, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65,
        ]);
        assert_eq!(
            phone,
            Person {
                name: "",
                id: 0,
                phone: vec![PhoneNumber {
                    number: "+1234-777-9090",
                    type_: "home"
                },],
            }
        );
    }

    // Put that all together into a single parse.
    #[test]
    fn test_full_person() {
        let person: Person = parse_message(&[
            0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a, 0x16, 0x0a,
            0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35, 0x2d, 0x31, 0x32, 0x31,
            0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a, 0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38,
            0x30, 0x30, 0x2d, 0x38, 0x36, 0x37, 0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d,
            0x6f, 0x62, 0x69, 0x6c, 0x65,
        ]);
        assert_eq!(
            person,
            Person {
                name: "maxwell",
                id: 42,
                phone: vec![
                    PhoneNumber {
                        number: "+1202-555-1212",
                        type_: "home"
                    },
                    PhoneNumber {
                        number: "+1800-867-5308",
                        type_: "mobile"
                    },
                ]
            }
        );
    }
}
