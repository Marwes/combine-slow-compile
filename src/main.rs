enum Either<L, R> {
    Left(L),
    Right(R),
}

macro_rules! choice {
    ($first : expr) => {
        $first
    };
    ($first : expr, $($rest : expr),+) => {
        $first.or(choice!($($rest),+))
    }
}

mod combine {

    use std::marker::PhantomData;

    pub trait Parser<Input: Stream> {
        type Output;

        type PartialState: Default;

        fn or<P2>(self, p: P2) -> Or<Self, P2>
        where
            Self: Sized,
            P2: Parser<Input, Output = Self::Output>,
        {
            or(self, p)
        }

        fn map<F, B>(self, f: F) -> Map<Self, F>
        where
            Self: Sized,
            F: FnMut(Self::Output) -> B,
        {
            map(self, f)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Map<P, F>(P, F);
    impl<Input, A, B, P, F> Parser<Input> for Map<P, F>
    where
        Input: Stream,
        P: Parser<Input, Output = A>,
        F: FnMut(A) -> B,
    {
        type Output = B;
        type PartialState = P::PartialState;
    }

    pub fn map<Input, P, F, B>(p: P, f: F) -> Map<P, F>
    where
        Input: Stream,
        P: Parser<Input>,
        F: FnMut(P::Output) -> B,
    {
        Map(p, f)
    }

    #[macro_export]
    macro_rules! choice {
    ($first : expr) => {
        $first
    };
    ($first : expr, $($rest : expr),+) => {
        $first.or(choice!($($rest),+))
    }
}

    pub trait ChoiceParser<Input: Stream> {
        type Output;
        type PartialState: Default;
    }

    macro_rules! tuple_choice_parser {
    ($head: ident) => {
        tuple_choice_parser_inner!($head; $head);
    };
    ($head: ident $($id: ident)+) => {
        tuple_choice_parser_inner!($head; $head $($id)+);
        tuple_choice_parser!($($id)+);
    };
}

    macro_rules! tuple_choice_parser_inner {
    ($partial_state: ident; $($id: ident)+) => {
        #[doc(hidden)]
        pub enum $partial_state<$($id),+> {
            Peek,
            $(
                $id($id),
            )+
        }

        impl<$($id),+> Default for self::$partial_state<$($id),+> {
            fn default() -> Self {
                self::$partial_state::Peek
            }
        }

        #[allow(non_snake_case)]
        impl<Input, Output $(,$id)+> ChoiceParser<Input> for ($($id,)+)
        where
            Input: Stream,
            $($id: Parser< Input, Output = Output>),+
        {

            type Output = Output;
            type PartialState = self::$partial_state<$($id::PartialState),+>;

        }
    }
}

    tuple_choice_parser!(A B);

    #[derive(Copy, Clone)]
    pub struct Choice<P>(P);

    impl<Input, P> Parser<Input> for Choice<P>
    where
        Input: Stream,
        P: ChoiceParser<Input>,
    {
        type Output = P::Output;
        type PartialState = P::PartialState;
    }

    pub fn choice<Input, P>(ps: P) -> Choice<P>
    where
        Input: Stream,
        P: ChoiceParser<Input>,
    {
        Choice(ps)
    }

    #[derive(Copy, Clone)]
    pub struct Or<P1, P2>(Choice<(P1, P2)>);
    impl<Input, O, P1, P2> Parser<Input> for Or<P1, P2>
    where
        Input: Stream,
        P1: Parser<Input, Output = O>,
        P2: Parser<Input, Output = O>,
    {
        type Output = O;
        type PartialState = <Choice<(P1, P2)> as Parser<Input>>::PartialState;
    }

    pub fn or<Input, P1, P2>(p1: P1, p2: P2) -> Or<P1, P2>
    where
        Input: Stream,
        P1: Parser<Input>,
        P2: Parser<Input, Output = P1::Output>,
    {
        Or(choice((p1, p2)))
    }

    pub struct P<I, O>(PhantomData<fn(I) -> O>);
    impl<Input, O> Parser<Input> for P<Input, O>
    where
        Input: Stream,
    {
        type Output = O;
        type PartialState = ();
    }

    impl<A, B, I> Parser<I> for (A, B)
    where
        A: Parser<I>,
        B: Parser<I>,
        I: Stream,
    {
        type Output = (A::Output, B::Output);
        type PartialState = (A::PartialState, B::PartialState);
    }

    pub trait StreamError<T, R, P> {}
    pub trait ParseError<T, R, P> {}

    pub trait StreamOnce {
        type Token: Clone;
        type Range: Clone;
        type Position: Clone + Ord;
        type Error: ParseError<Self::Token, Self::Range, Self::Position>;
    }

    pub trait RangeStreamOnce: StreamOnce {}

    pub trait Stream: StreamOnce + Clone {}

    impl<T> Stream for T
    where
        T: Clone + StreamOnce,
        T::Error: ParseError<T::Token, T::Range, T::Position>,
    {
    }

    pub trait RangeStream: RangeStreamOnce + Clone {}

    impl<T> RangeStream for T where T: Clone + RangeStreamOnce + Stream {}

    pub fn token<I>(_: I::Token) -> impl Parser<I, Output = I::Token>
    where
        I: Stream,
    {
        P(PhantomData)
    }
}

use combine::token;
use combine::{ParseError, Parser, RangeStream};

fn mcc_payload_item<'a, I: 'a>() -> impl Parser<I, Output = Either<u8, &'static [u8]>>
where
    I: RangeStream<Token = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice!(
        token(b'G').map(|_| Either::Right([0xfau8, 0x00, 0x00].as_ref())),
        token(b'H').map(|_| Either::Right([0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00].as_ref())),
        token(b'I').map(|_| {
            Either::Right([0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00].as_ref())
        }),
        token(b'J').map(|_| {
            Either::Right(
                [
                    0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                ]
                .as_ref(),
            )
        }),
        token(b'K').map(|_| {
            Either::Right(
                [
                    0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                    0x00, 0x00,
                ]
                .as_ref(),
            )
        }),
        token(b'L').map(|_| {
            Either::Right(
                [
                    0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                    0x00, 0x00, 0xfa, 0x00, 0x00,
                ]
                .as_ref(),
            )
        }),
        token(b'M').map(|_| {
            Either::Right(
                [
                    0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                    0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                ]
                .as_ref(),
            )
        }),
        token(b'N').map(|_| {
            Either::Right(
                [
                    0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                    0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                ]
                .as_ref(),
            )
        }),
        token(b'O').map(|_| {
            Either::Right(
                [
                    0xfau8, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                    0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00,
                    0x00,
                ]
                .as_ref(),
            )
        }),
        token(b'P').map(|_| Either::Right([0xfbu8, 0x80, 0x80].as_ref())),
        token(b'Q').map(|_| Either::Right([0xfcu8, 0x80, 0x80].as_ref())),
        token(b'R').map(|_| Either::Right([0xfdu8, 0x80, 0x80].as_ref())),
        token(b'S').map(|_| Either::Right([0x96u8, 0x69].as_ref())),
        token(b'T').map(|_| Either::Right([0x61u8, 0x01].as_ref())),
        token(b'U').map(|_| Either::Right([0xe1u8, 0x00, 0x00].as_ref())),
        token(b'Z').map(|_| Either::Left(0x00u8)),
        token(b'Z').map(|_| Either::Left(0x00u8)),
        token(b'Z').map(|_| Either::Left(0x00u8))
    )
}

fn main() {
    println!("Hello, world!");
}
