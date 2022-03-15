const { Contract, getAccountByName, getLogs } = require("secret-polar");

async function deploy_contract(contract_owner,contract){

   await contract.parseSchema();

  const deploy_response = await contract.deploy(
    contract_owner,
  );
  console.log(deploy_response);

}

async function instantiate_contract(contract_owner,contract){
 
  const contract_info = await contract.instantiate({}, "deploy test1", contract_owner);
  console.log(contract_info);
}

async function play(contract,player,move,coins,room_id){


  const response=await contract.tx.play({account: player, transferAmount: coins},{move1:move,room_id});
  console.log(response);
}


async function create_room(contract,account,title){
  const response=await contract.tx.create_room({account},{room_title:title});
  console.log(response);
}

async function query_winner(contract,room_id){

  const res=await contract.query.winner({room_id});
  console.log(res);
}

async function query_amount(contract,room_id){
  const response=await contract.query.amount({room_id});
  console.log(response);
}

async function query_rooms(contract,page_num,num_of_items){
  const rooms=await contract.query.rooms({page_num,num_of_items});
  console.log(rooms)
}

async function register(contract,account,reg_addr,reg_hash){
  const response=await contract.tx.register({account},{reg_addr,reg_hash});
  console.log(response)

}




async function run () {
  const contract_owner = getAccountByName("a");
  const contract = new Contract("sample-project");
  const transferAmount = [{denom: "uscrt", amount: "15000"}];

  await deploy_contract(contract_owner,contract);
  await instantiate_contract(contract_owner,contract);

  await create_room(contract,contract_owner,"room1");
  await create_room(contract,contract_owner,"room2");

  await register(contract,getAccountByName("a"),"secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg","E47144CD74E2E3E24275962CAA7719F081CCFA81A46532812596CA3D5BA6ECEB")

  //await play(contract,getAccountByName("a"),"scissors",transferAmount,0);
  //await play(contract,getAccountByName("b"),"paper",transferAmount,0);

  await query_winner(contract,0)
  await query_amount(contract,0)
  await query_rooms(contract,1,3)
 
}

module.exports = { default: run };
