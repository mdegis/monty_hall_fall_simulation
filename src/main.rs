extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

struct SimulationResult {
    win: bool,
    switch: bool,
    exploded: bool,
}

fn pick_a_door_that_isnt<R: Rng>(a: u32, b: u32, rng: &mut R) -> u32 {
    rand::sample(rng, (0..3).filter(|&x| x != a && x != b), 1)[0]
}

fn simulate_guess<R: Rng>(rng: &mut R) -> SimulationResult {
    let doors = Range::new(0, 3);
    let car = doors.ind_sample(rng);
    let mut player_choice = doors.ind_sample(rng);
    let host_opens = pick_a_door_that_isnt(player_choice, player_choice, rng);

    if host_opens == car {
        SimulationResult {
            win: false,
            switch: false,
            exploded: true,
        }
    } else {
        let switch = rng.gen();
        if switch {
            player_choice = pick_a_door_that_isnt(host_opens, player_choice, rng);
        }
        SimulationResult {
            win: player_choice == car,
            switch: switch,
            exploded: false,
        }
    }
}

fn simulate_knew<R: Rng>(rng: &mut R) -> SimulationResult {
    let doors = Range::new(0, 3);
    let car = doors.ind_sample(rng);
    let mut player_choice = doors.ind_sample(rng);
    let host_opens = pick_a_door_that_isnt(car, player_choice, rng);
    let switch = rng.gen();
    if switch {
        player_choice = pick_a_door_that_isnt(host_opens, player_choice, rng);
    }
    SimulationResult {
        win: player_choice == car,
        switch: switch,
        exploded: false,
    }
}

fn knew_the_answer() {
    let num_simulations = 10000;
    let mut rng = rand::thread_rng();
    let (mut switch_wins, mut switch_losses) = (0, 0);
    let (mut keep_wins, mut keep_losses) = (0, 0);
    println!("Running {} simulations where the host precisely avoids the car...",
             num_simulations);
    for _ in 0..num_simulations {
        let result = simulate_knew(&mut rng);
        match (result.win, result.switch) {
            (true, true) => switch_wins += 1,
            (true, false) => keep_wins += 1,
            (false, true) => switch_losses += 1,
            (false, false) => keep_losses += 1,
        }
    }
    let total_switches = switch_wins + switch_losses;
    let total_keeps = keep_wins + keep_losses;
    println!("Switched door {} times with {} wins and {} losses",
             total_switches,
             switch_wins,
             switch_losses);
    println!("Kept our choice {} times with {} wins and {} losses",
             total_keeps,
             keep_wins,
             keep_losses);
    println!("Estimated chance to win if we switch (should be 0.666): {:.3}",
             switch_wins as f32 / total_switches as f32);
    println!("Estimated chance to win if we don't (should be 0.333): {:.3}",
             keep_wins as f32 / total_keeps as f32);
}

fn random_guess() {
    let num_simulations = 10000;
    let mut rng = rand::thread_rng();
    let (mut switch_wins, mut switch_losses) = (0, 0);
    let (mut keep_wins, mut keep_losses) = (0, 0);
    let mut explosions = 0;
    println!("Running {} simulations where the host makes a random guess...",
             num_simulations);
    println!("If they pick the car, the universe explodes and we discard the trial");
    for _ in 0..num_simulations {
        let result = simulate_guess(&mut rng);
        if result.exploded {
            explosions += 1;
            continue;
        }
        match (result.win, result.switch) {
            (true, true) => switch_wins += 1,
            (true, false) => keep_wins += 1,
            (false, true) => switch_losses += 1,
            (false, false) => keep_losses += 1,
        }
    }
    let total_switches = switch_wins + switch_losses;
    let total_keeps = keep_wins + keep_losses;
    println!("Exploded {} times", explosions);
    println!("Switched door {} times with {} wins and {} losses",
             total_switches,
             switch_wins,
             switch_losses);
    println!("Kept our choice {} times with {} wins and {} losses",
             total_keeps,
             keep_wins,
             keep_losses);
    println!("Estimated chance to explode (should be 0.333): {:.3}",
             explosions as f32 / num_simulations as f32);
    println!("Estimated chance to win if we switch (should be 0.5): {:.3}",
             switch_wins as f32 / total_switches as f32);
    println!("Estimated chance to win if we don't (should be 0.5): {:.3}",
             keep_wins as f32 / total_keeps as f32);
}

fn main() {
    random_guess();
    println!("----");
    knew_the_answer();
}
