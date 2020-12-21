# advent2020

My solutions to the series of puzzles at [Advent of Code
2020](http://adventofcode.com/2020).

## Why

Mostly, this is an exercise to teach myself Rust.  (If I was trying to play for
speed, I'd probably stick to python3).  Please don't judge my Rust style too
harshly, I am just getting started.

## Lessons

A miniblog of Rust quirks and lessons that I've learned.

# 2020-12-04

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

# 2020-12-05

* In the example `foo.iter().filter(|x| x.foo())`, the type of `x` is actually
  a reference to the type of `foo`'s iterable data.  Thanks to
  auto-dereferencing, we usually don't care.

* Rust auto-dereferences on method invocation, which is to say that it performs
  a complicated search of the various ways to dereference until it finds a match.
  This is neat, but it means that `x.some_method()` will do roughly what I want
  even if x is a reference, but `x > 0` won't de-reference as Rust references
  have comparison operators.  Further reading:
  https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules

### 2020-12-06

* Unsigned subtraction panics on underflow.  Preusably, all operators panic on
  overflow or underflow.  An explicit `u32.wrapped_sub` is necessary to get the
  usual 2's complement behavior.  I wonder if there's a way to disable this
  behavior in favor of computational efficiency?

* Geez, they let just anybody submit crates to crate.io.  That's a pretty polluted
  namespace.  How does anyone separate the good libraries from the abandonware?

* Rust does not allow static members of structs.  I wonder how Rust implements
  singletons?

* `const` and `static` are not the same in Rust.

* Structs can not contain `const` nor `static` data.  Rust's pattern is to
  move class-static data to a method, but this precludes the ability to
  use a size constant when declaring fixed-size arrays within the struct.

### 2020-12-07

*Day 7*: I went on a little detour involving Rust lifetimes.  My big idea was to build
an object that contained a large text string, and then mark up that string
with a `HashMap<&str, &str>` where both the key and the values would be
references into the large non-mutable String.  This did not work, by I had to
unpeel two layers of the onion to understand why:

1. The first thing I learned what that I had created an situation where Rust
   could not correctly infer lifetimes.  I decided that all references into
   the text String should have the lifetime of the String itself, and managed
   that by explicitly specifying lifetimes.  This turned into explicitly
   specifying lifetimes for every function, and every reference, which was a
   fair amount of typing but I ultimately got that part to compile.

2. The second thing I learned is that Rust simply disallows what I'm trying
   to do in the first place.  That is: A Rust struct cannot have a value
   and a reference to that value in the same struct.  For an explanation,
   https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct

At this point, I had two options: I could start labeling my references as
unsafe, and just be careful not to clone or move my object, or rewrite the data
structure to create owned copies of all string snippets instead of references.
I did the latter, which produced a quick and easy implementation that also
thrashes the heap quite severely.  This would have been straightforward and
memory efficient in C++, but Rust's memory paranoia made me choose between
safety and inefficiency.

I keep feeling that there has to be a way to make my initial approach work.  I
could replace my string references with start and stop indexes?  Or, perhaps
there is simply a better way to construct my object system.  When I have time,
I'll get back to this and see if I can still fix it.

### 2020-12-12

I'm all caught up!  And then I went back and rewrote *Day 12* to see how
difficult it would be to dispatch commands via function pointers and table
lookup instead of a match statement.

I learned that the type for a method pointer is similar to `fn(&mut Class, i32,
i32) -> i32`.  I used the `lazy_static` macro to initialize my HashMap of
commands onto method pointers, and that felt a bit ugly but it works properly.
Maybe Rust will add a better way to initialize maps in the future?

### 2020-12-13

*Day 13* taught me: if I want a performant brute-force solution, I should
remember to turn compiler optimizations on.  `cargo --release run`.  I ended up
replacing my brute force solution with a more mathy solution that completes in
a blink, but it's possible my BF solution would have been good enough if I had
remembered that Rust compiles in debug mode by default.

### 2020-12-14

*Day 14* I wrote my first Iterator type.

### 2020-12-17

I've started to build a catalog of commonly used [traits](traits.md).

### 2020-12-21

*Day 17* I implemented a bitvector to represent the "pocket universe", but others
used a HashSet to represent the sparse data.  I decided to benchmark the two
versions:

| Implementation | Benchmark |
| --- | --- |
| BitVector | 8.2491 ms |
| HashSet | 140.07 ms |

Yipes!  It looks like the default hasher for std::collections is Sip, which is
very performant.  I tried some other hashing algorithms, too:

| Implementation | Benchmark | Command |
| --- | --- | --- |
| bit vector | 8.2491 ms | `cargo bench` |
| Default HashSet | 140.07 ms | `cargo bench --features "hashset hash-default"` |
| twox-hash HashSet | 134.41 ms | `cargo bench --features "hashset hash-xx"` |
| fxhash HashSet | 144.60 ms | `cargo bench --features "hashset hash-fx"` |
| ahash HashSet | 70.951 ms | `cargo bench --features "hashset hash-a"` |

## See Also

Some friends of mine have their own solutions on github, too:

* [WalrusCodes](http://github.com/WalrusCodes/adv2020)

