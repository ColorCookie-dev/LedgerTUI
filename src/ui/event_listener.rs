use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::time::Duration;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyEvent;

pub fn spawn_event_listener() -> Receiver<Result<KeyEvent, String>> {
    let (tx, rx) = channel::<core::result::Result<KeyEvent, String>>();
    std::thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(250))
                .expect("Polling failed") {
                if let Event::Key(key) = event::read()
                    .expect("Couldn't read key") {
                        tx.send(Ok(key))
                            .expect("Failed to send message");
                    }
            }
        }
    });
    rx
}

