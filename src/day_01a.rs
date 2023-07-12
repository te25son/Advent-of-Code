pub fn day_one() {
    let result = include_str!("../inputs/day_01.txt")
        .split("\n\n")
        .map(|collection| {
            collection
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("{}", result)
}
