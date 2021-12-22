use criterion::{criterion_group, criterion_main, Criterion};
use parsing::prelude::*;

type Request<'a> = (&'a [char], &'a [char], &'a [char]);
type Headers<'a> = Vec<(&'a [char], &'a [char])>;

fn http_parser<'a>() -> impl Parse<'a, char, Output = (Request<'a>, Headers<'a>)> {
    let method = Seq::str("GET")
        .or(Seq::str("POST"))
        .or(Seq::str("PUT"))
        .or(Seq::str("DELETE"))
        .or(Seq::str("HEAD"))
        .or(Seq::str("CONNECT"))
        .or(Seq::str("OPTIONS"))
        .or(Seq::str("TRACE"))
        .or(Seq::str("PATCH"));

    let req = method.skip_right(pchar(' '))
        .then(take_until(pchar(' ')))
        .skip_right(pchar(' '))
        .then(take_until(Seq::str("\r\n")))
        .skip_right(Seq::str("\r\n"))
        .map(|((a, b), c)| (a, b, c));

    let header = take_until(pchar(':'))
        .skip_right(pchar(':'))
        .skip_right(whitespace())
        .then(take_until(Seq::str("\r\n")))
        .skip_right(Seq::str("\r\n"));

    req.then(many1(header)).skip_right(Seq::str("\r\n"))
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
