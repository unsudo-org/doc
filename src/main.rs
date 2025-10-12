mod html;
mod markdown;
mod server;

trait Server {
    fn serve<T>(&mut self, path: T)
    where
        T: Into<::std::path::PathBuf>;
}

struct App<A> 
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

fn main() {
    let server: server::Server = server::Server;
    let app: App<server::Server> = App::new(server);
    app.run();
}