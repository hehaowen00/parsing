use criterion::{criterion_group, criterion_main, Criterion};
use parsing::prelude::*;
use parsing::str::*;

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
        .skip_right(one_char(' '))
        .then(take_until(str_seq("\r\n")))
        .skip_right(str_seq("\r\n"));

    let header = take_until(one_char(':'))
        .skip_right(one_char(':'))
        .skip_right(whitespace())
        .then(take_until(str_seq("\r\n")))
        .skip_right(str_seq("\r\n"));

    req.then(many1(header)).skip_right(str_seq("\r\n"))
}

fn bench_fn(c: &mut Criterion) {
    let input = include_str!("large.txt").to_string();
    let input = input.replace("\r\n", "\n").replace("\n", "\r\n");

    let parser = many1(http_parser());

    c.bench_function("http parse", |b| {
        b.iter(||{
            let mut cursor = input.as_str();
            while let Ok((_, xs)) = parser.parse(cursor) {
                cursor = xs;
            }
        })
    });
}

criterion_group!(benches, bench_fn);
criterion_main!(benches);
