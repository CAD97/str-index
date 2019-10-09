# str-index

A sketch for a general Rust string index newtype.

Discuss on Zulip: <https://rust-lang.zulipchat.com/#narrow/stream/185405-t-compiler.2Fwg-rls-2.2E0/topic/str_index>

The idea is to produce [one API to replace](https://xkcd.com/927/) the
[veritable number of Pos/Span structures on crates](https://github.com/CAD97/str-index/pull/1#issuecomment-539694123).
We have tentative buy-in from [text_unit](https://lib.rs/crates/text_unit) (used by rowan/rust-analyzer) and
[codespan](https://lib.rs/crates/codespan) (used by gluon) at least for adopting a unified type.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
