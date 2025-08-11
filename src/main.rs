use nom::{bytes::complete::tag, IResult};

#[derive(Debug, PartialEq)]
struct A {
    a: u8,
    b: u8,
}

fn ret_int1(i: &[u8]) -> IResult<&[u8], u8> {
    Ok((i, 1))
}
fn ret_int2(i: &[u8]) -> IResult<&[u8], u8> {
    Ok((i, 2))
}

fn f(i: &[u8]) -> IResult<&[u8], A> {
    let (i, _) = tag("abcd")(i)?;
    let (i, a) = ret_int1(i)?;
    let (i, _) = tag("efgh")(i)?;
    let (i, b) = ret_int2(i)?;

    Ok((i, A { a, b }))
}

fn main() {
    let r = f(b"abcdefghX");
    println!("{}", r);

    println!("Hello, world!");
}
