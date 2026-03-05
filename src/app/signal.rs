#[derive(Copy, Clone, Debug)]
pub enum Signal {
    Dot,
    Dash,
}

pub fn signals_to_char(signals: &[Signal]) -> Option<char> {
    use Signal::{Dash, Dot};

    match signals {
        [Dot, Dash] => Some('a'),
        [Dash, Dot, Dot, Dot] => Some('b'),
        [Dash, Dot, Dash, Dot] => Some('c'),
        [Dash, Dot, Dot] => Some('d'),
        [Dot] => Some('e'),
        [Dot, Dot, Dash, Dot] => Some('f'),
        [Dash, Dash, Dot] => Some('g'),
        [Dot, Dot, Dot, Dot] => Some('h'),
        [Dot, Dot] => Some('i'),
        [Dot, Dash, Dash, Dash] => Some('j'),
        [Dash, Dot, Dash] => Some('k'),
        [Dot, Dash, Dot, Dot] => Some('l'),
        [Dash, Dash] => Some('m'),
        [Dash, Dot] => Some('n'),
        [Dash, Dash, Dash] => Some('o'),
        [Dot, Dash, Dash, Dot] => Some('p'),
        [Dash, Dash, Dot, Dash] => Some('q'),
        [Dot, Dash, Dot] => Some('r'),
        [Dot, Dot, Dot] => Some('s'),
        [Dash] => Some('t'),
        [Dot, Dot, Dash] => Some('u'),
        [Dot, Dot, Dot, Dash] => Some('v'),
        [Dot, Dash, Dash] => Some('w'),
        [Dash, Dot, Dot, Dash] => Some('x'),
        [Dash, Dot, Dash, Dash] => Some('y'),
        [Dash, Dash, Dot, Dot] => Some('z'),
        _ => None,
    }
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Dot => ".",
            Self::Dash => "-",
        };

        write!(f, "{c}")
    }
}
