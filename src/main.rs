use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));

	let mut handles=vec![];
	for i in 0..10 {
		let vcopy_to_move=v.clone();
		let handle = thread::spawn(move || {
				for x in 0..100 {
					thread::sleep(Duration::from_millis(5));
					let mut v_thread = vcopy_to_move.lock().unwrap();
					v_thread.push(x+100*i);
				}
				});
		handles.push(handle);
	}
	for handle in handles {
		let _ = handle.join();
	}
    println!("v: {v:?}");
}

