use ::pulldown_cmark as cmark;
use ::tiny_http as http;

use super::*;

pub use app::Result;
pub use app::Error;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Html(String);

impl Html {
    pub fn serve(self) -> app::Result<()> {
        let host: http::Server = http::Server::http("0.0.0.0:8000").ok().ok_or(app::Error::UnableToBindToAddress)?;
        for request in host.incoming_requests() {
            let header: http::Header = "Content-Type: text/html".parse().expect("correct header");
            let response: String = self.0.to_owned();
            let response: http::Response<_> = http::Response::from_string(response);
            let response: http::Response<_> = response.with_header(header);
            request.respond(response)?;
        }
        Ok(())
    }
}

impl From<markdown::Markdown> for Html {
    fn from(value: markdown::Markdown) -> Self {
        let markdown: markdown::Markdown = value;
        let markdown: String = markdown.0;
        let mut conf: cmark::Options = cmark::Options::empty();
        conf.insert(cmark::Options::ENABLE_DEFINITION_LIST);
        conf.insert(cmark::Options::ENABLE_FOOTNOTES);
        conf.insert(cmark::Options::ENABLE_GFM);
        conf.insert(cmark::Options::ENABLE_HEADING_ATTRIBUTES);
        conf.insert(cmark::Options::ENABLE_MATH);
        conf.insert(cmark::Options::ENABLE_OLD_FOOTNOTES);
        conf.insert(cmark::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
        conf.insert(cmark::Options::ENABLE_SMART_PUNCTUATION);
        conf.insert(cmark::Options::ENABLE_STRIKETHROUGH);
        conf.insert(cmark::Options::ENABLE_SUBSCRIPT);
        conf.insert(cmark::Options::ENABLE_SUPERSCRIPT);
        conf.insert(cmark::Options::ENABLE_TABLES);
        let mut html: String = String::new();
        cmark::html::push_html(&mut html, cmark::Parser::new_ext(&markdown, conf));
        let ret: String = html;
        let ret: Self = Self(ret);
        ret
    }
}

impl TryFrom<::std::path::PathBuf> for Html {
    type Error = app::Error;

    fn try_from(value: ::std::path::PathBuf) -> ::std::result::Result<Self, Self::Error> {
        Ok(Self(::std::fs::read_to_string(value)?))
    }
}