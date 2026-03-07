#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Signal {
    Dot,
    Dash,
}

pub fn alphanumeric() -> [(Vec<Signal>, char); 36] {
    use Signal::{Dash, Dot};

    [
        (vec![Dot, Dash], 'a'),
        (vec![Dash, Dot, Dot, Dot], 'b'),
        (vec![Dash, Dot, Dash, Dot], 'c'),
        (vec![Dash, Dot, Dot], 'd'),
        (vec![Dot], 'e'),
        (vec![Dot, Dot, Dash, Dot], 'f'),
        (vec![Dash, Dash, Dot], 'g'),
        (vec![Dot, Dot, Dot, Dot], 'h'),
        (vec![Dot, Dot], 'i'),
        (vec![Dot, Dash, Dash, Dash], 'j'),
        (vec![Dash, Dot, Dash], 'k'),
        (vec![Dot, Dash, Dot, Dot], 'l'),
        (vec![Dash, Dash], 'm'),
        (vec![Dash, Dot], 'n'),
        (vec![Dash, Dash, Dash], 'o'),
        (vec![Dot, Dash, Dash, Dot], 'p'),
        (vec![Dash, Dash, Dot, Dash], 'q'),
        (vec![Dot, Dash, Dot], 'r'),
        (vec![Dot, Dot, Dot], 's'),
        (vec![Dash], 't'),
        (vec![Dot, Dot, Dash], 'u'),
        (vec![Dot, Dot, Dot, Dash], 'v'),
        (vec![Dot, Dash, Dash], 'w'),
        (vec![Dash, Dot, Dot, Dash], 'x'),
        (vec![Dash, Dot, Dash, Dash], 'y'),
        (vec![Dash, Dash, Dot, Dot], 'z'),
        (vec![Dot, Dash, Dash, Dash, Dash], '1'),
        (vec![Dot, Dot, Dash, Dash, Dash], '2'),
        (vec![Dot, Dot, Dot, Dash, Dash], '3'),
        (vec![Dot, Dot, Dot, Dot, Dash], '4'),
        (vec![Dot, Dot, Dot, Dot, Dot], '5'),
        (vec![Dash, Dot, Dot, Dot, Dot], '6'),
        (vec![Dash, Dash, Dot, Dot, Dot], '7'),
        (vec![Dash, Dash, Dash, Dot, Dot], '8'),
        (vec![Dash, Dash, Dash, Dash, Dot], '9'),
        (vec![Dash, Dash, Dash, Dash, Dash], '0'),
    ]
}

pub fn signals_to_str(signals: &[Signal]) -> String {
    let mut buf = String::new();

    for s in signals {
        buf.push_str(&s.to_string());
    }

    buf
}

pub fn char_to_signals(char: char) -> Option<Vec<Signal>> {
    alphanumeric()
        .iter()
        .find(|(_s, c)| *c == char)
        .map(|(s, _)| s)
        .cloned()
}

pub fn signals_to_char(signals: &[Signal]) -> Option<char> {
    alphanumeric()
        .iter()
        .find(|(s, _c)| s.iter().eq(signals.iter()))
        .map(|(_, c)| *c)
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
