# Rust Blackjack Game

Welcome to the Rust Blackjack game! This is a simple console-based simulation of the Blackjack card game, written in Rust.

## Overview

This game follows the rules of Blackjack, where players aim to achieve a hand total as close to 21 as possible without exceeding it. The game is played against a computer dealer.

## Running the Game
To run the game, use the following command:

cargo run


## Game Rules

1. **Each player is dealt two cards initially.**
   Players start the game with two cards in their hand.

2. **The goal is to get a hand total as close to 21 as possible without exceeding it.**
   Aim to have a hand total that is as close to 21 as possible without going over.

3. **Numbered cards are worth their face value, face cards (Jack, Queen, King) are worth 10, and Aces can be worth 1 or 11.**
   - Numbered cards (2-10) are worth their face value.
   - Face cards (Jack, Queen, King) are each worth 10 points.
   - Aces can be counted as either 1 or 11 points.

4. **Players can choose to 'Hit' (draw a card) or 'Stand' (keep their current hand).**
   Players have the option to draw an additional card ('Hit') or keep their current hand ('Stand').

5. **If a player's total exceeds 21, they lose. If the dealer's total exceeds 21, the player wins.**
   - If a player's total goes over 21 ('bust'), they lose the round.
   - If the dealer's total goes over 21, the player wins the round.

6. **If neither the player nor the dealer exceeds 21, the one with the closest total to 21 wins.**
   If both the player and dealer have totals below 21, the one closest to 21 wins the round.

7. **A 'push' occurs if the player and dealer have the same total, and no one wins.**
   If the player and dealer have the same total, it results in a 'push,' and no one wins the round.

