use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

pub struct Foo {
    items: Vec<u64>,
}

impl Foo {
    pub fn new(from: u64, to: u64) -> Self {
        Foo {
            items: (from..to).collect(),
        }
    }

    pub fn contains(&self, bar: u64) -> Option<bool> {
        thread::scope(|s| {
            let cores = thread::available_parallelism().ok()?.get();
            let mut threads = Vec::with_capacity(cores);

            let chunks = self
                .items
                .chunks(self.items.len() / cores)
                .collect::<Vec<_>>();

            let is_running = Arc::new(AtomicBool::new(true));

            for (i, &chunk) in chunks.iter().enumerate() {
                let is_running = Arc::clone(&is_running);
                let thread = s.spawn(move || {
                    for item in chunk {
                        if !is_running.load(Ordering::Relaxed) {
                            break;
                        }
                        println!("[{i} of {cores}]: {:?}", item);
                        if *item == bar {
                            println!("[{i} of {cores}]: {:?} FOUND!", item);
                            is_running.store(false, Ordering::Relaxed);
                            return true;
                        }
                    }
                    false
                });
                threads.push(thread);
            }

            let result = threads.into_iter().any(|t| t.join().unwrap_or(false));
            Some(result)
        })
    }
}

#[test]
fn try_threading() {
    let f = Foo::new(0, 100_000_000);
    if let Some(res) = f.contains(42424242) {
        println!("Contains: {res}");
    }
    else {
        println!("Some problem!");
    }
}
