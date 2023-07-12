# Calorie Counting: The Three Scale Menace

I'm back baby! Who knew I did sequels? Not this guy. \*points at self\*

Anyhoo, Santa's elf managers came back and now they want to know the top three elves carrying the most calories. I'm really trying to make sure this doesn't sound like we're eating the elves. How am I doing?

Since we already have most of our code written, this should be a cake walk. Let's get coding.

So we know we want the sum of the calories carried by the top three calorie carrying elves. We already managed to get the sums of all the items carried by each elf, so let's go back and start there. 

```rs
let calories_carried_by_elf = include_str!("../inputs/day_01.txt")
    .split("\n\n")
    .map(|collection| {
        collection
            .lines()
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
    });
```

Cool. Nothing new here. We just removed some code and did a bit of renaming. In the developer world we call this a productive day.

What do we need now? We have the sum of calories carried by each elf, so the next step would be to sort the iterable that we have. Maybe rust has a sort...

No. There's no sort method for an iterable in rust. How sad.

But wait! What about a `Vec`? You know, that special collection thing that rust has. We used it before when trying to visualize the mess that split created in Day 01a. Let's try.

```rs
let calories_carried_by_elf = include_str!("../inputs/day_01.txt")
    .split("\n\n")
    .map(|collection| {
        collection
            .lines()
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
    })
    .collect::<Vec<_>>();

calories_carried_by_elf.sort();
```

Ya love to see it! We have sort! But what the heck are these pesky red squiggly lines under my sort? And why didn't I just call sort at the end of my call chain for `calories_carried_by_elf`.

Let's start with the easier one. 

`sort()` doesn't return anything. It just sorts the elements in an iterable so the next time you use the iterable it will have been sorted. This is called mutation. And this leads us to the squigglies.

If you try to build the code `cargo build` rust will throw something like this at you

> cannot borrow `calories_carried_by_elf` as mutable, as it is not declared as mutable

Bless you and your error messages, rust.

By default all variables are immutable in rust. This means that they cannot be changed. Since we are changing our iterable, we have to make it mutable somehow.

Rust makes this easy by adding the `mut` keyword before our variable name. Observe.

```rs
let mut calories_carried_by_elf = include_str!("../inputs/day_01.txt")
    .split("\n\n")
    .map(|collection| {
        collection
            .lines()
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
    })
    .collect::<Vec<_>>();

calories_carried_by_elf.sort();
```

Oh yeah! We're back in business!

But did you notice rust doesn't have just one sort? What the heck are these other sorts? `sort_unstable`, `sort_by`, `sort_unstable_by`...

Well `unstable` sorts don't trouble themselves with sorting values that are equal to one another. Rust just doesn't care in this case, and this gives us a little extra performance boost. Sorting something with `by` allows us to define how we want rust to sort our list. We can use the `cmp()` method here to compare one element to another.

Since we just want the top three calorie carries, we don't really care about ordering equal values. If they're in the top three, they're in the top three. That's all we care about. So let's try sorting the values from lowest to highest.

```rs
calories_carried_by_elf.sort_unstable_by(|a, b| a.cmp(b));
```

Look at that beautiful `closure`. Did you know it can take more than one argument? Well you do now!

This is sorting our vec from lowest to highest, but we probably want the top three calory carries at the front of our vec. Could we just...

```rs
calories_carried_by_elf.sort_unstable_by(|a, b| b.cmp(a));
```

Rust FTW!

Just swap the `b` and the `a` with the `cmp()` and we now have a reverse sorted vec.

What else do we need? We just need the sum of the first three elements in our vec. We know that an iterable has a `sum()` method in rust, but does vec?

>`Vec<u32>` is not an iterator
>the following trait bounds were not satisfied:
>`Vec<u32>: Iterator`
>which is required by `&mut Vec<u32>: Iterator`
>`[u32]: Iterator`
>which is required by `&mut [u32]: Iterator`

Head. Desk.

Why?

Not that it would help us in the first place since we want to get the sum of the first three elements and now the sum of our entire vec.

So what to do?

Well, it looks like we have to covert our vec into an iterable, take the first three items, and then call sum on those items? It feels weird that we have to covert our iterable to a vec with `collect` then covert our vec back into an iterable with `into_iter`. But that's just the name of the game. Also, I'm a total noob at rust.

Let's try.

```rs
let sum_of_top_three = calories_carried_by_elf.into_iter().take(3).sum::<u32>();
```

Success!

We coverted our calory vec back into an iter, took the first three elements with the handy `take()` method, and used our familiar `sum()` method to find the sum of the top three. Surely, Santa will be pleased.

Day one done!
