mod bit_reader;
use bit_reader::BitReader;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Literal {
        version: u8,
        number: u64,
    },
    Operator {
        version: u8,
        operator: Operator,
        packets: Vec<Packet>,
    },
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
            let operator = match type_id {
                0 => Operator::Sum,
                1 => Operator::Product,
                2 => Operator::Minimum,
                3 => Operator::Maximum,
                5 => Operator::GreaterThan,
                6 => Operator::LessThan,
                7 => Operator::EqualTo,
                _ => unreachable!(),
            };

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
                    Packet::Operator {
                        version,
                        operator,
                        packets,
                    }
                }
                1 => {
                    // 11 bits = a number that represents the number of sub-packets immediately contained
                    let sub_packets = reader.read_bits_u16(11);
                    Packet::Operator {
                        version,
                        operator,
                        packets: (0..sub_packets).map(|_| parse(reader)).collect(),
                    }
                }
                _ => panic!("not a valid length_type_id, got {}", length_type_id),
            }
        }
    }
}

fn calculate(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version: _, number } => *number,
        Packet::Operator {
            version: _,
            operator: Operator::Sum,
            packets,
        } => packets.iter().fold(0, |acc, curr| acc + calculate(curr)),
        Packet::Operator {
            version: _,
            operator: Operator::Product,
            packets,
        } => packets.iter().fold(1, |acc, curr| acc * calculate(curr)),
        Packet::Operator {
            version: _,
            operator: Operator::Minimum,
            packets,
        } => packets.iter().map(|p| calculate(p)).min().unwrap(),
        Packet::Operator {
            version: _,
            operator: Operator::Maximum,
            packets,
        } => packets.iter().map(|p| calculate(p)).max().unwrap(),
        Packet::Operator {
            version: _,
            operator: Operator::LessThan,
            packets,
        } => {
            if calculate(&packets[0]) < calculate(&packets[1]) {
                return 1;
            }
            {
                return 0;
            }
        }
        Packet::Operator {
            version: _,
            operator: Operator::GreaterThan,
            packets,
        } => {
            if calculate(&packets[0]) > calculate(&packets[1]) {
                return 1;
            }
            {
                return 0;
            }
        }
        Packet::Operator {
            version: _,
            operator: Operator::EqualTo,
            packets,
        } => {
            if calculate(&packets[0]) == calculate(&packets[1]) {
                return 1;
            }
            {
                return 0;
            }
        }
        _ => unimplemented!(),
    }
}

fn main() {
    let content = include_str!("../input");
    let binary = binary_vec(content);
    let mut reader = BitReader::new(&binary);
    let expr = parse(&mut reader);
    println!("{:?}", calculate(&expr));
}

#[cfg(test)]
mod tests {
    use crate::{binary_vec, calculate, parse, BitReader, Operator, Packet};

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
                operator: Operator::LessThan,
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

    #[test]
    fn test_find_sum() {
        let input = "C200B40A82";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        let packet = parse(&mut reader);
        assert_eq!(3, calculate(&packet));
    }

    #[test]
    fn test_find_product() {
        let input = "04005AC33890";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        let packet = parse(&mut reader);
        assert_eq!(54, calculate(&packet));
    }

    #[test]
    fn test_find_minimum() {
        let input = "880086C3E88112";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        let packet = parse(&mut reader);
        assert_eq!(7, calculate(&packet));
    }

    #[test]
    fn test_find_maximum() {
        let input = "CE00C43D881120";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        let packet = parse(&mut reader);
        assert_eq!(9, calculate(&packet));
    }

    #[test]
    fn test_less_than() {
        let input = "D8005AC2A8F0";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        let packet = parse(&mut reader);
        assert_eq!(1, calculate(&packet));
    }

    #[test]
    fn test_greater_than() {
        let input = "F600BC2D8F";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        let packet = parse(&mut reader);
        assert_eq!(0, calculate(&packet));
    }

    #[test]
    fn test_equal_to() {
        let input = "9C005AC2F8F0";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        let packet = parse(&mut reader);
        assert_eq!(0, calculate(&packet));
    }

    #[test]
    fn test_equal_to_the_same() {
        let input = "9C0141080250320F1802104A08";
        let bytes = binary_vec(&input);
        let mut reader = BitReader::new(&bytes);
        let packet = parse(&mut reader);
        assert_eq!(1, calculate(&packet));
    }
}
