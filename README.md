# Rust Blackjack Game

Welcome to the Rust Blackjack game! This is a simple console-based simulation of the Blackjack card game, written in Rust.

## Overview

This game follows the rules of Blackjack, where players aim to achieve a hand total as close to 21 as possible without exceeding it. The game is played against a computer dealer.

Running the Game
To run the game, use the following command:

cargo run


Game Rules
Each player is dealt two cards initially.
The goal is to get a hand total as close to 21 as possible without exceeding it.
Numbered cards are worth their face value, face cards (Jack, Queen, King) are worth 10, and Aces can be worth 1 or 11.
Players can choose to 'Hit' (draw a card) or 'Stand' (keep their current hand).
If a player's total exceeds 21, they lose. If the dealer's total exceeds 21, the player wins.
If neither the player nor the dealer exceeds 21, the one with the closest total to 21 wins.
A 'push' occurs if the player and dealer have the same total, and no one wins.
