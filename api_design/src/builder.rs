#[derive(Debug)]
pub struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
    debug: bool,
}

impl ServerConfig {
    // Required field
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),

            // Sensible defaults
            port: 8080,
            workers: 4,
            debug: false,
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

pub fn demo() {
    let config = ServerConfig::new("localhost")
        .port(3000)
        .workers(8)
        .debug(true)
        .build();

    println!("{:#?}", config);
}