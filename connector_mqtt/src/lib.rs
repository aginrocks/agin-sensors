mod beanair;

use aginsensors_core::{
    connector::{ConnectorEvent, ConnectorRunner},
    define_connector,
};
use color_eyre::eyre::{Context, Result, bail};
use rumqttc::{AsyncClient, ConnectionError, Event, MqttOptions, Packet, QoS};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tracing::warn;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MqttFormat {
    BeanAir,
}

define_connector!(
    "mqtt",
    Mqtt,
    config = {
        pub host: String,
        pub port: u16,
        pub username: String,
        pub password: String,
        pub format: MqttFormat,
        pub topic: Option<String>,
    },
    state = {
        tx: Arc<Sender<Vec<ConnectorEvent>>>,
        rx: Arc<Receiver<Vec<ConnectorEvent>>>,
    }
);

impl MqttConnector for Mqtt {
    fn new(config: &ConfigMqtt) -> Self {
        let (tx, rx) = channel::<Vec<ConnectorEvent>>(1000);

        Mqtt {
            config: config.clone(),
            tx: Arc::new(tx),
            rx: Arc::new(rx),
        }
    }
}

impl ConnectorRunner for Mqtt {
    fn run(&self) -> Arc<Receiver<Vec<ConnectorEvent>>> {
        let this = self.clone();
        tokio::spawn(async move { this.listen().await });

        self.rx.clone()
    }
}

impl Mqtt {
    async fn listen(&self) -> Result<()> {
        let mut options =
            MqttOptions::new("agin-sensors", self.config.host.clone(), self.config.port);

        options.set_credentials(self.config.username.clone(), self.config.password.clone());

        let (client, mut eventloop) = AsyncClient::new(options, 1000);

        client
            .subscribe(
                self.config.topic.clone().unwrap_or("#".to_string()),
                QoS::AtLeastOnce,
            )
            .await?;

        loop {
            let event = eventloop.poll().await;
            if let Err(error) = self.handle_event(event).await {
                warn!(error = ?error, "Error while handling MQTT event");
            }
        }
    }

    async fn handle_event(&self, loop_event: Result<Event, ConnectionError>) -> Result<()> {
        let event = loop_event.wrap_err("Received an error from MQTT event loop")?;

        let parsed = self
            .parse_event(&event)
            .wrap_err("Failed to parse MQTT event")?;

        self.tx.send(vec![parsed]).await?;

        Ok(())
    }

    fn parse_event(&self, event: &Event) -> Result<ConnectorEvent> {
        match event {
            Event::Incoming(Packet::Publish(publish)) => {
                let (measurement, metadata) = match self.config.format {
                    MqttFormat::BeanAir => beanair::parse(),
                }?;

                Ok(ConnectorEvent::new_measurement(measurement, metadata))
            }
            _ => bail!("Unsupported MQTT event type"),
        }
    }
}
