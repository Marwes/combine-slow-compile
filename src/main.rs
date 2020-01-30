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

    pub trait Parser<Input> {
        type Output;

        fn or<P2>(self, p: P2) -> Or<Self, P2>
        where
            Self: Sized,
            P2: Parser<Input, Output = Self::Output>,
        {
            or(self, p)
        }
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

    #[derive(Copy, Clone)]
    pub struct Choice<P>(P);

    impl<Input, P> Parser<Input> for Choice<P> {
        type Output = u8;
    }

    pub fn choice<Input, P>(ps: P) -> Choice<P> {
        Choice(ps)
    }

    #[derive(Copy, Clone)]
    pub struct Or<P1, P2>(Choice<(P1, P2)>);
    impl<Input, O, P1, P2> Parser<Input> for Or<P1, P2>
    where
        P1: Parser<Input, Output = O>,
        P2: Parser<Input, Output = O>,
    {
        type Output = O;
    }

    pub fn or<Input, P1, P2>(p1: P1, p2: P2) -> Or<P1, P2>
    where
        P1: Parser<Input>,
        P2: Parser<Input, Output = P1::Output>,
    {
        Or(choice((p1, p2)))
    }

    pub struct P<I, O>(PhantomData<fn(I) -> O>);
    impl<Input, O> Parser<Input> for P<Input, O> {
        type Output = O;
    }

    impl<A, B, I> Parser<I> for (A, B)
    where
        A: Parser<I>,
        B: Parser<I>,
    {
        type Output = (A::Output, B::Output);
    }

    pub fn token<I>(_: u8) -> impl Parser<I, Output = u8> {
        P(PhantomData)
    }
}

use combine::token;
use combine::Parser;

fn mcc_payload_item<I>() -> impl Parser<I, Output = u8> {
    choice!(
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G'),
        token(b'G')
    )
}

fn main() {
    println!("Hello, world!");
}
