# Commonly Used Rust Traits

TODO: mark which traits are derive-able and which aren't.

## Core Traits

| Trait | Description | Derivable |
| --- | --- |
| [Display] | Enables println! | No |
| [Debug] | Enables dbg! | If all fields implement Debug |
| [Default] | Gives type a default value. | If all fields implement Default |
| [From] | Enables value-to-value conversion. | No |
| [FromStr] | Enables `s.parse::<Foo>`. | No. |
| [Clone] | Implements a method for object duplication. | If all fields implement Clone. |
| [Copy] | Marks type as memcpy-able. | If all fields are marked Copy. |
| [Borrow] | Defines a reference type (`&str`) for a storage type (`String`). | No. |
| [Read] | Read from I/O stream. | No. |
| [Write] | Write to I/O stream. | No. |
| [Error] | Can be used as an error. | Yes if impl Display and Debug. |
| [Eq] | Equality ([note 1](#eq)) | If all fields implement Eq. |
| [PartialEq] | Partial Equality ([note 1](#eq)) | If all fields implement Eq or PartialEq. |
| [Ord] | |  |
| [PartialOrd] | |  |

[Debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[Default]: https://doc.rust-lang.org/std/default/index.html
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[Copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[From]: https://doc.rust-lang.org/std/convert/trait.From.html
[FromStr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[Borrow]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
[Write]: https://doc.rust-lang.org/std/io/trait.Write.html
[Read]: https://doc.rust-lang.org/std/io/trait.Read.html
[Error]: https://doc.rust-lang.org/std/error/trait.Error.html
[Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[PartialEq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[Ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[PartialOrd]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html

Notes:

<a anchor="eq"></a>

* `Eq` and `PartialEq` provide slightly different promises.  `PartialEq`
  guarantees symmetric (`a == b` implies `b == a`) and transitive (if `a == b`
  and `b == c` then `a == c`) properties, but not the reflexive (`a == a`)
  property.  `Eq` adds the reflexive property guarantee.

## Iterator Traits

| Trait | Your type is ... |
| --- | --- |
| [Iterator] | an iterator. |
| [IntoIterator] | something iterable, ie `foo.iter()`. |
| [Sum] | an iterator that supports `foo.sum()`. |

[IntoIterator]: https://doc.rust-lang.org/core/iter/trait.IntoIterator.html
[Iterator]: https://doc.rust-lang.org/core/iter/trait.Iterator.html
[Sum]: https://doc.rust-lang.org/core/iter/trait.Sum.html

Iterable objects should support three flavors of the [IntoIterator] trait:
move, shared reference, and mutable reference.   See [this blog
post](https://www.philipdaniels.com/blog/2019/rust-api-design2/) for details.

## Collection Traits

Collections should implement [FromIterator] and [Extend].

| Trait | Your type is ... |
| --- | --- |
| [FromIterator] | a collection that works with `iter.collect()`. |
| [Extend] | a collection that supports `foo.extend(iterable)`. |

[Extend]: https://doc.rust-lang.org/core/iter/trait.Extend.html
[FromIterator]: https://doc.rust-lang.org/core/iter/trait.FromIterator.html

## Mathy Traits

Operations from https://doc.rust-lang.org/core/ops/#traits .

| Trait | Associated operation |
| --- | --- |
| [Add] | The addition operator +. |
| [AddAssign] | The addition assignment operator +=. |
| [BitAnd] | The bitwise AND operator &. |
| [BitAndAssign] | The bitwise AND assignment operator &=. |
| [BitOr] | The bitwise OR operator |. |
| [BitOrAssign] | The bitwise OR assignment operator |=. |
| [BitXor] | The bitwise XOR operator ^. |
| [BitXorAssign] | The bitwise XOR assignment operator ^=. |
| [Deref] | Used for immutable dereferencing operations, like \*v. |
| [DerefMut] | Used for mutable dereferencing operations, like in \*v = 1;. |
| [Div] | The division operator /. |
| [DivAssign] | The division assignment operator /=. |
| [Drop] | Custom code within the destructor. |
| [Fn] | The version of the call operator that takes an immutable receiver. |
| [FnMut] | The version of the call operator that takes a mutable receiver. |
| [FnOnce] | The version of the call operator that takes a by-value receiver. |
| [Index] | Used for indexing operations (container[index]) in immutable contexts. |
| [IndexMut] | Used for indexing operations (container[index]) in mutable contexts. |
| [Mul] | The multiplication operator \*. |
| [MulAssign] | The multiplication assignment operator \*=. |
| [Neg] | The unary negation operator -. |
| [Not] | The unary logical negation operator !. |
| [RangeBounds] | RangeBounds is implemented by Rust's built-in range types, produced by range syntax like .., a.., ..b, ..=c, d..e, or f..=g. |
| [Rem] | The remainder operator %. |
| [RemAssign] | The remainder assignment operator %=. |
| [Shl] | The left shift operator <<. |
| [ShlAssign] | The left shift assignment operator <<=. |
| [Shr] | The right shift operator >>. |
| [ShrAssign] | The right shift assignment operator >>=. |
| [Sub] | The subtraction operator -. |
| [SubAssign] | The subtraction assignment operator -=. |

[Add]: https://doc.rust-lang.org/std/ops/trait.Add.html
[AddAssign]: https://doc.rust-lang.org/std/ops/trait.AddAssign.html
[BitAnd]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
[BitAndAssign]: https://doc.rust-lang.org/std/ops/trait.BitAndAssign.html
[BitOr]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
[BitOrAssign]: https://doc.rust-lang.org/std/ops/trait.BitOrAssign.html
[BitXor]: https://doc.rust-lang.org/std/ops/trait.BitXor.html
[BitXorAssign]: https://doc.rust-lang.org/std/ops/trait.BitXorAssign.html
[Deref]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[DerefMut]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[Div]: https://doc.rust-lang.org/std/ops/trait.Div.html
[DivAssign]: https://doc.rust-lang.org/std/ops/trait.DivAssign.html
[Drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[Fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[FnMut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[FnOnce]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
[Index]: https://doc.rust-lang.org/std/ops/trait.Index.html
[IndexMut]: https://doc.rust-lang.org/std/ops/trait.IndexMut.html
[Mul]: https://doc.rust-lang.org/std/ops/trait.Mul.html
[MulAssign]: https://doc.rust-lang.org/std/ops/trait.MulAssign.html
[Neg]: https://doc.rust-lang.org/std/ops/trait.Neg.html
[Not]: https://doc.rust-lang.org/std/ops/trait.Not.html
[RangeBounds]: https://doc.rust-lang.org/std/ops/trait.RangeBounds.html
[Rem]: https://doc.rust-lang.org/std/ops/trait.Rem.html
[RemAssign]: https://doc.rust-lang.org/std/ops/trait.RemAssign.html
[Shl]: https://doc.rust-lang.org/std/ops/trait.Shl.html
[ShlAssign]: https://doc.rust-lang.org/std/ops/trait.ShlAssign.html
[Shr]: https://doc.rust-lang.org/std/ops/trait.Shr.html
[ShrAssign]: https://doc.rust-lang.org/std/ops/trait.ShrAssign.html
[Sub]: https://doc.rust-lang.org/std/ops/trait.Sub.html
[SubAssign]: https://doc.rust-lang.org/std/ops/trait.SubAssign.html

## Less Common Traits

| Trait | Your type is ... |
| --- | --- |
| [DoubleEndedIterator] | An [Iterator] that can be iterated forwards or backwards. |
| [ExactSizeIterator] | An [Iterator] with an efficient `len` method. |
| [FusedIterator] | An iterator that always continues to yield None when exhausted. |
| [Product] | Trait to represent types that can be created by multiplying elements of an iterator. |
| [Sum] | Trait to represent types that can be created by summing up an iterator. |

[DoubleEndedIterator]: https://doc.rust-lang.org/core/iter/trait.DoubleEndedIterator.html
[ExactSizeIterator]: https://doc.rust-lang.org/core/iter/trait.ExactSizeIterator.html
[FusedIterator]: https://doc.rust-lang.org/core/iter/trait.FusedIterator.html
[Product]: https://doc.rust-lang.org/core/iter/trait.Product.html

## References

* [The Rust Book 10.2: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
* [The Common Rust Traits](https://stevedonovan.github.io/rustifications/2018/09/08/common-rust-traits.html)

