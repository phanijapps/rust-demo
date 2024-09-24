use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use async_lock::{Mutex, Semaphore};

/*
 Baboons Crossing a canyon.
 Baboons are crossing east to west and west to east from the end of
 a canyon and only one baboon is allowed on the thread.

 args
 west i8: number of baboons trying to cross from west to east
 east i8: number of baboons trying to cross from east to west
*/

struct Baboons {
    west_bound: usize,
    east_bound: usize,
    rope: Semaphore<>,
    mutex: Mutex<()>
}

impl Baboons {

    /**
    * Initialize new Baboons
    */
    fn new(rope_limit: usize) -> Self {
        Self {
            west_bound: 0,
            east_bound: 0,
            rope: Semaphore::new(rope_limit),
            mutex: Mutex::new(())
        }
    }


     async fn cross_west(&mut self){
        self.mutex.lock(); // lock

        self.west_bound += 1;

        if(self.west_bound == 1) {
            self.rope.acquire();
            self.corss_canyon("Westbound");
        }

         self.mutex.lock(); // lock
        self.west_bound = 0;
         self.rope.add_permits(1);

    }

    async fn cross_east(&mut self){
        self.mutex.lock(); // lock

        self.east_bound += 1;

        if(self.east_bound == 1) {
            self.rope.acquire();
            self.corss_canyon("Eastbound");
        }

        self.mutex.lock(); // lock
        self.east_bound = 0;
        self.rope.add_permits(1);

    }


    async fn corss_canyon(&mut self, direction : &str) {
        println!("Baboon crossing {} started", direction);
        sleep(Duration::from_millis(200)); // Sleep 200 milliseconds
        println!("Baboon crossing {} completed", direction);
    }
}


pub async fn baboons_crossing(west : usize, east : usize){

    let  canyon = Arc::new(Baboons::new(1));

    let mut tasks = Vec::new();
    let baboons = Arc::clone(&canyon);
    tasks.push(
        tokio::spawn(
            async move {
                for _ in 0..west  {
                    baboons.clone().cross_east().await;
                }
            }
        )
    );

    let baboons = Arc::clone(&canyon);
    tasks.push(
        tokio::spawn(
            async move {
                for _ in 0..east  {
                    baboons.clone().cross_east().await;
                }
            }
        )
    );

    // Await all tasks to finish
    for task in tasks {
        task.await.unwrap();
    }

}


