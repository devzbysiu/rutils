use clokwerk::{Job, Scheduler, TimeUnits};
use log::{debug, info};
use std::thread;
use std::time::Duration;

pub fn schedule<F>(schedule: &[String], fun: F) -> !
where
    F: 'static + FnMut() + Send,
{
    let mut scheduler = Scheduler::new();
    let first_hour = schedule
        .first()
        .expect("At least one hour should be specified in run_at");
    info!("setting scheduler at: {}", first_hour);
    let mut job = scheduler.every(1.day()).at(first_hour);
    if schedule.len() > 1 {
        for time in &schedule[1..] {
            info!("setting scheduler at: {}", time);
            job = job.and_every(1.day()).at(time);
        }
    }
    job.run(fun);
    debug!("starting scheduler");
    run(&mut scheduler);
}

fn run(scheduler: &mut Scheduler) -> ! {
    let seconds = 600;
    loop {
        scheduler.run_pending();
        info!("no time yet, waiting {} seconds", seconds);
        thread::sleep(Duration::from_secs(seconds));
    }
}
