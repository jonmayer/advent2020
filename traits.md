# Commonly Used Rust Traits

TODO: mark which traits are derive-able and which aren't.

## Core Traits

| Trait | Description | Derivable |
| --- | --- | --- |
| [Display][display] | Enables println! | No |
| [Debug][debug] | Enables dbg! | If all fields implement Debug |
| [Default][default] | Gives type a default value. | If all fields implement Default |
| [From][from] | Enables value-to-value conversion. | No |
| [FromStr][fromstr] | Enables `s.parse::<Foo>`. | No. |
| [Clone][clone] | Implements a method for object duplication. | If all fields implement Clone. |
| [Copy][copy] | Marks type as memcpy-able. | If all fields are marked Copy. |
| [Borrow][borrow] | Defines a reference type (`&str`) for a storage type (`String`). | No. |
| [Read][read] | Read from I/O stream. | No. |
| [Write][write] | Write to I/O stream. | No. |
| [Error][error] | Can be used as an error. | Yes if impl Display and Debug. |
| [Eq][eq] | Equality ([note 1](#eq)) | If all fields implement Eq. |
| [PartialEq][partialeq] | Partial Equality ([note 1](#eq)) | If all fields implement Eq or PartialEq. |
| [Ord][ord] | |  |
| [PartialOrd][partialord] | |  |

### Notes

#### Note 1

`Eq` and `PartialEq` provide slightly different promises.  `PartialEq`
guarantees symmetric (`a == b` implies `b == a`) and transitive (if `a == b`
and `b == c` then `a == c`) properties, but not the reflexive (`a == a`)
property.  `Eq` adds the reflexive property guarantee.

## Iterator Traits

| Trait | Your type is ... |
| \-\-\- | \-\-\- |
| [Iterator][iterator] | an iterator. |
| [IntoIterator][intoiterator] | something iterable, ie `foo.iter()`. |
| [Sum][sum] | an iterator that supports `foo.sum()`. |

Iterable objects should support three flavors of the [IntoIterator][intoiterator] trait:
move, shared reference, and mutable reference.   See [this blog
post](https://www.philipdaniels.com/blog/2019/rust-api-design2/) for details.

## Collection Traits

Collections should implement [FromIterator][fromiterator] and [Extend][extend].

| Trait | Your type is ... |
| \-\-\- | \-\-\- |
| [FromIterator][fromiterator] | a collection that works with `iter.collect()`. |
| [Extend][extend] | a collection that supports `foo.extend(iterable)`. |

## Mathy Traits

Operations from https://doc.rust-lang.org/core/ops/\#traits .

| Trait | Associated operation |
| \-\-\- | \-\-\- |
| [Add][add] | The addition operator +. |
| [AddAssign][addassign] | The addition assignment operator +=. |
| [BitAnd][bitand] | The bitwise AND operator &. |
| [BitAndAssign][bitandassign] | The bitwise AND assignment operator &=. |
| [BitOr][bitor] | The bitwise OR operator |. |
| [BitOrAssign][bitorassign] | The bitwise OR assignment operator |=. |
| [BitXor][bitxor] | The bitwise XOR operator ^. |
| [BitXorAssign][bitxorassign] | The bitwise XOR assignment operator ^=. |
| [Deref][deref] | Used for immutable dereferencing operations, like \*v. |
| [DerefMut][derefmut] | Used for mutable dereferencing operations, like in \*v = 1;. |
| [Div][div] | The division operator /. |
| [DivAssign][divassign] | The division assignment operator /=. |
| [Drop][drop] | Custom code within the destructor. |
| [Fn][fn] | The version of the call operator that takes an immutable receiver. |
| [FnMut][fnmut] | The version of the call operator that takes a mutable receiver. |
| [FnOnce][fnonce] | The version of the call operator that takes a by-value receiver. |
| [Index][index] | Used for indexing operations (container[index][index]) in immutable contexts. |
| [IndexMut][indexmut] | Used for indexing operations (container[index][index]) in mutable contexts. |
| [Mul][mul] | The multiplication operator \*. |
| [MulAssign][mulassign] | The multiplication assignment operator \*=. |
| [Neg][neg] | The unary negation operator -. |
| [Not][not] | The unary logical negation operator !. |
| [RangeBounds][rangebounds] | RangeBounds is implemented by Rust's built-in range types, produced by range syntax like .., a.., ..b, ..=c, d..e, or f..=g. |
| [Rem][rem] | The remainder operator %. |
| [RemAssign][remassign] | The remainder assignment operator %=. |
| [Shl][shl] | The left shift operator \<\<. |
| [ShlAssign][shlassign] | The left shift assignment operator \<\<=. |
| [Shr][shr] | The right shift operator >>. |
| [ShrAssign][shrassign] | The right shift assignment operator >>=. |
| [Sub][sub] | The subtraction operator -. |
| [SubAssign][subassign] | The subtraction assignment operator -=. |

## Less Common Traits

| Trait | Your type is ... |
| \-\-\- | \-\-\- |
| [DoubleEndedIterator][doubleendediterator] | An [Iterator][iterator] that can be iterated forwards or backwards. |
| [ExactSizeIterator][exactsizeiterator] | An [Iterator][iterator] with an efficient `len` method. |
| [FusedIterator][fusediterator] | An iterator that always continues to yield None when exhausted. |
| [Product][product] | Trait to represent types that can be created by multiplying elements of an iterator. |
| [Sum][sum] | Trait to represent types that can be created by summing up an iterator. |

## References

- [The Rust Book 10.2: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [The Common Rust Traits](https://stevedonovan.github.io/rustifications/2018/09/08/common-rust-traits.html)

[add]: https://doc.rust-lang.org/std/ops/trait.Add.html
[addassign]: https://doc.rust-lang.org/std/ops/trait.AddAssign.html
[bitand]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
[bitandassign]: https://doc.rust-lang.org/std/ops/trait.BitAndAssign.html
[bitor]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
[bitorassign]: https://doc.rust-lang.org/std/ops/trait.BitOrAssign.html
[bitxor]: https://doc.rust-lang.org/std/ops/trait.BitXor.html
[bitxorassign]: https://doc.rust-lang.org/std/ops/trait.BitXorAssign.html
[borrow]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[default]: https://doc.rust-lang.org/std/default/index.html
[deref]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[derefmut]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[div]: https://doc.rust-lang.org/std/ops/trait.Div.html
[divassign]: https://doc.rust-lang.org/std/ops/trait.DivAssign.html
[doubleendediterator]: https://doc.rust-lang.org/core/iter/trait.DoubleEndedIterator.html
[drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[error]: https://doc.rust-lang.org/std/error/trait.Error.html
[exactsizeiterator]: https://doc.rust-lang.org/core/iter/trait.ExactSizeIterator.html
[extend]: https://doc.rust-lang.org/core/iter/trait.Extend.html
[fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnonce]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
[from]: https://doc.rust-lang.org/std/convert/trait.From.html
[fromiterator]: https://doc.rust-lang.org/core/iter/trait.FromIterator.html
[fromstr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[fusediterator]: https://doc.rust-lang.org/core/iter/trait.FusedIterator.html
[index]: https://doc.rust-lang.org/std/ops/trait.Index.html
[indexmut]: https://doc.rust-lang.org/std/ops/trait.IndexMut.html
[intoiterator]: https://doc.rust-lang.org/core/iter/trait.IntoIterator.html
[iterator]: https://doc.rust-lang.org/core/iter/trait.Iterator.html
[mul]: https://doc.rust-lang.org/std/ops/trait.Mul.html
[mulassign]: https://doc.rust-lang.org/std/ops/trait.MulAssign.html
[neg]: https://doc.rust-lang.org/std/ops/trait.Neg.html
[not]: https://doc.rust-lang.org/std/ops/trait.Not.html
[ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[partialeq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[partialord]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[product]: https://doc.rust-lang.org/core/iter/trait.Product.html
[rangebounds]: https://doc.rust-lang.org/std/ops/trait.RangeBounds.html
[read]: https://doc.rust-lang.org/std/io/trait.Read.html
[rem]: https://doc.rust-lang.org/std/ops/trait.Rem.html
[remassign]: https://doc.rust-lang.org/std/ops/trait.RemAssign.html
[shl]: https://doc.rust-lang.org/std/ops/trait.Shl.html
[shlassign]: https://doc.rust-lang.org/std/ops/trait.ShlAssign.html
[shr]: https://doc.rust-lang.org/std/ops/trait.Shr.html
[shrassign]: https://doc.rust-lang.org/std/ops/trait.ShrAssign.html
[sub]: https://doc.rust-lang.org/std/ops/trait.Sub.html
[subassign]: https://doc.rust-lang.org/std/ops/trait.SubAssign.html
[sum]: https://doc.rust-lang.org/core/iter/trait.Sum.html
[write]: https://doc.rust-lang.org/std/io/trait.Write.html
