pub fn day_one() {
    let mut sums = include_str!("../inputs/day_01.txt")
        .split("\n\n")
        .map(|collection| {
            collection
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    sums.sort_unstable_by(|a, b| b.cmp(a));

    println!("{:#?}", sums.into_iter().take(3).sum::<u32>())
}