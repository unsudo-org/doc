use super::*;

pub struct Server;

impl app::Server for Server {
    fn serve<T>(&mut self, path: T)
    where
        T: Into<::std::path::PathBuf> {
        let content: ::std::path::PathBuf = path.into();
        let content: markdown::Markdown = content.try_into().unwrap();
        let content: html::Html = content.into();
        content.serve();
    }
}