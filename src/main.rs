#![deny(warnings)]

use crate::exporter::ExporterRegistry;
use anyhow::Error;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use log::*;
use std::env;

mod exporter;
mod mqtt;
mod prometheus_server;

// The exported data types
mod exporters;
use exporters::*;

// The topics to which we subscribe.
const TOPICS: &[&str] = &["particle"];
const QOS: &[i32] = &[1, 1];

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "tcp://localhost:1883".to_string());

    let (cli, strm) = mqtt::setup(TOPICS, QOS, host).await?;

    let addr = std::net::SocketAddr::from(([192, 168, 1, 133], 3000));
    info!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(prometheus_server::serve))
    }));

    // <<< Any new parsers must be registered here >>>
    let mut parser = ExporterRegistry::new();
    parser.register(Box::new(particles::ParticleExporter::default()));


    let dispatch = &|msg| parser.dispatch(msg);
    tokio::select! {
        _prometheus = serve_future => (),
        _mqtt = mqtt::run(cli, strm, dispatch) => (),
    }
    Ok(())
}
