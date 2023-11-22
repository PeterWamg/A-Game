use std::io;

// Define the structure for a card
#[derive(Debug)]
struct Card {
    value: u8,
}

// Define the structure for a player
#[derive(Debug)]
struct Player {
    hand: Vec<Card>,
}

// Calculate the total value of a player's hand
fn calculate_hand_value(hand: &Vec<Card>) -> u8 {
    let mut total_value = 0;
    let mut num_aces = 0;

    for card in hand {
        if card.value == 1 {
            num_aces += 1;
            total_value += 1;
        } else {
            total_value += card.value;
        }
    }

    // If there are Aces and counting them as 11 doesn't bust the hand, count them as 11
    while num_aces > 0 && total_value + 10 <= 21 {
        total_value += 10;
        num_aces -= 1;
    }

    total_value
}


fn deal_card() -> Card {
    let value = rand::random::<u8>() % 10 + 1; // Generate a random number between 1 and 10
    Card { value }
}

fn main() {
    println!("Welcome to the Blackjack game!");
    println!("Here are the rules:");

    println!("1. Each player is dealt two cards initially.");
    println!("2. The goal is to get a hand total as close to 21 as possible without exceeding it.");
    println!("3. Numbered cards are worth their face value, face cards (Jack, Queen, King) are worth 10, and Aces can be worth 1 or 11.");
    println!("4. Players can choose to 'Hit' (draw a card) or 'Stand' (keep their current hand).");
    println!("5. If a player's total exceeds 21, they lose. If the dealer's total exceeds 21, the player wins.");
    println!("6. If neither the player nor the dealer exceeds 21, the one with the closest total to 21 wins.");
    println!("7. A 'push' occurs if the player and dealer have the same total, and no one wins.");
    let mut player_wins = 0;
    let mut dealer_wins = 0;

    loop {
        let mut player = Player { hand: vec![] };
        let mut dealer = Player { hand: vec![] };

        // Deal two cards to the player and dealer
        player.hand.push(deal_card());
        player.hand.push(deal_card());

        dealer.hand.push(deal_card());
        dealer.hand.push(deal_card());

        println!("Player's hand: {:?}", player.hand);
        println!("Dealer's first card: {:?}", dealer.hand[0]);

        // Player's turn
        while calculate_hand_value(&player.hand) <= 21 {
            println!("Your total points: {}", calculate_hand_value(&player.hand));
            println!("Do you want to 'Hit'? (y/n)");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error reading input");

            match input.trim() {
                "y" => {
                    player.hand.push(deal_card());
                    println!("You drew a card: {:?}", player.hand.last());
                }
                "n" => break,
                _ => {
                    println!("Invalid input. Please enter 'y' or 'n'");
                    // Here you can add logic to clear invalid input or provide other handling
                }
            }
        }

        // Dealer's turn
        while calculate_hand_value(&dealer.hand) <= 17 {
            dealer.hand.push(deal_card());
        }

        println!("Your final hand: {:?}", player.hand);
        println!("Your final points: {}", calculate_hand_value(&player.hand));
        println!("Dealer's final hand: {:?}", dealer.hand);
        println!("Dealer's final points: {}", calculate_hand_value(&dealer.hand));

        // Determine the winner
        let player_score = calculate_hand_value(&player.hand);
        let dealer_score = calculate_hand_value(&dealer.hand);

        if player_score > 21 || (dealer_score <= 21 && dealer_score >= player_score) {
            println!("You lose!");
            dealer_wins += 1;
        } else if dealer_score > 21 || player_score > dealer_score {
            println!("You win!");
            player_wins += 1;
        } else {
            println!("It's a tie!");
        }

        // Output current win counts
        println!("Current wins: Player {} - Dealer {}", player_wins, dealer_wins);

        // Ask if the player wants to continue playing
        println!("Do you want to continue playing? (y/n)");
        let mut continue_input = String::new();
        io::stdin().read_line(&mut continue_input).expect("Error reading input");

        if continue_input.trim() != "y" {
            break;
        }
    }

    // Inform the player of the final results when the game is over
    if player_wins > dealer_wins {
        println!("Final result: You won more games!");
    } else if dealer_wins > player_wins {
        println!("Final result: Dealer won more games!");
    } else {
        println!("Final result: It's a tie!");
    }
}
