mod html;
mod markdown;
mod server;

mod app {
    pub trait Server {
        fn serve<T>(&mut self, path: T)
        where
            T: Into<::std::path::PathBuf>;
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

        pub fn run(mut self) {
            self.server.serve("doc/main.md");
        }
    }
}

fn main() {
    let server: server::Server = server::Server;
    let app: app::App<server::Server> = app::App::new(server);
    app.run();
}