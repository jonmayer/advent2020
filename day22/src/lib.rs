#![allow(dead_code)]

use log::*;

use std::collections::HashSet;
use std::collections::VecDeque;

type Card = i32;
type Deck = VecDeque<Card>;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Mode {
    Part1,
    Part2,
}

fn deck_to_str(deck: &Deck) -> String {
    let tmp: Vec<String> = deck.iter().map(|x| x.to_string() as String).collect();
    return tmp.join(", ");
}

#[derive(Clone, Default, Debug, Hash, Eq, PartialEq)]
struct Game {
    decks: [Deck; 2],
}

impl Game {
    fn new() -> Game {
        Default::default()
    }

    fn parse_deck(&mut self, deck_index: usize, text: &str) {
        self.decks[deck_index] = text
            .lines()
            .skip(1) // discard header.
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    fn parse(&mut self, text: &str) {
        let parts: Vec<&str> = text.split("\n\n").collect();
        self.parse_deck(0, parts[0]);
        self.parse_deck(1, parts[1]);
    }

    fn get_winner(&self) -> usize {
        return if self.decks[0].len() == 0 { 1 } else { 0 };
    }

    fn do_round(&mut self, game_number: usize, mode: Mode) -> bool {
        debug!("Player 0 deck: {}", deck_to_str(&self.decks[0]));
        debug!("Player 1 deck: {}", deck_to_str(&self.decks[1]));
        if self.decks.iter().any(|deck| deck.len() == 0) {
            return false;
        }
        let cards: Vec<Card> = self
            .decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();

        if mode == Mode::Part2 {
            if (self.decks[0].len() >= cards[0] as usize)
                && (self.decks[1].len() >= cards[1] as usize)
            {
                let mut subgame: Game = Game::new();
                for index in 0..(cards[0] as usize) {
                    subgame.decks[0].push_back(self.decks[0][index]);
                }
                for index in 0..(cards[1] as usize) {
                    subgame.decks[1].push_back(self.decks[1][index]);
                }
                debug!("Player 0 subdeck: {}", deck_to_str(&subgame.decks[0]));
                debug!("Player 1 subdeck: {}", deck_to_str(&subgame.decks[1]));
                subgame.do_game(game_number + 1, mode);
                let winner = subgame.get_winner();
                if winner == 0 {
                    self.decks[0].push_back(cards[0]);
                    self.decks[0].push_back(cards[1]);
                } else {
                    self.decks[1].push_back(cards[1]);
                    self.decks[1].push_back(cards[0]);
                }
                return true;
            }
        }

        assert!(cards[0] != cards[1]);
        if cards[0] > cards[1] {
            debug!("Card {} vs Card {}: Player 0 wins", cards[0], cards[1]);
            self.decks[0].push_back(cards[0]);
            self.decks[0].push_back(cards[1]);
        } else {
            debug!("Card {} vs Card {}: Player 1 wins", cards[0], cards[1]);
            self.decks[1].push_back(cards[1]);
            self.decks[1].push_back(cards[0]);
        }

        return true;
    }

    fn do_game(&mut self, game_number: usize, mode: Mode) {
        debug!("GAME {} started.", game_number);
        let mut history: HashSet<Game> = HashSet::new();
        while self.do_round(game_number, mode) {
            if history.contains(self) {
                // player 1 (our 0) is the winner
                self.decks[1].clear();
                debug!(
                    "game {} done, repeat state winner={}",
                    game_number,
                    self.get_winner()
                );
                return;
            }
            history.insert(self.clone());
        }
        debug!("game {} done, winner={}", game_number, self.get_winner());
    }

    fn score(&self) -> i64 {
        let winner: &Deck = if self.decks[0].len() == 0 {
            &self.decks[1]
        } else {
            &self.decks[0]
        };
        let score: i64 = winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &v)| v as i64 * (1 + i as i64))
            .sum();
        return score;
    }
}

#[cfg(test)]
mod tests {
    extern crate simplelog;
    use super::*;

    use simplelog::*;
    use std::fs;

    #[test]
    fn test_testcase() {
        let _ = TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed);
        let contents =
            fs::read_to_string("testcase.txt").expect("Something went wrong reading the file");
        info!("Loaded {} bytes", contents.len());
        let mut game = Game::new();
        game.parse(&contents);
        game.do_game(1, Mode::Part1);
        debug!("game state = {:?}", &game);
        assert_eq!(game.decks[0].len(), 0);
        assert_eq!(game.decks[1], [3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        info!("score={}", game.score());
    }

    #[test]
    fn test_part1() {
        let _ = TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed);
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");
        info!("Loaded {} bytes", contents.len());
        let mut game = Game::new();
        game.parse(&contents);
        game.do_game(1, Mode::Part1);
        debug!("game state = {:?}", &game);
        info!("score={}", game.score());
    }

    #[test]
    fn test_part2_testcase() {
        let _ = TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed);
        let contents =
            fs::read_to_string("testcase.txt").expect("Something went wrong reading the file");
        info!("Loaded {} bytes", contents.len());
        let mut game = Game::new();
        game.parse(&contents);
        game.do_game(1, Mode::Part2);
        debug!("game state = {:?}", &game);
        info!("score={}", game.score());
        assert_eq!(291, game.score());
    }

    #[test]
    fn test_part2() {
        let _ = TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed);
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");
        info!("Loaded {} bytes", contents.len());
        let mut game = Game::new();
        game.parse(&contents);
        game.do_game(1, Mode::Part2);
        debug!("game state = {:?}", &game);
        info!("score={}", game.score());
    }
} // mod tests
