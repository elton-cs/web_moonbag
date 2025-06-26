use bevy::prelude::*;
use std::collections::HashMap;
use starknet::core::types::Felt;

use super::types::*;

#[derive(Resource, Default, Debug)]
pub struct GameState {
    pub moon_rocks: HashMap<Felt, MoonRocks>,
    pub games: HashMap<(Felt, u32), Game>,
    pub game_counters: HashMap<Felt, GameCounter>,
    pub active_games: HashMap<Felt, ActiveGame>,
    pub orb_bag_slots: HashMap<(Felt, u32, u32), OrbBagSlot>,
    pub drawn_orbs: HashMap<(Felt, u32, u32), DrawnOrb>,
    pub shop_inventory: HashMap<(Felt, u32, u8, u8), ShopInventory>,
    pub purchase_history: HashMap<(Felt, u32, OrbType), PurchaseHistory>,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_from_model(&mut self, model: &DojoModel) {
        match model {
            DojoModel::MoonRocks(moon_rocks) => {
                self.moon_rocks.insert(moon_rocks.player, moon_rocks.clone());
            }
            DojoModel::Game(game) => {
                self.games.insert((game.player, game.game_id), game.clone());
            }
            DojoModel::GameCounter(counter) => {
                self.game_counters.insert(counter.player, counter.clone());
            }
            DojoModel::ActiveGame(active) => {
                self.active_games.insert(active.player, active.clone());
            }
            DojoModel::OrbBagSlot(slot) => {
                self.orb_bag_slots.insert(
                    (slot.player, slot.game_id, slot.slot_index),
                    slot.clone(),
                );
            }
            DojoModel::DrawnOrb(drawn) => {
                self.drawn_orbs.insert(
                    (drawn.player, drawn.game_id, drawn.draw_index),
                    drawn.clone(),
                );
            }
            DojoModel::ShopInventory(shop) => {
                self.shop_inventory.insert(
                    (shop.player, shop.game_id, shop.level, shop.slot_index),
                    shop.clone(),
                );
            }
            DojoModel::PurchaseHistory(purchase) => {
                self.purchase_history.insert(
                    (purchase.player, purchase.game_id, purchase.orb_type.clone()),
                    purchase.clone(),
                );
            }
        }
    }

    pub fn get_player_moon_rocks(&self, player: &Felt) -> Option<&MoonRocks> {
        self.moon_rocks.get(player)
    }

    pub fn get_player_active_game(&self, player: &Felt) -> Option<&Game> {
        self.active_games
            .get(player)
            .and_then(|active| self.games.get(&(active.player, active.game_id)))
    }

    pub fn get_game(&self, player: &Felt, game_id: u32) -> Option<&Game> {
        self.games.get(&(*player, game_id))
    }

    pub fn get_game_orb_slots(&self, player: &Felt, game_id: u32) -> Vec<&OrbBagSlot> {
        self.orb_bag_slots
            .iter()
            .filter_map(|((p, g, _), slot)| {
                if p == player && *g == game_id {
                    Some(slot)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_game_drawn_orbs(&self, player: &Felt, game_id: u32) -> Vec<&DrawnOrb> {
        self.drawn_orbs
            .iter()
            .filter_map(|((p, g, _), orb)| {
                if p == player && *g == game_id {
                    Some(orb)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_shop_inventory(&self, player: &Felt, game_id: u32, level: u8) -> Vec<&ShopInventory> {
        self.shop_inventory
            .iter()
            .filter_map(|((p, g, l, _), shop)| {
                if p == player && *g == game_id && *l == level {
                    Some(shop)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.moon_rocks.clear();
        self.games.clear();
        self.game_counters.clear();
        self.active_games.clear();
        self.orb_bag_slots.clear();
        self.drawn_orbs.clear();
        self.shop_inventory.clear();
        self.purchase_history.clear();
    }
}