///thread_a locks resource a
///then thread_b locks resource b
///thread_a needs resource b to continue and release a
///but for thread_b to release b it needs to have access to resource a

use std::sync::Mutex;
use std::thread;
use lazy_static::lazy_static;
lazy_static! {
    static ref mutex_a: Mutex<()> = Mutex::new(());//() needs 0 bytes of ram
    static ref mutex_b: Mutex<()> = Mutex::new(());
}

fn main() {
    let thread_a = thread::spawn(|| {
        let lock_a = mutex_a.lock();
        thread::sleep_ms(100);//makes it easier to dead lock behavior but not nessasary to deadlock
        let lock_b = mutex_b.lock();
    });
    let thread_b = thread::spawn(|| {
        let lock_b = mutex_b.lock();
        let lock_a = mutex_a.lock();
    });
    thread_a.join().unwrap();//Should not be able to error
    thread_b.join().unwrap();
}