use anyhow::Error;
use futures::stream::StreamExt;
use log::*;
use paho_mqtt::{self as mqtt, AsyncClient, AsyncReceiver};
use std::{process, time::Duration};

pub async fn setup(
    topics: &[&str],
    qos: &[i32],
    host: String,
) -> Result<(AsyncClient, AsyncReceiver<Option<mqtt::Message>>), mqtt::Error> {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("mqtt-exporter")
        .finalize();

    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        info!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    // Get message stream before connecting.
    let strm = cli.get_stream(25);

    let lwt = mqtt::Message::new("fin", "Async subscriber lost connection", mqtt::QOS_1);

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(30))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(false)
        .will_message(lwt)
        .finalize();

    info!("Connecting to the MQTT server...");
    cli.connect(conn_opts).await?;

    info!("Subscribing to topics: {:?}", topics);
    cli.subscribe_many(topics, qos).await?;

    Ok((cli, strm))
}

pub async fn run(
    cli: AsyncClient,
    mut strm: AsyncReceiver<Option<mqtt::Message>>,
    dispatch: &dyn Fn(mqtt::Message),
) -> Result<(), Error> {
    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            dispatch(msg);
        } else {
            warn!("Lost connection. Attempting reconnect.");
            while let Err(err) = cli.reconnect().await {
                warn!("Error reconnecting: {}", err);
                tokio::time::sleep(Duration::from_millis(1000)).await
            }
        }
    }
    Ok(())
}
