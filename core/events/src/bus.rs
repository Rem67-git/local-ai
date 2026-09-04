use crate::events::Event;
use async_broadcast::broadcast;
use std::sync::Arc;

pub struct EventBus {
    tx: async_broadcast::Sender<Arc<Event>>,
    rx: async_broadcast::Receiver<Arc<Event>>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = broadcast(capacity);
        Self { tx, rx }
    }

    pub async fn emit(&self, event: Event) {
        let event = Arc::new(event);
        let _ = self.tx.broadcast(event).await;
    }

    pub fn subscribe(&self) -> async_broadcast::Receiver<Arc<Event>> {
        self.rx.clone()
    }

    pub fn sender(&self) -> async_broadcast::Sender<Arc<Event>> {
        self.tx.clone()
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventType;

    #[tokio::test]
    async fn test_event_bus_emit_and_receive() {
        let bus = EventBus::new(10);
        let mut rx = bus.subscribe();

        let event = Event::new(EventType::MissionStarted {
            mission_id: "test".to_string(),
        });

        bus.emit(event).await;

        let received = rx.recv().await;
        assert!(received.is_ok());
    }
}
