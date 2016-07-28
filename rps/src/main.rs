use std::io;
extern crate rand;
use rand::Rng;

enum Hand{
	Rock,
	Paper,
	Scissors,
	Quit
}

struct Stats{
	num_wins: i32,
	num_losses: i32,
	num_ties: i32,
	num_rock: i32,
	num_paper: i32,
	num_scissors: i32
}

fn main() {

	let mut player: Hand;
	let mut comp: Hand;
	let mut stats = Stats{
		num_wins: 0,
		num_losses: 0,
		num_ties: 0,
		num_rock: 0,
		num_paper: 0,
		num_scissors: 0
	};

	println!("Welcome to Rock/Paper/Scissors DEATHMATCH\n");
	
	// main loop driving the program
	loop{
		player = take_input();
		// if player == Hand::Quit{
		// 	break;
		// }
		// comp = comp_choice();
		// I broke up the battle logic into 3 separate methods because otherwise it would probably look gross and messy
		match player{
			Hand::Rock => {
				comp = comp_choice();
				fight_as_rock(&comp, &mut stats);
			}
			Hand::Paper => {
				comp = comp_choice();
				fight_as_paper(&comp, &mut stats);
			}
			Hand::Scissors => {
				comp = comp_choice();
				fight_as_scissors(&comp, &mut stats);
			}
			Hand::Quit => break // This should already be taken care of and therefore never hit this line
		}
	}
	
	display_stats(&stats);

}

fn take_input() -> Hand{
	let mut input = String::new();
	loop{
		println!("Please enter choice of r, p, s (or q to quit)");
		io::stdin().read_line(&mut input).expect("Failed to read line");
		// println!("Your input is {}", input);
		match input.trim(){
			"r" => {
				println!("You chose Rock.");
				return Hand::Rock;
			},
			"p" => {
				println!("You chose Paper.");
				return Hand::Paper;
			},
			"s" => {
				println!("You chose Scissors.");
				return Hand::Scissors;
			},
			"q" => {
				println!("Bye!");
				return Hand::Quit;
			},
			_ => {
				println!("That is not one of the options.");
				input = String::new();
			}
		}
	}
}

fn comp_choice() -> Hand{
	let choice = rand::thread_rng().gen_range(0, 3);
		// 0 = rock, 1 = paper, 2 = scissors 
	if choice == 0{
		println!("Computer chose Rock");
		Hand::Rock
	}else if choice == 1{
		println!("Computer chose Paper");
		Hand::Paper
	}else{
		println!("Computer chose Scissors");
		Hand::Scissors
	}
}

// The player has chosen rock
fn fight_as_rock (comp: &Hand, stats: &mut Stats){
	stats.num_rock+=1;
	match comp{
		&Hand::Rock => {
			stats.num_ties+=1;
			println!("You tied!\n");
		},
		&Hand::Paper => {
			stats.num_losses+=1;
			println!("You lost!\n");
		},
		&Hand::Scissors => {
			stats.num_wins+=1;
			println!("You won!\n");
		},
		&Hand::Quit => {} // This one should never happen
	}
}

// The player has chosen paper
fn fight_as_paper (comp: &Hand, stats: &mut Stats){
	stats.num_paper+=1;
	match comp{
		&Hand::Rock => {
			stats.num_wins+=1;
			println!("You won!\n");
		},
		&Hand::Paper => {
			stats.num_ties+=1;
			println!("You tied!\n");
		},
		&Hand::Scissors => {
			stats.num_losses+=1;
			println!("You lost!\n");
		},
		&Hand::Quit => {} // This one should never happen
	}
}

// The player has chosen scissors
fn fight_as_scissors (comp: &Hand, stats: &mut Stats){
	stats.num_scissors+=1;
	match comp{
		&Hand::Rock => {
			stats.num_losses+=1;
			println!("You lost!\n");
		},
		&Hand::Paper => {
			stats.num_wins+=1;
			println!("You won!\n");
		},
		&Hand::Scissors => {
			stats.num_ties+=1;
			println!("You tied!\n");
		},
		&Hand::Quit => {} // This one should never happen
	}
}

fn display_stats(stats: &Stats){
	let mut total_games: f32 = stats.num_wins as f32 + stats.num_losses as f32 + stats.num_ties as f32;

	// This is just to make sure that NaN is never displayed
	// This doesn't actually affect any of the stats or output, since if it's 0, all the other stats will be 0 anyway
	if total_games == 0.0{
		total_games = 1.0;
	}

	let percent_win = 100.00 * stats.num_wins as f32 / total_games;
	let percent_loss = 100.00 * stats.num_losses as f32 / total_games;
	let percent_tie = 100.00 * stats.num_ties as f32 / total_games;

	println!("\nPlayer stats:");
	println!("Wins: {} ({:.2}%)", stats.num_wins, percent_win);
	println!("Losses: {} ({:.2}%)", stats.num_losses, percent_loss);
	println!("Ties: {} ({:.2}%)", stats.num_ties, percent_tie);
	println!("Rocks: {}", stats.num_rock);
	println!("Paper: {}", stats.num_paper);
	println!("Scissors: {}", stats.num_scissors);

	println!("\n------------------\n");
}
