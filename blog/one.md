# Calorie Counting

Let's get this show on the road. We want to find the elf with the largest set of calories. That sounds like we're going to eat the elf, but no we just want him to share his food with us.

Anyway, the first step to solving this problem is to ingest our input somehow. Our input is in a txt file that look like this:

```
1000
2000

3000

4000
```

Rust comes with a builtin macro called `include_str` that takes the relative path of the file we want to read and returns the contents as a `&str`.

Boom.

The next step is to split the string representation of our txt file by newlines. Now, every line in the txt file ends with an invisible `\n` character. So we can imagine our txt file actually looks like this:

```
1000\n
2000\n
\n
3000\n
\n
4000\n
```

When this file is read by `include_str` the output becomes:

`1000\n2000\n\n3000\n\n4000\n`

Since a blank line represents the end of one elf's items and the beginning of another elf's items, we don't need to split the string on a single newline but on two newlines together.

Lucky for us, `&str` has a handy method called `split` that we can use. Let's use it.

```rs
let result = include_str!("../inputs/elf_list.txt").split("\n\n");
```

So we're splitting the elves' list on double newlines. This will return a `Split` that will give us a headache if we try to print it. Just try having rust print `result` if you don't believe me.

A better way to visualize the mess that split created would be to use `collect`. Try comparing the output of result above with the output of result below.

```rs
let result = include_str!("../inputs/elf_list.txt").split("\n\n").collect::<Vec<_>>();
```

Nothing like a good turbofish (::<>) to get your day started.

But we're good!

We have successfully split our list into groups of items belonging to each elf.

Hooray!

Now, we want to find some way to transform the contents of split into something else. Hmmm... if only there was something... I got it! We can use `map`!

According to the rust docs, `map`...

> transforms one iterator into another, by means of its argument: something that implements [FnMut]. It produces a new iterator which calls this closure on each element of the original iterator.
>
> If you are good at thinking in types, you can think of map() like this: If you have an iterator that gives you elements of some type A, and you want an iterator of some other type B, you can use map(), passing a closure that takes an A and returns a B.

There are some scary words in there - I'm looking at you `closure` - but otherwise this seems to do exactly what we want it to do. Let's use it!

Also let's assume we know a couple of things. We know what a closure is. Check. We know that `split` returns an iterator that contains all the substrings separated by the character(s) we provided. Check. We know we want to trasform our substrings into the sum of all the items in each result. Check. We know that we have to convert each item from a string to some kind of integer so that we can do some quick math on it. Check.

Let's code!

```rs
let result = include_str!("../inputs/elf_list.txt")
                .split("\n\n")
                .map(|collection| {
                    collection
                        .lines()
                        .map(|item| item.parse::<u32>().unwrap())
                        .sum::<u32>()
                });
```

Whoa!

What did we learn besides the fact that map is freaking dope? Well, we learned that we can transform the contents of an iterable using `map` and this `closure` thing. 

We learned that `&str`'s in rust have a handy `line` function that we can use to create a new iteratable. `line` splits the string by a single newline. How useful! 

We also learned that we can parse strings into other types (using that turbofish thing again). In this case, we parse each item in an elf's collection into a 32-bit unsigned integer, effectionately known as a `u32`. However, we learned that `parse` returns a `result` that we need to `unwrap` first.

We really should have assumed we knew what `result`s were earlier too. My apologies.

Finally, we learned that we can use `sum` on an iterable to get the sum of all the items in that iterable.

Ain't rust grand?

So, so far we have successfully read the contents of a file to a string. We've split that string into a iterable of substrings. Then we've iterated through our collection of substrings, transforming each substring into a sum of all the items in that substring.

Now what?

Easy.

All we need to do is find the max value in our iterable of sums. Wouldn't it be great if rust had some way to do this for us? Oh wait!

Rust has a `max` function specifically for this purpose.

```rs
let result = include_str!("../inputs/elf_list.txt")
                .split("\n\n")
                .map(|collection| {
                    collection
                        .lines()
                        .map(|item| item.parse::<u32>().unwrap())
                        .sum::<u32>()
                })
                .max();
```

Wait a minute. There's that `result` thing again. You know what we have to do.

```rs
let result = include_str!("../inputs/elf_list.txt")
                .split("\n\n")
                .map(|collection| {
                    collection
                        .lines()
                        .map(|item| item.parse::<u32>().unwrap())
                        .sum::<u32>()
                })
                .max()
                .unwrap();
```

Presto magico!

We can now find the max amount of calories an elf has. That still sounds like we're gonna eat the elf. Words, what can I say.

Grad yourself a snack. We've earned it.
