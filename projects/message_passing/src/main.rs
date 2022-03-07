use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    // `tx` = transmitter (Sender<T>)
    // `rx` = receiver (Reveiver<T>)
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter `tx` for use in a 2nd spawned thread
    let tx1 = tx.clone();

    thread::spawn(move || {
        let val = String::from("hi");
        // `tx` ownership is moved into this thread
        tx.send(val).unwrap();

        thread::sleep(Duration::from_millis(3000));
        tx.send("hello again...".to_owned()).unwrap();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();

            // Thread sleeps for 1 second
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Using 2nd transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("here"),
            String::from("are"),
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("spawned thread #2"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs_f64(1.2));
        }
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // recv() call will block until it receives "hello again..." after 3 thread sleeps for 3 seconds.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Treat `rx` as an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}
