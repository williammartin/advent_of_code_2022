use crate::day07::{Input, Output};

pub fn solve(fs: &Input) -> Output {
    fs.dirs()
        .filter_map(|dir_idx| {
            let dir_size = fs.size_of(dir_idx);
            if dir_size < 100000 {
                Some(dir_size)
            } else {
                None
            }
        })
        .sum::<u32>()
        .into()
}
