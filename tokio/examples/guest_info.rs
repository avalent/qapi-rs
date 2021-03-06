#[cfg(feature = "qga")]
mod main {
    use std::env::args;
    use tokio_uds::UnixStream;
    use tokio::prelude::*;
    use tokio::run;
    use tokio_qapi::{self, qga};

    pub fn main() {
        ::env_logger::init();

        let socket_addr = args().nth(1).expect("argument: QEMU Guest Agent socket path");

        run(UnixStream::connect(socket_addr)
            .map(|stream| tokio_qapi::stream(stream))
            .and_then(|stream| tokio_qapi::qga_handshake(stream))
            .and_then(|stream| stream.execute(qga::guest_info { }))
            .and_then(|(info, _stream)| info.map_err(From::from))
            .map(|info| println!("Guest Agent version: {}", info.version))
            .map_err(|e| panic!("Failed with {:?}", e))
        );
    }
}

#[cfg(not(feature = "qga"))]
mod main {
    pub fn main() { panic!("requires feature qga") }
}

fn main() { main::main() }
