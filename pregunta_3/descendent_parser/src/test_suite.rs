use crate::desc_parser;

#[allow(unused)]
pub fn s(s : &str) -> String {
    s.to_string()
}

#[allow(unused)]
pub fn parse(s : &str) -> Option<String> {
    desc_parser::Parser::new().parse_string(s)
}
#[test]
fn test_parse_ok() {
    let eit = desc_parser::to_either;

    assert_eq!(parse("int"),        Some(s("int") ));
    assert_eq!(parse("int;char"),   Some(s("char")));
    assert_eq!(parse("int ; char"), Some(s("char")));
    assert_eq!(parse("int ; char; bool; i64"), Some(s("i64")));
    assert_eq!(parse("try char catch i64"), Some(eit(s("char"), s("i64"))));
    assert_eq!(parse("try char catch i64 finally bool"), Some(s("bool")));
    assert_eq!(parse("try char catch i64 finally bool; try char catch i32"), Some(eit(s("char"), s("i32"))));
    assert_eq!(parse("try char catch i64 finally bool; try char catch i32; string"), Some(eit(s("char"), s("string"))));
    assert_eq!(parse("try char catch i64 finally bool; try char catch i32; string finally f64"), Some(s("f64")));
    assert_eq!(parse("try char catch i64 finally bool; try char catch i32; string finally f64; maybe"), Some(s("maybe")));
    assert_eq!(parse("try char catch try f64 catch i32 finally char finally bool"), Some(s("bool")));
    assert_eq!(parse("try try i32 catch bool catch f64"), Some(eit(  eit(s("i32"), s("bool")), s("f64"))  ));
    assert_eq!(parse("try try i32 catch bool catch try char catch string"), Some(eit(  eit(s("i32"), s("bool")), eit(s("char"), s("string")))  ));
    
}

#[test]
fn test_parse_reject() {
    assert_eq!(parse("try"), None);
    assert_eq!(parse("catch"), None);
    assert_eq!(parse("finally"), None);
    assert_eq!(parse(";"), None);
    assert_eq!(parse("try error"), None);
    assert_eq!(parse("try error catch "), None);
    assert_eq!(parse("try error catch wtf finally "), None);
    assert_eq!(parse("try error catch wtf finally success;"), None);
    assert_eq!(parse(";try error catch wtf finally success"), None);
    assert_eq!(parse(";anarchy;chaos"), None);
    assert_eq!(parse(""), None);
}