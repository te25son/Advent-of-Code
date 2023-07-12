# Calorie Counting: The Scale Menace

I'm back baby! Who knew I did sequels? Not this guy. \*points at self\*

Anyhoo, Santa's elf managers came back and now they want to know the top three elves carrying the most calories. I'm really trying to make sure this doesn't sound like we're eating the elves. How am I doing?

Since we already have most of our code written, this should be a cake walk. Let's get coding.

So we know we want the sum of the calories carried by the top three calorie carrying elves. We already managed to get the sums of all the items carried by each elf, so let's go back and start there. 

```rs
let result = include_str!("../inputs/day_01.txt")
        .split("\n\n")
        .map(|collection| {
            collection
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        });
```
