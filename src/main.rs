use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rayon::prelude::*;
use rayon::Scope;

fn main() {
	std_threads();
	rayon_threads();
	rayon_spawn();
}


// STD THREADS
fn std_threads() {

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

// RAYON PAR_ITER
type Pixel = (u8, u8, u8);  // Red, Green, Blue
type Image = Vec<Vec<Pixel>>;

fn apply_filter(pixel: &Pixel) -> Pixel {
    let (r, g, b) = *pixel;

    // Compute the average of the red, green and blue values (use integer arithmetic)
    let grey = (r as u16 + g as u16 + b as u16) / 3;

    // Return a new pixel where all color channels have the grey value
    // The as u8 cast is safe because the average of three u8 values will always be within u8 range.
    (grey as u8, grey as u8, grey as u8)
}
fn rayon_threads() {
    let image: Image = vec![vec![(10, 20, 30); 800]; 600];

    let processed_image: Image = image.par_iter()
        .map(|row| {
            row.iter()
               .map(|pixel| apply_filter(pixel))
               .collect()
        })
        .collect();

    println!("Processed image size: {}x{}", processed_image.len(), processed_image[0].len());
	println!("pixel sample [0][0]: {:?}",processed_image[0][0]);
}

// RAYON SPAWN

fn rayon_spawn() {
	let mut v : Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(vec![vec![];10]));

	rayon::scope(|s: &Scope| {
		for i in 0..10 {
			let i_move=i;
			let v_rayon=v.clone();
			s.spawn( move |_s| {
					let iter = (0..100).enumerate().map(|(_,num)| { thread::sleep(Duration::from_millis(5)) ; num+i_move*100}); 
					let mut v_thread = v_rayon.lock().unwrap();
					v_thread[i_move]=Vec::from_iter(iter);
					});
		}
	});
    println!("v: {v:?}");
}
