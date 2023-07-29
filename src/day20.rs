use itertools::Itertools;

fn parse_list(content: &str) -> Vec<i32> {
    content.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn mix_list(list: &mut Vec<i32>) {
    #[cfg(test)]
    println!("{:?}", list);
    let mut order = list.iter().cloned().rev().collect_vec();

    while let Some(num) = order.pop() {
        let (index, _) = list
            .iter()
            .find_position(|x| **x == num)
            .expect("number not found");
        move_in_direction(list, index, num);
        #[cfg(test)]
        println!("{:?}", list);
    }
}

fn jump_in_direction(list: &mut Vec<i32>, mut from: usize, dir: i32) {
    let value = list.remove(from);

    let mut from = from as i32;

    let mut to = (from + dir).rem_euclid(list.len() as i32);

    list.insert(to as usize, value);
}

fn move_in_direction(list: &mut Vec<i32>, mut from: usize, mut dir: i32) {
    while dir != 0 {
        // Mathematical modulo with list length
        let mut to = (from as i32 + dir.signum()).rem_euclid(list.len() as i32) as usize;
        dir -= dir.signum();

        list.swap(from, to);

        from = to;
    }
}

fn calc_coordinates(list: &[i32]) -> i32 {
    let (zero_pos, _) = list.iter().find_position(|x| **x == 0).unwrap();
    let first = *list
        .get((zero_pos + 1000usize).rem_euclid(list.len()))
        .unwrap();
    let second = *list
        .get((zero_pos + 2000usize).rem_euclid(list.len()))
        .unwrap();
    let third = *list
        .get((zero_pos + 3000usize).rem_euclid(list.len()))
        .unwrap();
    return first + second + third;
}

pub fn day20(content: String) {
    println!();
    println!("==== Day 20 ====");
    let mut list = parse_list(&content);

    println!("Part 1");
    mix_list(&mut list);
    println!("Coordinates: {}", calc_coordinates(&list));

    println!();
    println!("Part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"1
2
-3
3
-2
0
4"#;

    #[test]
    fn test_part_1() {
        let mut list = parse_list(EXAMPLE);
        mix_list(&mut list);
        println!("{:?}", list);
        // assert_eq!(&list, &[1, 2, -3, 4, 0, 3, -2]);
        assert_eq!(calc_coordinates(&list), 3);
    }

    #[test]
    fn test_part_2() {}
}
