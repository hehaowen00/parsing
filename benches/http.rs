use criterion::{criterion_group, criterion_main, Criterion};
use parsing::prelude::*;

type Request<'a> = (&'a [char], &'a [char], &'a [char]);
type Headers<'a> = Vec<(&'a [char], &'a [char])>;

fn http_parser<'a>() -> impl Parse<'a, char, Output = (Request<'a>, Headers<'a>)> {
    let method = str_seq("GET")
        .or(str_seq("POST"))
        .or(str_seq("PUT"))
        .or(str_seq("DELETE"))
        .or(str_seq("HEAD"))
        .or(str_seq("CONNECT"))
        .or(str_seq("OPTIONS"))
        .or(str_seq("TRACE"))
        .or(str_seq("PATCH"));

    let req = method.skip_right(one_char(' '))
        .then(take_until(one_char(' ')))
        .skip_right(one_char(' '))
        .then(take_until(str_seq("\r\n")))
        .skip_right(str_seq("\r\n"))
        .map(|((a, b), c)| (a, b, c));

    let header = take_until(one_char(':'))
        .skip_right(one_char(':'))
        .skip_right(whitespace())
        .then(take_until(str_seq("\r\n")))
        .skip_right(str_seq("\r\n"));

    req.then(many1(header)).skip_right(str_seq("\r\n"))
}

fn bench_fn(c: &mut Criterion) {
    let message = "GET /index.html HTTP/1.1\r\n\
        User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n\
        Accept-Language: en-us\r\n\
        Accept-Encoding: gzip, deflate\r\n\
        Connection: Keep-Alive\r\n\r\n";
    let chars: Vec<_> = message.chars().collect();
    let parser = http_parser();

    c.bench_function("http parse", |b| {
        b.iter(|| parser.parse(&chars))
    });
}

criterion_group!(benches, bench_fn);
criterion_main!(benches);
