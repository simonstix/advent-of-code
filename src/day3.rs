use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    compartments: Vec<HashSet<char>>,
}

impl Rucksack {
    fn new_two_compartment(content: &str) -> Self {
        let size = content.len() / 2;
        let compartments = content.chars().chunks(size);
        let compartments = compartments.into_iter().map(|x| x.collect()).collect();
        Self { compartments }
    }

    fn find_duplicates(&self) -> HashSet<char> {
        self.compartments
            .iter()
            .fold(all_letters(), |set, compartment| {
                set.intersection(compartment).copied().collect()
            })
    }

    fn all_items(&self) -> HashSet<char> {
        self.compartments
            .iter()
            .fold(HashSet::new(), |set, compartment| {
                set.union(compartment).copied().collect()
            })
    }

    fn find_duplicate_priority(&self) -> usize {
        let duplicates = self.find_duplicates();
        duplicates.iter().copied().map(letter_priority).sum()
    }
}

pub fn day3(content: String) -> anyhow::Result<()> {
    println!("Day 3");
    println!("Part 1");
    let rucksacks = content
        .lines()
        .map(Rucksack::new_two_compartment)
        .collect_vec();
    let priority_sum: usize = rucksacks.iter().map(|x| x.find_duplicate_priority()).sum();
    println!("Priority sum: {}", priority_sum);

    println!();
    println!("Part 2");
    let chunks = rucksacks.iter().chunks(3);
    let badges = chunks.into_iter().map(find_badge).collect_vec();
    let group_priorities: usize = badges.iter().copied().map(letter_priority).sum();
    println!("Group priorities: {}", group_priorities);

    println!();
    Ok(())
}

/// Find the badge in a single group of elfs
fn find_badge<'a>(elfs: impl IntoIterator<Item = &'a Rucksack>) -> char {
    let set = elfs.into_iter().fold(all_letters(), |set, elf| {
        set.intersection(&elf.all_items()).copied().collect()
    });
    assert_eq!(set.len(), 1);
    set.into_iter().next().unwrap()
}

fn all_letters() -> HashSet<char> {
    let mut set = HashSet::new();
    set.extend('a'..='z');
    set.extend('A'..='Z');
    set
}

fn letter_priority(letter: char) -> usize {
    if letter.is_ascii_lowercase() {
        return letter as usize - 'a' as usize + 1;
    }
    if letter.is_ascii_uppercase() {
        return letter as usize - 'A' as usize + 27;
    }

    panic!("unsupported letter {}", letter);
}
