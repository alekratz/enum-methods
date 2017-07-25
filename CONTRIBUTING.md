# Contributing

All contributions are welcome. If there is a feature you would like to see,
please make an issue for it and we can discuss it.

If there's an issue that isn't obviously claimed by someone, feel free to work
on it and submit a PR. Work-in-progress PRs are allowed, but if you go inactive
your PR will probably be forgotten and/or deleted.

# Code style

* Spaces only; no tabs.
* Remember to run [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt). If
  you don't, it'll probably be okay unless your style is blatantly different
  from that of the project.
* Write tests for testable stuff.
* Use descriptive function names. A lot of these functions only get called once
  or twice and a long name isn't the end of the world.

# Commenting

* Try to comment non-obvious or unintuitive things going on in the code.
* Describe the *why*, not the *how*. The code should "speak for itself" in how
  it works, and the comment should explain its rationale if it's not obvious.
  [This blog post is a good explanation of what I like to see.](https://blog.codinghorror.com/code-tells-you-how-comments-tell-you-why/)

