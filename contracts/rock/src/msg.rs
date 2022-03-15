use cosmwasm_std::{Coin, Uint128, HumanAddr, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
   
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Move{
    Rock,
    Paper,
    Scissors
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
 
    Play{
        move1:Move,
        room_id:u64
      
    },
    CreateRoom{
        room_title:String
    },
    Register{
        reg_addr:HumanAddr,
        reg_hash:String
    },
    Receive{
        sender:HumanAddr,
        from:HumanAddr,
        amount:Uint128,
        memo:Option<String>,
        msg:Binary
    }
        

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]

pub struct PlayMsg{
    pub move1:Move,
    pub room_id:u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    
   
    Winner{room_id:u64},
    Amount{room_id:u64},
    Rooms{page_num:u64,num_of_items:u64}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WinnerResponse {
    pub winner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmountResponse{
    pub amount:Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoomsResponse{
     pub rooms:Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Snip20Msg {
    RegisterReceive {
        code_hash: String,
        padding: Option<String>,
    },
    Transfer{
        recipient:String,
        amount:Uint128,
        padding:Option<String>
    }
}

impl Snip20Msg{
    pub fn register_receive(code_hash: String) -> Self {
        Snip20Msg::RegisterReceive {
            code_hash,
            padding: None, // TODO add padding calculation
        }
    }
    pub fn transfer(recipient:String,amount:Uint128)->Self{
        Snip20Msg::Transfer{
            recipient,
            amount,
            padding:None
        }
    }
}