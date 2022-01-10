use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::{fold_many0, length_count, length_value, many1},
    sequence::{preceded, tuple},
    IResult,
};
use nom_bitvec::BSlice;

type Transmission = BitBox<Msb0, u8>;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Transmission {
    input
        .trim()
        .chars()
        .fold(BitVec::with_capacity(input.len() * 4), |mut bits, a| {
            let a = a.to_digit(16).unwrap() as u8;
            bits.extend_from_bitslice(&[a].view_bits::<Msb0>()[4..]);
            bits
        })
        .into_boxed_bitslice()
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

type Literal = u64;

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: Literal,
    },
    Operator {
        version: u8,
        op: Op,
        operands: Vec<Packet>,
    },
}

fn parse_packet(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Packet> {
    fn to_u8(s: BSlice<Msb0, u8>) -> u8 {
        s.0.load_be::<u8>()
    }

    fn to_usize(s: BSlice<Msb0, u8>) -> usize {
        s.0.load_be::<usize>()
    }

    fn parse_literal(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Packet> {
        let (input, (version, _)) = tuple((
            map(take(3_usize), to_u8),
            tag(BSlice(bits![static 1, 0, 0])),
        ))(input)?;

        let (input, mut value) = fold_many0(
            preceded(tag(BSlice(bits![static 1])), map(take(4_usize), to_u8)),
            || 0,
            |acc, n| acc << 4 | Literal::from(n),
        )(input)?;

        let (input, n) = preceded(tag(BSlice(bits![static 0])), map(take(4_usize), to_u8))(input)?;

        value = (value << 4) | Literal::from(n);

        Ok((input, Packet::Literal { version, value }))
    }

    fn parse_operator(input: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Packet> {
        let (input, (version, type_id)) =
            tuple((map(take(3_usize), to_u8), map(take(3_usize), to_u8)))(input)?;

        let (input, length_type_id) = map(take(1_usize), to_u8)(input)?;

        let (input, operands) = if length_type_id == 0 {
            length_value(map(take(15_usize), to_usize), many1(parse_packet))(input)
        } else {
            length_count(map(take(11_usize), to_usize), parse_packet)(input)
        }?;

        let op = match type_id {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Minimum,
            3 => Op::Maximum,
            5 => Op::GreaterThan,
            6 => Op::LessThan,
            7 => Op::EqualTo,
            _ => unreachable!(),
        };

        Ok((
            input,
            Packet::Operator {
                version,
                op,
                operands,
            },
        ))
    }

    alt((parse_literal, parse_operator))(input)
}

fn sum_packet_versions(packet: &Packet) -> u16 {
    match packet {
        Packet::Literal { version, .. } => u16::from(*version),
        Packet::Operator {
            version, operands, ..
        } => u16::from(*version) + operands.iter().map(sum_packet_versions).sum::<u16>(),
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Transmission) -> u16 {
    let (_, root) = parse_packet(BSlice(input.as_bitslice())).unwrap();
    sum_packet_versions(&root)
}

fn eval(packet: &Packet) -> Literal {
    match packet {
        Packet::Literal { value, .. } => *value,
        Packet::Operator {
            op: Op::Sum,
            operands,
            ..
        } => operands.iter().map(eval).sum(),
        Packet::Operator {
            op: Op::Product,
            operands,
            ..
        } => operands.iter().map(eval).product(),
        Packet::Operator {
            op: Op::Minimum,
            operands,
            ..
        } => operands.iter().map(eval).min().unwrap(),
        Packet::Operator {
            op: Op::Maximum,
            operands,
            ..
        } => operands.iter().map(eval).max().unwrap(),
        Packet::Operator {
            op: Op::GreaterThan,
            operands,
            ..
        } => Literal::from(eval(&operands[0]) > eval(&operands[1])),
        Packet::Operator {
            op: Op::LessThan,
            operands,
            ..
        } => Literal::from(eval(&operands[0]) < eval(&operands[1])),
        Packet::Operator {
            op: Op::EqualTo,
            operands,
            ..
        } => Literal::from(eval(&operands[0]) == eval(&operands[1])),
    }
}

#[aoc(day16, part2)]
pub fn part2(input: &Transmission) -> Literal {
    let (_, root) = parse_packet(BSlice(input.as_bitslice())).unwrap();
    eval(&root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator("8A004A801A8002F478")), 16);
        assert_eq!(part1(&input_generator("620080001611562C8802118E34")), 12);
        assert_eq!(part1(&input_generator("C0015000016115A2E0802F182340")), 23);
        assert_eq!(
            part1(&input_generator("A0016C880162017C3686B18A3D4780")),
            31
        );
    }

    #[test]
    fn solution1() {
        assert_eq!(
            part1(&input_generator(include_str!("../input/2021/day16.txt"))),
            945,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator("C200B40A82")), 3);
        assert_eq!(part2(&input_generator("04005AC33890")), 54);
        assert_eq!(part2(&input_generator("880086C3E88112")), 7);
        assert_eq!(part2(&input_generator("CE00C43D881120")), 9);
        assert_eq!(part2(&input_generator("D8005AC2A8F0")), 1);
        assert_eq!(part2(&input_generator("F600BC2D8F")), 0);
        assert_eq!(part2(&input_generator("9C005AC2F8F0")), 0);
        assert_eq!(part2(&input_generator("9C0141080250320F1802104A08")), 1);
    }

    #[test]
    fn solution2() {
        assert_eq!(
            part2(&input_generator(include_str!("../input/2021/day16.txt"))),
            10_637_009_915_279,
        );
    }
}
