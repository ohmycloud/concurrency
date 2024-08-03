use mpsc::metrics::Metrics;
use rand::Rng;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;

fn task_worker(idx: usize, mut metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics
            .inc(format!("call.thread.worker.{}", idx))
            .expect("inc error");
    });
}

fn request_worker(metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{}", page));
    });
}
fn main() {
    let mut metrics = Metrics::new();
    metrics.inc("req.page.1");
    metrics.inc("call.thread.worker.1");

    for i in 0..100 {
        metrics.inc("call.thread.worker.1").expect("inc error");
    }

    println!("{:?}", metrics.snapshot());

    // start N workers and M requesters
    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }

    for idx in 0..M {
        request_worker(metrics.clone())
    }

    loop {
        thread::sleep(Duration::from_millis(1000));
        println!("{:?}", metrics.snapshot());
    }
}
