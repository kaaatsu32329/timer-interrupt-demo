use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

fn main() {
    let timer_100milli_flag = Arc::new(Mutex::new(false));
    let cloned_flag = timer_100milli_flag.clone();

    thread::spawn(move || {
        let delta_time = 0.1;
        let mut counter = 0;
        let started_time = Instant::now();
        loop {
            if (started_time.elapsed().as_secs_f64() - delta_time * counter as f64) > delta_time {
                counter += 1;

                let mut flag = cloned_flag.lock().unwrap();
                *flag = true;
            }
        }
    });

    let started_time = Instant::now();

    loop {
        let mut flag = timer_100milli_flag.lock().unwrap();
        if *flag {
            println!(
                "100 milli seconds passed!, elapsed time: {:10.4}",
                started_time.elapsed().as_secs_f64()
            );
            *flag = false;
        }
    }
}
