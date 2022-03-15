use cosmwasm_std::{
    from_slice, to_binary, to_vec, Api, BankMsg, Binary, Coin, CosmosMsg, Env, Extern,
    HandleResponse, HumanAddr, InitResponse, Querier, StdResult, Storage, Uint128, WasmMsg, from_binary,
};

use crate::msg::{
    AmountResponse, HandleMsg, InitMsg, Move, QueryMsg, RoomsResponse, Snip20Msg, WinnerResponse,PlayMsg
};
use crate::state::{config, Player, Room, SnipState, config_read};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    deps.storage.set(b"total", &to_vec(&0)?);
    let snip_state=SnipState{
        //known_snip_20:vec![]
        addr:HumanAddr::default(),
        hash:String::new()
    };
    config(&mut deps.storage).save(&snip_state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Play { move1, room_id } => try_play(deps, env, move1, room_id),
        HandleMsg::CreateRoom { room_title } => try_create(deps, env, room_title),
        HandleMsg::Register { reg_addr, reg_hash } => try_register(deps, env, reg_addr, reg_hash),
        HandleMsg::Receive {
            sender,
            from,
            amount,
            memo,
            msg,
        } => try_receive(deps, env, sender, from, amount, msg),
    }
}

pub fn try_create<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    room_title: String,
) -> StdResult<HandleResponse> {
    Room::create_room(deps, room_title)?;

    Ok(HandleResponse::default())
}

pub fn try_play<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    move1: Move,
    room_id: u64,
) -> StdResult<HandleResponse> {
    let mut room = Room::get_room(deps, room_id)?;
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    room.play(
        Player {
            move1,
            address: sender_address_raw,
        },
        calculate_amount(&env.message.sent_funds),
    )?;
    Room::update_room(deps, room_id, &room)?;
    if room.is_finished {
        return Ok(send_tokens(
            env.contract.address,
            deps.api.human_address(&room.winner.clone().unwrap())?,
            room.amount,
        ));
    }

    Ok(HandleResponse::default())
}

fn send_tokens(
    from_address: HumanAddr,
    to_address: HumanAddr,
    total_amount: Uint128,
) -> HandleResponse {
    HandleResponse {
        data: None,
        log: vec![],
        messages: vec![cosmwasm_std::CosmosMsg::Bank(BankMsg::Send {
            from_address,
            to_address,
            amount: vec![Coin {
                amount: total_amount,
                denom: String::from("uscrt"),
            }],
        })],
    }
}

pub fn try_register<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    reg_addr: HumanAddr,
    reg_hash: String,
) -> StdResult<HandleResponse> {
    let mut conf = config(&mut deps.storage);
    let mut snip_state = conf.load()?;
    /*if !snip_state.known_snip_20.contains(&reg_addr) {
        snip_state.known_snip_20.push(reg_addr.clone());
    }*/
    snip_state.addr=reg_addr.clone();
    snip_state.hash=reg_hash.clone();
    conf.save(&snip_state)?;
    let msg = to_binary(&Snip20Msg::register_receive(env.contract_code_hash))?;

    let message = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: reg_addr,
        callback_code_hash: reg_hash,
        msg,
        send: vec![],
    });

    Ok(HandleResponse {
        messages: vec![message],
        log: vec![],
        data: None,
    })
}

pub fn try_receive<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _sender: HumanAddr,
    from: HumanAddr,
    amount: Uint128,
    msg: Binary,
)-> StdResult<HandleResponse> {

    let msg: PlayMsg = from_binary(&msg)?;
    let mut room = Room::get_room(deps, msg.room_id)?;
    let sender_address_raw=deps.api.canonical_address(&from)?;
    room.play(
        Player {
            move1:msg.move1,
            address: sender_address_raw,
           
        },
        amount,
    )?;
    Room::update_room(deps, msg.room_id, &room)?;
    if room.is_finished {
        /*return Ok(send_tokens(
            env.contract.address,
            deps.api.human_address(&room.winner.clone().unwrap())?,
            room.amount,
        ));*/
        let msg=to_binary(&Snip20Msg::transfer(deps.api.human_address(&room.winner.unwrap())?.to_string(), room.amount))?;
        let  conf = config_read(& deps.storage);
        let  snip_state = conf.load()?;
        //let addr=snip_state.known_snip_20.get(0).unwrap();
        let message=CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: snip_state.addr,
         
            msg,
            send: vec![],
            callback_code_hash:snip_state.hash,
        });
        return Ok(HandleResponse{
            messages:vec![message],
            log: vec![],
            data: None,
            
        })
    }

    Ok(HandleResponse::default())

}
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Winner { room_id } => to_binary(&query_winner(deps, room_id)?),
        QueryMsg::Amount { room_id } => to_binary(&query_amount(deps, room_id)?),
        QueryMsg::Rooms {
            page_num,
            num_of_items,
        } => to_binary(&query_rooms(deps, page_num, num_of_items)?),
    }
}

fn query_winner<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    room_id: u64,
) -> StdResult<WinnerResponse> {
    let room = Room::read_room(deps, room_id)?;

    let winner = if let Some(addr) = room.winner {
        deps.api.human_address(&addr)?.to_string()
    } else {
        "No winner".to_string()
    };
    Ok(WinnerResponse { winner })
}
fn query_amount<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    room_id: u64,
) -> StdResult<AmountResponse> {
    let room = Room::read_room(deps, room_id)?;
    Ok(AmountResponse {
        amount: room.amount,
    })
}
fn query_rooms<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    page_num: u64,
    num_of_items: u64,
) -> StdResult<RoomsResponse> {
    let titles = Room::get_rooms(deps, page_num, num_of_items)?;
    Ok(RoomsResponse { rooms: titles })
}
pub fn calculate_amount(coins: &Vec<Coin>) -> Uint128 {
    let mut total_amount: Uint128 = Uint128(0);
    for coin in coins.iter() {
        if coin.denom == "uscrt" {
            total_amount += coin.amount;
        }
    }
    total_amount
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg {};
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn create_room() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {};
        let env = mock_env("creator", &coins(2, "uscrt"));
        let _res = init(&mut deps, env, msg).unwrap();

        let env = mock_env("player1", &coins(40, "uscrt"));
        let msg = HandleMsg::CreateRoom {
            room_title: String::from("room1"),
        };
        let _res = handle(&mut deps, env, msg).unwrap();

        let env = mock_env("player1", &coins(40, "uscrt"));
        let msg = HandleMsg::CreateRoom {
            room_title: String::from("room2"),
        };
        let _ress = handle(&mut deps, env, msg).unwrap();

        let res = query(
            &deps,
            QueryMsg::Rooms {
                page_num: 1,
                num_of_items: 3,
            },
        )
        .unwrap();
        let value: RoomsResponse = from_binary(&res).unwrap();
        println!("{:#?}", value.rooms);
        assert_eq!(
            vec![String::from("room1"), String::from("room2")],
            value.rooms
        );
    }

    #[test]
    fn play() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {};
        let env = mock_env("creator", &coins(2, "uscrt"));
        let _res = init(&mut deps, env, msg).unwrap();

        let env = mock_env("player1", &coins(40, "uscrt"));
        let msg = HandleMsg::CreateRoom {
            room_title: String::from("room1"),
        };
        let _res = handle(&mut deps, env, msg).unwrap();

        let env = mock_env("player1", &coins(40, "uscrt"));
        let msg = HandleMsg::CreateRoom {
            room_title: String::from("room1"),
        };
        let ress = handle(&mut deps, env, msg).unwrap();
        println!("{:?}", ress.data);
        let env = mock_env("player1", &coins(40, "uscrt"));
        let msg = HandleMsg::Play {
            move1: Move::Scissors,
            room_id: 1,
        };
        let _res = handle(&mut deps, env, msg).unwrap();

        let env = mock_env("player2", &coins(2, "uscrt"));
        let msg = HandleMsg::Play {
            move1: Move::Rock,
            room_id: 1,
        };
        let res2 = handle(&mut deps, env, msg).unwrap();
        println!("{:#?}", res2.messages);

        let res = query(&deps, QueryMsg::Winner { room_id: 1 }).unwrap();
        let value: WinnerResponse = from_binary(&res).unwrap();

        println!("{}", value.winner);
        assert_eq!(value.winner, "player2");
    }
}
