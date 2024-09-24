
use async_lock::{Mutex, Semaphore};
/*
 Baboons Crossing a canyon.
 Baboons are crossing east to west and west to east from the end of
 a canyon and only one baboon is allowed on the thread.

 args
 west i8: number of baboons trying to cross from west to east
 east i8: number of baboons trying to cross from east to west
*/

pub struct Rope {
    west_bound: i8,
    east_bound: i8,
    semaphore: Semaphore

}

impl Rope {

    pub fn init(&mut self){
        self.east_bound = 0;
        self.west_bound = 0;
        self.semaphore = Semaphore::new(1);
    }
    pub fn acquire_eastbound(&mut self){
        self.east_bound = 1;
        self.semaphore.acquire();
    }

    pub fn acquire_westbound(&mut self){
        self.east_bound = 1;
        self.semaphore.acquire();
    }


}

pub struct Lock {
    mutex : Mutex<Rope>
}


impl Lock {
    pub fn east_bound(&mut self){
        let guard = self.mutex.lock();
        let rope = self.mutex.get_mut();

        if(self.mutex.get_mut().west_bound == 1){

        }
        else {

        }
    }
}


impl Rope {
    pub fn east_bound(&mut self) {
        if(self.west_bound > 0){
            //wait
        }
        else{

            //aquire lock
            self.east_bound  =1;
        }

    }
}
fn baboons_crossing(west : i8, east : i8){

    // Set allowed size
    let semaphore = Semaphore::new(1);

}


