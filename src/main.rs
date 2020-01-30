use either::Either;

use combine;
use combine::parser::byte::hex_digit;
use combine::{choice, token};
use combine::{ParseError, Parser, RangeStream};

fn mcc_payload_item<'a, I: 'a>() -> impl Parser<Input = I, Output = Either<u8, &'static [u8]>>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice!(
        token(b'G').map(|_| Either::Right([0xfau8, 0x00, 0x00].as_ref())),
        token(b'H').map(|_| Either::Right([0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00].as_ref())),
        token(b'I').map(|_| Either::Right(
            [0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00].as_ref()
        )),
        token(b'J').map(|_| Either::Right(
            [0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00].as_ref()
        )),
        token(b'K').map(|_| Either::Right(
            [
                0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00
            ]
            .as_ref()
        )),
        token(b'L').map(|_| Either::Right(
            [
                0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00, 0xfa, 0x00, 0x00
            ]
            .as_ref()
        )),
        token(b'M').map(|_| Either::Right(
            [
                0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00
            ]
            .as_ref()
        )),
        token(b'N').map(|_| Either::Right(
            [
                0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00
            ]
            .as_ref()
        )),
        token(b'O').map(|_| Either::Right(
            [
                0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00
            ]
            .as_ref()
        )),
        token(b'P').map(|_| Either::Right([0xfbu8, 0x80, 0x80].as_ref())),
        token(b'Q').map(|_| Either::Right([0xfcu8, 0x80, 0x80].as_ref())),
        token(b'R').map(|_| Either::Right([0xfdu8, 0x80, 0x80].as_ref())),
        token(b'S').map(|_| Either::Right([0x96u8, 0x69].as_ref())),
        token(b'T').map(|_| Either::Right([0x61u8, 0x01].as_ref())),
        token(b'U').map(|_| Either::Right([0xe1u8, 0x00, 0x00].as_ref())),
        token(b'Z').map(|_| Either::Left(0x00u8)),
        (hex_digit(), hex_digit()).map(|(u, l)| {
            let hex_to_u8 = |v: u8| match v {
                v if v >= b'0' && v <= b'9' => v - b'0',
                v if v >= b'A' && v <= b'F' => 10 + v - b'A',
                v if v >= b'a' && v <= b'f' => 10 + v - b'a',
                _ => unreachable!(),
            };
            let val = (hex_to_u8(u) << 4) | hex_to_u8(l);
            Either::Left(val)
        })
    )
    .message("while parsing MCC payload")
}

fn main() {
    println!("Hello, world!");
}
