use std::{io, sync::Arc, time::Duration};

use serenity::all::{ActivityData, ShardManager};
use tokio::time;

use super::client::Client;

pub struct Activity {
    client: Client,
    shard_manager: Arc<ShardManager>,
}

impl Activity {
    pub async fn new<T: Into<String>>(
        host: T,
        port: u16,
        shard_manager: Arc<ShardManager>,
    ) -> io::Result<Self> {
        let client = Client::new(host.into(), port).await?;

        Ok(Self {
            client,
            shard_manager,
        })
    }

    pub async fn start(self) {
        let mut interval = time::interval(Duration::from_secs(30));

        loop {
            interval.tick().await;

            let info = match self.client.status().await {
                Ok(info) => info,
                Err(e) => {
                    log::error!("Failed to fetch status: {}", e);
                    continue;
                }
            };

            if let Err(e) = self
                .update_activity(ActivityData::watching(format!(
                    "{}/{}",
                    info.players.online, info.players.max
                )))
                .await
            {
                log::error!("Failed to update activity: {}", e);
            }
        }
    }

    async fn update_activity(&self, activity: ActivityData) -> io::Result<()> {
        for (.., runner) in self.shard_manager.runners.lock().await.iter() {
            runner.runner_tx.set_activity(Some(activity.clone()));
        }
        Ok(())
    }
}
