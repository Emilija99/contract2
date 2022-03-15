use std::convert::TryInto;

use schemars::{JsonSchema, _serde_json::ser::State};
use secret_toolkit::{
    serialization::Json,
    storage::{AppendStore, AppendStoreMut},
};
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    from_slice, to_vec, Api, CanonicalAddr, Extern, HandleResponse, Querier, StdError, StdResult,
    Storage, Uint128, HumanAddr,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage, Singleton, singleton, ReadonlySingleton, singleton_read};

use crate::msg::Move;

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Room {
    pub player1: Option<Player>,
    pub player2: Option<Player>,
    pub is_finished: bool,
    pub amount: Uint128,
    pub winner: Option<CanonicalAddr>,
    pub room_id: u64,
    pub room_title: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SnipState{
    pub addr:HumanAddr,
    pub hash:String
}




pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, SnipState> {
    singleton(storage, CONFIG_KEY)
}
pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, SnipState> {
    singleton_read(storage, CONFIG_KEY)
}

impl Room {
    pub fn new(room_id: u64, room_title: String) -> Room {
        Room {
            player1: None,
            player2: None,
            is_finished: false,
            amount: Uint128(0),
            winner: None,
            room_id,
            room_title,
        }
    }
    pub fn play(&mut self, player: Player, amount: Uint128) -> StdResult<HandleResponse> {
        if amount.is_zero() {
            return Err(StdError::GenericErr {
                msg: "Not enough coins".to_string(),
                backtrace: None,
            });
        }
        if self.is_finished {
            return Err(StdError::GenericErr {
                msg: "Game already finished".to_string(),
                backtrace: None,
            });
        }
        if self.player1 == None {
            self.player1 = Some(player);
            self.amount += amount;
            return Ok(HandleResponse::default());
        } else {
            if self.player1.clone().unwrap().address == player.address {
                return Err(StdError::GenericErr {
                    msg: "You can't play twice".to_string(),
                    backtrace: None,
                });
            }
            self.player2 = Some(player);
            self.amount += amount;
            self.is_finished = true;
            self.winner = Self::calculate_winner(
                self.player1.clone().unwrap(),
                self.player2.clone().unwrap(),
            );

            return Ok(HandleResponse::default());
        }
    }

    pub fn calculate_winner(player1: Player, player2: Player) -> Option<CanonicalAddr> {
        match (player1.move1, player2.move1) {
            (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissors, Move::Scissors) => None,
            (Move::Paper, Move::Rock)
            | (Move::Rock, Move::Scissors)
            | (Move::Scissors, Move::Paper) => Some(player1.address),
            (Move::Rock, Move::Paper)
            | (Move::Scissors, Move::Rock)
            | (Move::Paper, Move::Scissors) => Some(player2.address),
        }
    }
    pub fn get_room<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        room_id: u64,
    ) -> Result<Room, StdError> {
        let mut store = PrefixedStorage::new(b"/rooms/", &mut deps.storage);

        let test =
            AppendStoreMut::<Room, _, _>::attach_or_create_with_serialization(&mut store, Json)?;
        let room = test.iter().find(|x| x.as_ref().unwrap().room_id == room_id);
        room.ok_or(StdError::GenericErr {
            msg: "Room not found".to_string(),
            backtrace: None,
        })?
    }

    fn next_id<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
    ) -> Result<u64, StdError> {
        let total = from_slice::<u64>(&deps.storage.get(b"total").unwrap())?;
        deps.storage.set(b"total", &to_vec(&(total + 1))?);
        Ok(total)
    }

    pub fn create_room<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        room_title: String,
    ) -> Result<(), StdError> {
        let new_room = Self::new(Self::next_id(deps)?, room_title);
        let mut store = PrefixedStorage::new(b"/rooms/", &mut deps.storage);
        let mut test =
            AppendStoreMut::<Room, _, _>::attach_or_create_with_serialization(&mut store, Json)?;
        test.push(&new_room)?;
        Ok(())
    }

    pub fn update_room<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        room_id: u64,
        room: &Room,
    ) -> Result<(), StdError> {
        let mut store = PrefixedStorage::new(b"/rooms/", &mut deps.storage);
        let mut test =
            AppendStoreMut::<Room, _, _>::attach_or_create_with_serialization(&mut store, Json)?;
        let index = test
            .iter()
            .position(|r| r.unwrap().room_id == room_id)
            .ok_or(StdError::GenericErr {
                msg: "Room not found".to_string(),
                backtrace: None,
            })?;
        test.set_at(index.try_into().unwrap(), room)?;

        Ok(())
    }

    pub fn read_room<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        room_id: u64,
    ) -> Result<Room, StdError> {
        let store = ReadonlyPrefixedStorage::new(b"/rooms/", &deps.storage);

        let test = AppendStore::<Room, _, _>::attach_with_serialization(&store, Json).ok_or(
            StdError::GenericErr {
                msg: "Rooms not created".to_string(),
                backtrace: None,
            },
        )??;

        test.iter()
            .find(|x| x.as_ref().unwrap().room_id == room_id)
            .ok_or(StdError::GenericErr {
                msg: "Room not found".to_string(),
                backtrace: None,
            })?
    }

    pub fn get_rooms<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        page_num: u64,
        num_of_items: u64,
    ) -> Result<Vec<String>, StdError> {
        let store = ReadonlyPrefixedStorage::new(b"/rooms/", &deps.storage);

        let test = AppendStore::<Room, _, _>::attach_with_serialization(&store, Json).ok_or(
            StdError::GenericErr {
                msg: "Rooms not created".to_string(),
                backtrace: None,
            },
        )??;

        Ok(test
            .iter()
            .map(|x| x.as_ref().unwrap().room_title.clone())
            .skip(((page_num-1) * num_of_items).try_into().unwrap())
            .take(num_of_items.try_into().unwrap())
            .collect::<Vec<String>>())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Player {
    pub move1: Move,
    pub address: CanonicalAddr,
}
