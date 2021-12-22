#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Literal { version: u8, number: u64 },
    Operator { version: u8, packets: Vec<Packet> },
}

struct BitReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> BitReader<'a> {
        BitReader { bytes, pos: 0 }
    }

    pub fn has_next(&self) -> bool {
        self.pos < self.bytes.len()
    }

    pub fn read_bit(&mut self) -> u8 {
        let pos = self.pos;
        self.pos += 1;
        self.bytes[pos]
    }

    pub fn read_bits(&mut self, count: usize) -> &'a [u8] {
        let pos = self.pos..(self.pos + count);
        self.pos += count;
        &self.bytes[pos]
    }

    pub fn read_bits_u8(&mut self, count: usize) -> u8 {
        if count > 8 {
            panic!("cannot read more than 8 bits in a byte");
        }
        let read = self.read_bits(count);
        (0..count).fold(0, |mut accum, curr| {
            let b = read[curr] << (read.len() - curr - 1);
            accum = accum | b;
            accum
        })
    }

    pub fn read_bits_u16(&mut self, count: usize) -> u16 {
        if count > 16 {
            panic!("cannot read more than 16 bits in a word");
        }
        let read = self.read_bits(count);
        (0..count).fold(0, |mut accum, curr| {
            let b = (read[curr] as u16) << (read.len() - curr - 1);
            accum = accum | b;
            accum
        })
    }
}

fn hex_to_bin(byte: &u8) -> Vec<u8> {
    let bytes = match byte {
        b'0' => "0000",
        b'1' => "0001",
        b'2' => "0010",
        b'3' => "0011",
        b'4' => "0100",
        b'5' => "0101",
        b'6' => "0110",
        b'7' => "0111",
        b'8' => "1000",
        b'9' => "1001",
        b'A' => "1010",
        b'B' => "1011",
        b'C' => "1100",
        b'D' => "1101",
        b'E' => "1110",
        b'F' => "1111",
        _ => panic!("unknown char"),
    };
    bytes.as_bytes().iter().map(|b| b - b'0').collect()
}

/// Convert hexadecimal string to vector
/// where every bit is represented as u8
fn binary_vec(hex: &str) -> Vec<u8> {
    hex.as_bytes().iter().flat_map(|b| hex_to_bin(&b)).collect()
}

/// Read an expression and return the AST
fn parse(reader: &mut BitReader) -> Packet {
    let (version, type_id) = (reader.read_bits_u8(3), reader.read_bits_u8(3));
    match type_id {
        4 => {
            // Literal value
            let mut number: u64 = 0;
            let mut i = 0;
            while reader.has_next() {
                let next = reader.read_bit();
                number = number << 4;
                number = number | reader.read_bits_u8(4) as u64;
                if next == 0 {
                    break;
                }
                // Safety guard because I think we might
                // get something nasty in some assignment
                i += 1;
                if i == 16 {
                    panic!("number won't fit u64, use something like BigNum");
                }
            }
            Packet::Literal { version, number }
        }
        _ => {
            // Operator
            let length_type_id = reader.read_bit();
            match length_type_id {
                0 => {
                    // 15 bits = total length in bits of the sub-packets contained by this packet
                    let length = reader.read_bits_u16(15);
                    let chunk = reader.read_bits(length as usize);
                    let mut children = BitReader::new(&chunk);
                    let mut packets = Vec::new();
                    while children.has_next() {
                        let packet = parse(&mut children);
                        packets.push(packet);
                    }
                    Packet::Operator { version, packets }
                }
                1 => {
                    // 11 bits = a number that represents the number of sub-packets immediately contained
                    let sub_packets = reader.read_bits_u16(11);
                    Packet::Operator {
                        version,
                        packets: (0..sub_packets).map(|_| parse(reader)).collect(),
                    }
                }
                _ => panic!("not a valid length_type_id, got {}", length_type_id),
            }
        }
    }
}

fn sum_version(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version, number: _ } => *version as u64,
        Packet::Operator { version, packets } => {
            *version as u64 + packets.iter().fold(0, |acc, curr| acc + sum_version(curr))
        }
    }
}

fn main() {
    let content = include_str!("../input");
    let binary = binary_vec(content);
    let mut reader = BitReader::new(&binary);
    let expr = parse(&mut reader);
    println!("{:?}", sum_version(&expr));
}

#[cfg(test)]
mod tests {
    use crate::{binary_vec, parse, BitReader, Packet};

    #[test]
    fn test_convert_to_binary_vec() {
        let input = "D2FE28";
        let bin = binary_vec(input);
        assert_eq!(
            "110100101111111000101000"
                .as_bytes()
                .to_vec()
                .iter()
                .map(|c| c - b'0')
                .collect::<Vec<_>>(),
            bin
        );
    }

    #[test]
    fn test_parse_header() {
        let input = "D2FE28";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        assert_eq!(6, *&reader.read_bits_u8(3));
        assert_eq!(4, *&reader.read_bits_u8(3));
    }

    #[test]
    fn test_parse_literal() {
        let input = "D2FE28";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        assert_eq!(
            Packet::Literal {
                version: 6,
                number: 2021
            },
            parse(&mut reader)
        );
    }

    #[test]
    fn test_parse_operator() {
        let input = "38006F45291200";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        assert_eq!(
            Packet::Operator {
                version: 1,
                packets: vec![
                    Packet::Literal {
                        version: 6,
                        number: 10
                    },
                    Packet::Literal {
                        version: 2,
                        number: 20
                    }
                ]
            },
            parse(&mut reader)
        );
    }
}
