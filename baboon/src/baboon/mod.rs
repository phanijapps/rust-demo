use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{sleep, Duration};

struct BaboonsCrossingCanyon {
    northbound_count: Arc<AtomicUsize>, // Count of northbound baboons
    southbound_count: Arc<AtomicUsize>, // Count of southbound baboons
    rope: Arc<Semaphore>,               // Semaphore to control the rope
    mutex: Arc<Mutex<()>>,              // Mutex to protect shared variables
}

impl BaboonsCrossingCanyon {
    fn new() -> Self {
        Self {
            northbound_count: Arc::new(AtomicUsize::new(0)),
            southbound_count: Arc::new(AtomicUsize::new(0)),
            rope: Arc::new(Semaphore::new(1)), // Only one group can cross at a time
            mutex: Arc::new(Mutex::new(())),
        }
    }

    async fn northbound_baboon_wants_to_cross(self: Arc<Self>) {
        {
            let _lock = self.mutex.lock().await; // Lock to protect shared variables
            self.northbound_count.fetch_add(1, Ordering::SeqCst); // Increment northbound count
            if self.northbound_count.load(Ordering::SeqCst) == 1 {
                self.rope.add_permits(1); // First baboon acquires the rope
            }
        }

        // Baboon crosses the canyon
        self.cross_canyon("Northbound").await;
        {
            let _lock = self.mutex.lock().await; // Lock to protect shared variables
            self.northbound_count.fetch_sub(1, Ordering::SeqCst); // Decrement northbound count
            if self.northbound_count.load(Ordering::SeqCst) == 0 {
                self.rope.add_permits(1); // Last baboon releases the rope
            }
        }
    }

    async fn southbound_baboon_wants_to_cross(self: Arc<Self>) {
        {
            let _lock = self.mutex.lock().await; // Lock to protect shared variables
            self.southbound_count.fetch_add(1, Ordering::SeqCst); // Increment southbound count
            if self.southbound_count.load(Ordering::SeqCst) == 1 {
                self.rope.acquire().await; // First baboon acquires the rope
            }
        }

        // Baboon crosses the canyon
        self.cross_canyon("Southbound").await;
        {
            let _lock = self.mutex.lock().await; // Lock to protect shared variables
            self.southbound_count.fetch_sub(1, Ordering::SeqCst); // Decrement southbound count
            if self.southbound_count.load(Ordering::SeqCst) == 0 {
                self.rope.add_permits(1); // Last baboon releases the rope
            }
        }
    }

    async fn cross_canyon(&self, direction: &str) {
        println!("{} baboon is crossing the canyon.", direction);
        //sleep(Duration::from_secs(1)).await; // Simulate time taken to cross
        println!("{} baboon has crossed the canyon.", direction);
    }
}

pub async fn baboons_crossing_canyon(northbound: usize, southbound: usize) {
    let canyon = Arc::new(BaboonsCrossingCanyon::new());

    let mut tasks = Vec::new();

    // Simulate northbound baboons
    let canyon_clone = Arc::clone(&canyon);
    tasks.push(tokio::spawn(async move {
        for _ in 0..northbound {
            canyon_clone
                .clone()
                .northbound_baboon_wants_to_cross()
                .await;
        }
    }));

    // Simulate southbound baboons
    let canyon_clone = Arc::clone(&canyon);
    tasks.push(tokio::spawn(async move {
        for _ in 0..southbound {
            canyon_clone
                .clone()
                .southbound_baboon_wants_to_cross()
                .await;
        }
    }));

    // Await all tasks to finish
    for task in tasks {
        task.await.unwrap();
    }
}
