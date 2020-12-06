# advent2020

My solutions to the series of puzzles at [Advent of Code
2020](http://adventofcode.com/2020).

## Why

Mostly, this is an exercise to teach myself Rust.  (If I was trying to play for
speed, I'd probably stick to python3).  Please don't judge my Rust style too
harshly, I am just getting started.

## Lessons

A miniblog of neat things I've learned:

* I wrote my own `icargo` wrapper to combine iwatch and cargo, but it would
  be great if this was a built-in.  (Maybe I'll look at the cargo source and
  see if I can integrate inotify support for linux, at least.)

* My dev environment is tmux + vim + rust.vim + icargo.  Good enough.

* lifetime annotations are necessary for seemingly simple data types, such as
  `type Foo = HashMap<&str, &str>`.

* Use dbg! instead of println! to provide annotated output.  Sadly, no support
  for verbosity levels or tags.

* lazy_static! macro provides a useful tool when declaring things like regexps
  within functions.

* `HashMap<&str, &str>` works the way I hoped it would (ie. it doesn't do silly
  things like hash on the pointer to the string rather than contents.

* `unwrap()` asserts and unpacks a Result<T> object.

* Rust seems to support both Result<T> and Option<T> for similar purposes.  The
  `Option` template at least provides useful methods like `is_none`.

* `rustfmt` is deprecated but the replacement only works on the unstable branch.
  For now, just `cargo fmt -- -f` is needed to pass the "force" option to the
  underlying deprecated rustfmt.  It worries me that the rust overseers would
  allow rustfmt to be deprecated before the replacement was ready.  How do I
  judge the maturity, stability, and support of any given Rust crate?  Maybe
  Rust needs a LTS subset of core crates for users to depend on.

* Rust is really persnickety about places where I add extra parens for clarity.

* In the example `foo.iter().filter(|x| x.foo())`, the type of `x` is actually
  a reference to the type of `foo`'s iterable data.  Thanks to
  auto-dereferencing, we usually don't care.

* Rust auto-dereferences on method invocation, which is to say that it performs
  a complicated search of the various ways to dereference until it finds a match.
  This is neat, but it means that `x.some_method()` will do roughly what I want
  even if x is a reference, but `x > 0` won't de-reference as Rust references
  have comparison operators.  Further reading:
  https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules

* Geez, they let just anybody submit crates to crate.io.  That's a pretty polluted
  namespace.  How does anyone separate the good libraries from the abandonware?

## See Also

Some friends of mine have their own solutions on github, too:

* [WalrusWalrus](http://github.com/WalrusWalrus/adv2020)

