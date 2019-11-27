use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens, Token};
use str_index::{StrIndex, StrRange};

#[test]
fn str_index() {
    assert_tokens(
        &StrIndex::from(0),
        &[Token::NewtypeStruct { name: "StrIndex" }, Token::U32(0)],
    );
    assert_de_tokens(&StrIndex::from(0), &[Token::U32(0)])
}

#[test]
fn str_range() {
    let range = StrRange::from(StrIndex::from(0)..StrIndex::from(10));
    assert_tokens(
        &range,
        &[
            Token::Struct {
                name: "StrRange",
                len: 2,
            },
            Token::Str("start"),
            Token::NewtypeStruct { name: "StrIndex" },
            Token::U32(0),
            Token::Str("end"),
            Token::NewtypeStruct { name: "StrIndex" },
            Token::U32(10),
            Token::StructEnd,
        ],
    );
    assert_de_tokens(
        &range,
        &[
            Token::Map { len: Some(2) },
            Token::Str("start"),
            Token::U32(0),
            Token::Str("end"),
            Token::U32(10),
            Token::MapEnd,
        ],
    );
    assert_de_tokens(
        &range,
        &[
            Token::Seq { len: Some(2) },
            Token::U32(0),
            Token::U32(10),
            Token::SeqEnd,
        ],
    );

    assert_de_tokens_error::<StrRange>(
        &[
            Token::Seq { len: Some(2) },
            Token::U32(10),
            Token::U32(0),
            Token::SeqEnd,
        ],
        "invalid string range 10..0",
    );
    assert_de_tokens_error::<StrRange>(
        &[
            Token::Struct {
                name: "StrRange",
                len: 2,
            },
            Token::Str("start"),
            Token::U32(10),
            Token::Str("end"),
            Token::U32(0),
            Token::StructEnd,
        ],
        "invalid string range 10..0",
    );
}
