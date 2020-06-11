mod config;
use actix::prelude::*;
use better_dwm_statusbar::module_worker::ModuleWorker;
use better_dwm_statusbar::status_server::StatusServer;

use config::get_config;

fn main() {
    let config = get_config();
    let server = StatusServer::new(config.len(), " | ");

    System::run(|| {
        let server_addr = server.start();
        config
            .into_iter()
            .enumerate()
            .for_each(|(index, module_wrapper)| {
                let worker = ModuleWorker::new(module_wrapper, server_addr.clone(), index);
                SyncArbiter::start(1, || worker);
            });
    });
}
