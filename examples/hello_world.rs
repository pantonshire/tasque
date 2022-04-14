use std::num::NonZeroU8;

use tasque::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum TaskId {
    Task1,
    Task2,
}

fn main() {
    let nonzero_5 = NonZeroU8::new(5).unwrap();

    let task1_schedule = Schedule::new_every_minute()
        .at_every_nth_second(nonzero_5);

    let task2_schedule = Schedule::new_every_minute()
        .at_every_nth_second_between(1..=59, nonzero_5)
        .unwrap();

    let mut scheduler = Scheduler::<TaskId, _>::new_utc();
    scheduler.insert(Task::builder(TaskId::Task1).at(task1_schedule).build());
    scheduler.insert(Task::builder(TaskId::Task2).at(task2_schedule).build());

    for id in scheduler.take(4) {
        run_task(id);
    }
}

fn run_task(id: TaskId) {
    match id {
        TaskId::Task1 => println!("Hello,"),
        TaskId::Task2 => println!("world!"),
    }
}
