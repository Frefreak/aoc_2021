macro_rules! SP {
    ($p1:expr, $p2:expr) => {
        Snailfish::Pair(Box::new($p1), Box::new($p2))
    };
}

macro_rules! SN {
    ($p1:expr) => {
        Snailfish::Num($p1)
    };
}
