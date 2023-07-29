use itertools::Itertools;

fn is_distinct(markers: &[char]) -> bool {
    markers.iter().duplicates().next().is_none()
}

fn find_start_of_packet(content: &str, length: usize) -> (String, usize) {
    let content = content.chars().collect_vec();
    let (index, group) = content
        .windows(length)
        .enumerate()
        .find(|(_, markers)| is_distinct(markers))
        .expect("no marker found");

    (group.iter().collect(), index + length)
}

pub fn day6(content: String) {
    println!();
    println!("==== Day 6 ====");

    println!("Part 1");
    let (marker, end_of_marker) = find_start_of_packet(&content, 4);
    println!("Marker: {:?} Packet start: {}", marker, end_of_marker);

    println!("Part 2");
    let (_, start_of_message) = find_start_of_packet(&content, 14);
    println!("Start of message: {}", start_of_message);
}

#[cfg(test)]
mod tests {
    use crate::day6::find_start_of_packet;

    #[test]
    fn test_find_marker() {
        let values = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (value, target_repeated) in values {
            let (_, first_repeated) = find_start_of_packet(value, 4);
            assert_eq!(first_repeated, target_repeated);
        }
    }

    #[test]
    fn test_find_start_of_package() {
        let values = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (value, target_repeated) in values {
            let (_, first_repeated) = find_start_of_packet(value, 14);
            assert_eq!(first_repeated, target_repeated);
        }
    }
}
