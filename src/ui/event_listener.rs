use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::time::Duration;
use anyhow::Context;
use crossterm::event;
use crossterm::event::Event;

pub fn spawn_event_listener() -> Receiver<anyhow::Result<Event>> {
    let (tx, rx) = channel::<anyhow::Result<Event>>();
    std::thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(250)).expect("Polling failed") {
                tx.send(event::read().with_context(|| "Failed to read event"))
                    .expect("Failed to send message");
            }
        }
    });
    rx
}

