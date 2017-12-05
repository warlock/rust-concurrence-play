pub mod start {
  use std::thread;
  use std::sync::mpsc;
  use std::time::Duration;

  pub fn run() {
    println!("start play");
  }

  pub fn first() {
    let handle = thread::spawn(||{
      for i in 1..100{
        println!("A: {}", i);
      }
    });

    let v = vec!["A","B","C"];
    let handle2 = thread::spawn(||{
      for i in v {
        println!("B: {}", i);
      }
    });
    let _1 = handle.join();
    let _2 = handle2.join();
    println!("END");
  }

  pub fn message() {
    let (tx,rx) = mpsc::channel();
  
    let _h = thread::spawn(move ||{
      thread::sleep(Duration::new(5,0));
      let val = String::from("A");
      tx.send(val).unwrap();
    });
  
    loop {
      if let Ok(msg) = rx.try_recv() {
        println!("{}",msg);
        break;
      }
    }
  }
}