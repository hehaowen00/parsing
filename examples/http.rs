use parsing::prelude::*;

type Request<'a> = (&'a [char], &'a [char], &'a [char]);
type Headers<'a> = Vec<(&'a [char], &'a [char])>;

fn http_parser<'a>() -> impl Parse<'a, char, Output = (Request<'a>, Headers<'a>)> {
    let method = pstr("GET")
        .or(pstr("POST"))
        .or(pstr("PUT"))
        .or(pstr("DELETE"))
        .or(pstr("HEAD"))
        .or(pstr("CONNECT"))
        .or(pstr("OPTIONS"))
        .or(pstr("TRACE"))
        .or(pstr("PATCH"));

    let req = method.skip_right(pchar(' '))
        .then(take_until(pchar(' ')))
        .skip_right(pchar(' '))
        .then(take_until(pstr("\r\n")))
        .skip_right(pstr("\r\n"))
        .map(|((a, b), c)| (a, b, c));

    let header = take_until(pchar(':'))
        .skip_right(pchar(':'))
        .skip_right(whitespace())
        .then(take_until(pstr("\r\n")))
        .skip_right(pstr("\r\n"));

    req.then(many1(header)).skip_right(pstr("\r\n"))
}

fn main() {
    let message = "GET /index.html HTTP/1.1\r\n\
        User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n\
        Accept-Language: en-us\r\n\
        Accept-Encoding: gzip, deflate\r\n\
        Connection: Keep-Alive\r\n\r\n";
    let chars: Vec<_> = message.chars().collect();

    let parser = http_parser();
    let res = parser.parse(&chars).unwrap();
    println!("{:?}", res);
}
