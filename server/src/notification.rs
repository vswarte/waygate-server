use std::sync::mpsc::Sender;

use dashmap::DashMap;
use thiserror::Error;

use crate::logging::LogContext;

#[derive(Debug, Error)]
pub enum NotificationChannelPoolError {
    #[error("Player's notification channel could not be found.")]
    MissingPlayer,
    #[error("Channel error")]
    SendError,
}

#[derive(Default)]
/// Pool to hold a copy of the players push channel. Used for sending notifcations like server
/// maintenance announcements and message rating.
pub struct NotificationChannelPool {
    entries: DashMap<i32, Sender<Vec<u8>>>,
}

impl NotificationChannelPool {
    /// Send a notification push message to a specific player.
    #[allow(dead_code)]
    pub fn notify_player(
        &self,
        player: i32,
        message: Vec<u8>,
    ) -> Result<(), NotificationChannelPoolError> {
        let Some(channel) = self.entries.get(&player).map(|i| i.clone()) else {
            return Err(NotificationChannelPoolError::MissingPlayer);
        };

        channel
            .send(message)
            .map_err(|_| NotificationChannelPoolError::SendError)?;
        Ok(())
    }

    /// Send a notification push message to all connected players.
    pub fn broadcast(&self, message: Vec<u8>) -> Result<(), NotificationChannelPoolError> {
        self.entries.iter().for_each(|e| {
            if let Err(e) = e.value().send(message.clone()) {
                log::error!(
                    context:serde = LogContext::current(),
                    error:? = e;
                    "Could not broadcast message to user."
                );
            }
        });

        Ok(())
    }

    pub fn insert(&self, player_id: i32, entry: Sender<Vec<u8>>) -> NotificationChannelPoolToken {
        self.entries.insert(player_id, entry);
        NotificationChannelPoolToken(self, player_id)
    }

    pub fn remove(&self, player: i32) -> Result<(), NotificationChannelPoolError> {
        self.entries
            .remove(&player)
            .ok_or(NotificationChannelPoolError::MissingPlayer)?;
        Ok(())
    }
}

/// Represents an entry in the sign pool. Removes corresponding entry when dropped.
pub struct NotificationChannelPoolToken<'a>(&'a NotificationChannelPool, pub i32);

impl Drop for NotificationChannelPoolToken<'_> {
    fn drop(&mut self) {
        log::info!(
            context:serde = LogContext::current();
            "Destroying notification channel pool token"
        );
        let _ = self.0.remove(self.1);
    }
}
