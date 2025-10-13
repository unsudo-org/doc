mod html;
mod markdown;
mod server;

mod app {
    const WHITEPAPER: &str = "doc/whitepaper.md";

    pub trait Server {
        fn serve<T>(&mut self, path: T) -> Result<()>
        where
            T: Into<::std::path::PathBuf>;
    }

    pub type Result<T> = ::std::result::Result<T, Error>;

    #[derive(Debug)]
    #[derive(::thiserror::Error)]
    pub enum Error {
        #[error("{0}")]
        IO(#[from] ::std::io::Error),
        #[error("unable to bind to address")]
        UnableToBindToAddress
    }

    pub struct App<A> 
    where
        A: Server {
        server: A
    }

    impl<A> App<A> 
    where
        A: Server {
        pub fn new(server: A) -> Self {
            Self {
                server
            }
        }

        pub fn run(mut self) -> Result<()> {
            self.server.serve(WHITEPAPER)?;
            Ok(())
        }
    }
}

fn main() {
    let server: server::Server = server::Server;
    let app: app::App<server::Server> = app::App::new(server);
    app.run().unwrap();
}