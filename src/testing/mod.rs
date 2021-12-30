use crate::str::*;
use crate::prelude::*;

type Request<'a> = ((&'a str, &'a str), &'a str);
type Headers<'a> = Vec<(&'a str, &'a str)>;

fn http_parser<'a>() -> impl ParseStr<'a, Output = (Request<'a>, Headers<'a>)> {
    let methods = [
        "GET", "POST", "PUT", "DELETE", "HEAD",
        "CONNECT", "OPTIONS", "TRACE", "PATCH",
    ];
    let method = OneOf::<String>::from(&methods);

    let req = method.skip_right(one_char(' '))
        .then(take_until(one_char(' ')))
        .skip_n(1)
        .then(take_until(str_seq("\r\n")))
        .skip_n(2);

    let header = take_until(one_char(':'))
        .skip_n(2)
        .then(take_until(str_seq("\r\n")))
        .skip_n(2);

    req.then(many1(header)).skip_n(2)
}

#[test]
fn single_input() {
    let input = "GET /index.html HTTP/1.1\r\n\
        User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n\
        Accept-Language: en-us\r\n\
        Accept-Encoding: gzip, deflate\r\n\
        Connection: Keep-Alive\r\n\r\n";

    let parser = http_parser();
    let res = parser.parse(&input);
    println!("{:?}", res.unwrap());
}

#[test]
fn large_input() {
    let input = include_str!("large.txt").to_string();
    let input = input.replace("\r\n", "\n").replace("\n", "\r\n");

    let parser = many1(http_parser());
    let res = parser.parse(&input);
    println!("{:?}", res);
}

