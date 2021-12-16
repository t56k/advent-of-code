use std::time::Instant;

struct Bits {
    bytes: Vec<u8>,
    bit_offset: usize,
}

impl std::fmt::Debug for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bits {{ ")?;
        writeln!(f, "  bit_offset={:} ", self.bit_offset)?;
        writeln!(f, "  bytes=[")?;
        for chunk in self.bytes.chunks(8) {
            write!(f, "    ")?;
            for b in chunk {
                write!(f, "{:>08b} ", b)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "  ]")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl From<&str> for Bits {
    fn from(s: &str) -> Self {
        let bytes = s
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect::<Vec<_>>();
        Self {
            bytes,
            bit_offset: 0,
        }
    }
}

impl Bits {
    fn take<T>(&mut self, n: usize) -> T
    where
        T: std::ops::BitOrAssign
            + std::ops::Shl<Output = T>
            + Default
            + From<u8>
            + std::fmt::Binary,
    {
        let mut result = T::default();
        for i in (0..n).rev() {
            let byte_offset: usize = self.bit_offset / 4;
            let byte: u8 = self.bytes[byte_offset];
            let offset: u8 = 3 - (self.bit_offset % 4) as u8;
            let mask: u8 = 0b1 << offset;
            let bit: u8 = (byte & mask) >> offset;
            result |= T::from(bit) << T::from(i.try_into().unwrap());
            self.bit_offset += 1;
        }
        result
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Max(Vec<Packet>),
    Min(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    Equal(Vec<Packet>),
}

impl From<&mut Bits> for PacketType {
    fn from(bits: &mut Bits) -> Self {
        let type_id = bits.take(3);
        match type_id {
            4 => {
                let literal = Self::parse_literal(bits);
                Self::Literal(literal)
            }
            op_id => {
                let packets = match bits.take::<u8>(1) {
                    0 => Self::parse_subpackets_type0(bits),
                    1 => Self::parse_subpackets_type1(bits),
                    _ => panic!(),
                };
                match op_id {
                    0 => Self::Sum(packets),
                    1 => Self::Product(packets),
                    2 => Self::Min(packets),
                    3 => Self::Max(packets),
                    5 => Self::GreaterThan(packets),
                    6 => Self::LessThan(packets),
                    7 => Self::Equal(packets),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl PacketType {
    #[inline]
    fn parse_literal(bits: &mut Bits) -> usize {
        let mut literal = 0;
        loop {
            let n = bits.take::<usize>(5);
            literal |= n & 0b01111;
            if n & 0b10000 == 0 {
                return literal;
            }
            literal <<= 4;
        }
    }

    fn parse_subpackets_type0(bits: &mut Bits) -> Vec<Packet> {
        let mut sub_length = bits.take::<usize>(15);
        let mut packets = Vec::new();
        while sub_length > 0 {
            let packet = Packet::from(&mut *bits);
            sub_length -= packet.size();
            packets.push(packet);
        }
        packets
    }

    fn parse_subpackets_type1(bits: &mut Bits) -> Vec<Packet> {
        let sub_num = bits.take::<usize>(11);
        let mut packets = Vec::new();
        for _ in 0..sub_num {
            let packet = Packet::from(&mut *bits);
            packets.push(packet);
        }
        packets
    }

    fn eval(&self) -> usize {
        match self {
            PacketType::Literal(n) => *n as usize,
            PacketType::Sum(ps) => ps.iter().map(|p| p.eval()).sum::<usize>(),
            PacketType::Product(ps) => ps.iter().map(|p| p.eval()).product::<usize>(),
            PacketType::Max(ps) => ps.iter().map(|p| p.eval()).max().unwrap(),
            PacketType::Min(ps) => ps.iter().map(|p| p.eval()).min().unwrap(),
            PacketType::GreaterThan(ps) => {
                if ps[0].eval() > ps[1].eval() {
                    1
                } else {
                    0
                }
            }
            PacketType::LessThan(ps) => {
                if ps[0].eval() < ps[1].eval() {
                    1
                } else {
                    0
                }
            }
            PacketType::Equal(ps) => {
                if ps[0].eval() == ps[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_: PacketType,
    size: usize,
}

impl From<&mut Bits> for Packet {
    fn from(bits: &mut Bits) -> Self {
        let version = bits.take(3);
        let start = bits.bit_offset;
        let type_ = PacketType::from(&mut *bits);
        let size = bits.bit_offset - start + 3;
        Self {
            version,
            type_,
            size,
        }
    }
}

impl From<&str> for Packet {
    fn from(hexstr: &str) -> Self {
        let mut iterator = Bits::from(hexstr);
        Self::from(&mut iterator)
    }
}

impl Packet {
    fn eval(&self) -> usize {
        self.type_.eval()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn versions(&self) -> Vec<u8> {
        let mut v = match &self.type_ {
            PacketType::Product(ps)
            | PacketType::Sum(ps)
            | PacketType::GreaterThan(ps)
            | PacketType::LessThan(ps)
            | PacketType::Max(ps)
            | PacketType::Min(ps)
            | PacketType::Equal(ps) => ps.iter().flat_map(|p| p.versions()).collect::<Vec<_>>(),
            PacketType::Literal(_) => vec![],
        };
        v.push(self.version);
        v
    }
    fn sum_versions(&self) -> usize {
        self.versions().iter().map(|d| *d as usize).sum::<usize>()
    }
}

fn main() {
    let input = include_str!("../input");
    let start = Instant::now();

    for line in input.lines() {
        let packet = Packet::from(line.trim());
        println!("part one: {:}", packet.sum_versions());
        println!("part two: {:}", packet.eval());
    }

    let end = start.elapsed();
    println!("time: {:}us", end.as_micros());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut iterator = Bits::from("011");
        assert_eq!(0, iterator.take::<u8>(7));
        assert_eq!(2, iterator.take::<u8>(2));
        assert_eq!(1, iterator.take::<u8>(3));

        let mut iterator = Bits::from("011");
        assert_eq!(17, iterator.take::<usize>(12));
    }

    #[test]
    fn test_versions() {
        assert_eq!(
            16,
            Packet::from("8A004A801A8002F478").sum_versions()
        );
        assert_eq!(
            12,
            Packet::from("620080001611562C8802118E34").sum_versions()
        );
        assert_eq!(
            23,
            Packet::from("C0015000016115A2E0802F182340").sum_versions()
        );
        assert_eq!(
            31,
            Packet::from("A0016C880162017C3686B18A3D4780").sum_versions()
        );
    }
}
