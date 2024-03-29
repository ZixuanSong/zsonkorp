use crate::deck::Deck;
use crate::config::CtaWagerType;
use crate::config;
use crate::game::{Game, GameType};
use anyhow::Result;
use crate::payout::Payout;
use crate::state::State;
use crate::state::GameState::*;
use crate::transition::Transition;

struct Cta {
    deck_pool: Vec<Deck>,
    config: config::Cta,
    enforce_optimal_cut: bool,
    state: State
}

impl Cta {
    pub fn new(config: config::Cta) -> Result<Self> {
        let mut game = Cta {
            deck_pool: vec![Deck::default()],
            config,
            state: State::Game(Setup),
            enforce_optimal_cut: false
        };

        game.apply_config()?;

        Ok(game)
    }

    fn apply_config(&mut self) -> Result<()> {

        // optimal cut is enforced when there are reverse wagers
        if self.config.get_base_config().get_wagers()
            .iter()
            .flat_map( |(_, wagers)| wagers.iter())
            .any( |wager| *wager.get_wager_type() == CtaWagerType::Reverse) {

                self.enforce_optimal_cut = true;
        }

        Ok(())
    }
}

impl Game for Cta {
    fn get_type(&self) -> GameType {
        GameType::Cta
    }

    fn transition(&mut self, transition: Transition) -> Result<()> {
        todo!()
    }

    fn get_valid_transitions(&self) -> Vec<Transition> {
        todo!()
    }

    fn get_payout(&self) -> Result<Vec<Payout>> {
        todo!()
    }
}