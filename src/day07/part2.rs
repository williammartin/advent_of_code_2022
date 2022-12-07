use crate::day07::{Input, Output};

pub fn solve(fs: &Input) -> Output {
    let required_space = 30000000 - (70000000 - fs.size_of(super::NodeIndex(0)));

    fs.dirs()
        .filter_map(|dir_idx| {
            let dir_size = fs.size_of(dir_idx);
            if dir_size >= required_space {
                Some(dir_size)
            } else {
                None
            }
        })
        .min()
        .expect("to have at least one directory with enough space")
        .into()
}
