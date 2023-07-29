use itertools::Itertools;

pub fn day1(content: String) -> anyhow::Result<()> {
    println!("Day 1");
    let groups = content.split('\n').group_by(|x| x.is_empty());
    let value: usize = groups
        .into_iter()
        .filter_map(|(is_empty, x)| {
            if is_empty {
                None
            } else {
                let sum: usize = x.map(|x| x.parse::<usize>().expect("not an int")).sum();
                Some(sum)
            }
        })
        .sorted()
        .rev()
        .take(3)
        .sum();
    println!("Most calories: {}", value);
    println!();
    Ok(())
}
