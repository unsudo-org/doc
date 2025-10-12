use super::*;

pub use app::Result;
pub use app::Error;

pub struct Server;

impl app::Server for Server {
    fn serve<T>(&mut self, path: T) -> app::Result<()>
    where
        T: Into<::std::path::PathBuf> {
        let content: ::std::path::PathBuf = path.into();
        let content: markdown::Markdown = content.try_into()?;
        let content: html::Html = content.into();
        content.serve()?;
        Ok(())
    }
}