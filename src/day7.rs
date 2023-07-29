use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
enum File<'a> {
    File { size: usize },
    Directory { files: HashMap<&'a str, File<'a>> },
}

impl<'a> File<'a> {
    fn new_dir() -> Self {
        Self::Directory {
            files: Default::default(),
        }
    }

    fn new_file(size: usize) -> Self {
        Self::File { size }
    }

    fn extract_filesystem(&mut self, command_stack: &mut Vec<&'a str>) {
        let Self::Directory {files} = self else {
            panic!();
        };

        while let Some(command) = command_stack.pop() {
            match &command[0..4] {
                "$ cd" => {
                    let name = &command[5..];

                    if name == ".." {
                        return;
                    }

                    let sub_dir = files.get_mut(name).unwrap();
                    sub_dir.extract_filesystem(command_stack);
                }
                "$ ls" => {
                    while let Some(value) = command_stack.last() {
                        if value.starts_with('$') {
                            break;
                        }

                        let file = command_stack.pop().unwrap();

                        if file.starts_with("dir") {
                            let name = &file[4..];
                            let dir = Self::new_dir();
                            files.insert(name, dir);
                        } else {
                            let (size, name): (&str, &str) =
                                file.split_whitespace().collect_tuple().unwrap();
                            let size = size
                                .parse::<usize>()
                                .unwrap_or_else(|_| panic!("{} could not be parsed to int", size));

                            files.insert(name, File::new_file(size));
                        }
                    }
                }
                command => panic!("unknown command {}", command),
            }
        }
    }

    fn size(&self) -> usize {
        match self {
            File::File { size } => *size,
            File::Directory { files } => files.iter().map(|(_, x)| x.size()).sum(),
        }
    }

    fn visit_dirs(&self, name: &str, func: &mut dyn FnMut(&str, &HashMap<&str, File>)) {
        match self {
            File::File { .. } => {
                // Do nothing
            }
            File::Directory { files } => {
                func(name, files);

                for (name, file) in files {
                    file.visit_dirs(name, func);
                }
            }
        }
    }
}

fn sum_dirs_with_max_size(file: &File, name: &str, max_size: usize) -> usize {
    let mut sum = 0;
    file.visit_dirs(name, &mut |_, dir| {
        let size: usize = dir.iter().map(|(_, file)| file.size()).sum();
        if size <= max_size {
            sum += size;
        }
    });
    sum
}

fn smallest_dir_with_enough_space(file: &File, name: &str, min_size: usize) -> usize {
    let mut list = vec![];
    file.visit_dirs(name, &mut |_, dir| {
        // code duplication...
        let size: usize = dir.iter().map(|(_, file)| file.size()).sum();

        if size >= min_size {
            list.push(size);
        }
    });

    *list.iter().min().unwrap()
}

pub fn day7(content: String) {
    println!();
    println!("==== Day 7 ====");

    let mut command_stack = content.lines().rev().collect_vec();
    let enter_root = command_stack.pop().unwrap();
    assert_eq!(enter_root, "$ cd /");

    let mut root = File::new_dir();
    root.extract_filesystem(&mut command_stack);
    let root = root;

    println!("Part 1");
    let file_size_sum = sum_dirs_with_max_size(&root, "/", 100000);
    println!("Sum of Dir sizes below 10000: {}", file_size_sum);

    println!("Part 2");
    let current_free_space = 70000000 - root.size();
    let min_delete_size = 30000000 - current_free_space;
    let smallest_file_to_delete = smallest_dir_with_enough_space(&root, "/", min_delete_size);
    println!("Deleted file size: {}", smallest_file_to_delete);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let example = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        let mut command_stack = example.lines().rev().collect_vec();
        let _ = command_stack.pop().unwrap();

        let mut root = File::new_dir();
        root.extract_filesystem(&mut command_stack);
        let root = root;

        let file_size_sum = sum_dirs_with_max_size(&root, "/", 100000);
        assert_eq!(file_size_sum, 95437);
    }

    #[test]
    fn test_example_part_2() {
        let example = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        let mut command_stack = example.lines().rev().collect_vec();
        let _ = command_stack.pop().unwrap();

        let mut root = File::new_dir();
        root.extract_filesystem(&mut command_stack);
        let root = root;

        let current_free_space = 70000000 - root.size();
        let min_delete_size = 30000000 - current_free_space;
        let smallest_file_to_delete = smallest_dir_with_enough_space(&root, "/", min_delete_size);
        assert_eq!(smallest_file_to_delete, 24933642);
    }
}
