use crate::day06::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let marker = char_windows(input, 4)
        .enumerate()
        .find(|(i, substring)| {
            let mut chars = substring.chars().collect::<Vec<_>>();
            chars.sort(); // lol
            chars.dedup(); // lolllll

            chars.len() == 4
        })
        .expect("to find a marker");

    (marker.0 + 4).into() // hahahahaha just add 4 cuz
}

fn char_windows<'a>(src: &'a str, win_size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}
