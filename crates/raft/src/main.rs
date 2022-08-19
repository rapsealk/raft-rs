use std::thread;

enum ConsensusState {
    Follower,
    Candidate,
    Leader,
}

fn spawn_timer_thread() -> thread::JoinHandle<i32> {
    return thread::spawn(move || {
        println!("TimerThread!");
        return 0;
    });
}

fn main() {
    println!("Hello, world!");
    let initial_state = ConsensusState::Follower;
    let timer_thread_join_handler = spawn_timer_thread();

    match initial_state {
        ConsensusState::Follower => println!("Initial state: Follower"),
        ConsensusState::Candidate => println!("Initial state: Candidate"),
        ConsensusState::Leader => println!("Initial state: Leader"),
    }

    timer_thread_join_handler.join().expect("panic!");
}
