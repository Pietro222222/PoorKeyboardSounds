use std::{io::{Cursor}, thread};
use rdev::listen;



use anyhow::Result;

fn play_sound() -> Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.set_volume(0.2);
    let a: &[u8] = include_bytes!("../../assets/music.wav");
    let cursor = Cursor::new(a);

    sink.append(rodio::Decoder::new(cursor).unwrap());

    sink.sleep_until_end();

    Ok(())
}

pub async fn listener() {
    tokio::spawn(async move {
        if let Err(_) = listen(move |e| {

            match e.event_type {
                rdev::EventType::KeyPress(_) => {
                    thread::spawn(|| {
                        play_sound().unwrap();
                    });
                }
                _ => {}
            }

        }) {
            //we do nothing
        }
    }).await.ok();
}
