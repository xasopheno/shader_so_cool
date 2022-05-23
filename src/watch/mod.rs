use notify::{
    event::AccessKind, event::AccessMode, event::EventKind, event::ModifyKind, RecommendedWatcher,
    RecursiveMode, Watcher,
};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};

pub fn watch(dir: &'static str) -> Result<(Receiver<bool>, Sender<bool>), notify::Error> {
    let (tx, rx) = channel();
    let (output_tx, output_rx) = channel();
    let (kill_tx, kill_rx) = channel();
    let path = Path::new(dir);
    let mut socool_watcher = RecommendedWatcher::new(tx).unwrap();

    std::thread::spawn(move || -> Result<(), notify::Error> {
        socool_watcher
            .watch(path.as_ref(), RecursiveMode::Recursive)
            .unwrap();

        loop {
            {
                if let Ok(true) = kill_rx.try_recv() {
                    println!("killed watcher");
                    break;
                }
                if let Ok(event) = rx.try_recv() {
                    // println!("{:?}", event);
                    match event {
                        Ok(notify::Event {
                            kind: EventKind::Access(AccessKind::Close(AccessMode::Write)),
                            ..
                        }) => {
                            println!("{}", path.display());
                            output_tx.send(true).expect("oh no watcher can't send");
                        }
                        Ok(notify::Event {
                            kind: EventKind::Modify(ModifyKind::Data { .. }),
                            ..
                        }) => {
                            println!("{}", path.display());
                            output_tx.send(true).expect("oh no! watcher can't send!");
                        }
                        _ => {
                            // dbg!(event);
                        }
                    }
                }
            }
        }
        Ok(())
    });

    Ok((output_rx, kill_tx))
}
