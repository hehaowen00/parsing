use parsing::prelude::*;

struct RequestBuilder<'a> {
    method: Option<&'a str>,
    path: Option<&'a str>,
    http: Option<&'a str>,
    headers: Vec<(&'a str, &'a str)>,
}

struct Request<'a> {
    method: &'a str,
    path: &'a str,
    http: &'a str,
    headers: Vec<(&'a str, &'a str)>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new() -> Self {
        Self {
            method: None,
            path: None,
            http: None,
            headers: Vec::new(),
        }
    }

    pub fn build(self) -> Request<'a> {
        Request {
            method: self.method.unwrap(),
            path: self.path.unwrap(),
            http: self.http.unwrap(),
            headers: self.headers,
        }
    }
}

fn http_parser<'a>() -> impl Parse<'a, Output = Request<'a>> {
    state(|| RequestBuilder::new())
        .then(parse_request())
        .map(|(mut builder, (method, path, version))| {
            builder.method = Some(method);
            builder.path = Some(path);
            builder.http = Some(version);
            builder
        })
        .then(parse_headers())
        .map(|(mut builder, headers)| {
            builder.headers = headers;
            builder
        })
        .skip(slice(b"\r\n"))
        .map(|builder| builder.build())
}

fn parse_request<'a>() -> Cell<'a, impl Parse<'a, Output = (&'a str, &'a str, &'a str)>> {
    let method = slice(b"GET")
        | slice(b"HEAD")
        | slice(b"POST")
        | slice(b"PUT")
        | slice(b"DELETE")
        | slice(b"CONNECT")
        | slice(b"OPTIONS")
        | slice(b"TRACE")
        | slice(b"PATCH");

    let method = method.map(|bytes| to_str(bytes));
    let path = take_until_literal(b" ").map(|bytes| to_str(bytes));
    let version = take_until_literal(b"\r\n").map(|bytes| to_str(bytes));

    method
        .skip(chr(' '))
        .then(path)
        .skip(chr(' '))
        .then(version)
        .skip(slice(b"\r\n"))
        .map(|((a, b), c)| (a, b, c))
}

fn parse_headers<'a>() -> Cell<'a, impl Parse<'a, Output = Vec<(&'a str, &'a str)>>> {
    let header = take_until_literal(b":")
        .skip(slice(b": "))
        .then(take_until_literal(b"\r\n"))
        .map(|(key, value)| (to_str(key), to_str(value)))
        .skip(slice(b"\r\n"));

    many1(header)
}

fn to_str<'a>(bytes: &'a [u8]) -> &'a str {
    unsafe { std::str::from_utf8_unchecked(bytes) }
}

fn http_test() {
    let bytes = "GET /index.html HTTP/1.1\r\n\
        User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n\
        Accept-Language: en-us\r\n\
        Accept-Encoding: gzip, deflate\r\n\
        Connection: Keep-Alive\r\n\r\n"
        .to_stream();

    let parser = http_parser();

    match parser.parse(bytes) {
        Ok((xs, request)) => {
            assert_eq!(request.method, "GET");
            assert_eq!(request.path, "/index.html");
            assert_eq!(request.http, "HTTP/1.1");
            assert_eq!(
                request.headers,
                vec![
                    (
                        "User-Agent",
                        "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)"
                    ),
                    ("Accept-Language", "en-us"),
                    ("Accept-Encoding", "gzip, deflate"),
                    ("Connection", "Keep-Alive")
                ]
            );
            assert_eq!(xs, &[]);
        }
        Err(xs) => {
            panic!("failed to parse bytes: {:?}", to_str(xs));
        }
    }
}
