# Backer

# Demo

Create a directory just for this project - will refer to it as "home" directory.

## Install terrad and setup accounts
You will need to install terrad-v0.5.6-oracle

Run these commands from the "home" directory
```
> git clone git@github.com:terra-money/core.git
> cd core
> git checkout v0.5.6-oracle
> make install
```
This should install the correct terrad version.

Next you will need to create 2 accounts - one for the pool and another to interact with the smart contract

Run these commands
```
> echo "cabin undo always radar expand hidden crack tiny waste trouble enroll foster defy shrimp dentist effort chuckle domain ceiling three drop amateur around easy" | terrad keys add pool --recover
> echo "satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn" | terrad keys add user1 --recover
```

The account "pool" will have zero balance while "user1" account has some existing balance since its a special account

## Run terra network locally
To test our smart contract we need to add the contract to the blockchain. Adding it to the testnet is overly complicated. A simpler way would be to run the terra network locally.

Open up a seperate terminal, go to the "home" directory and run these commands
```
> git clone git@github.com:terra-money/LocalTerra.git
> cd LocalTerra
> docker-compose up
```

## Upload and initialize smart contract

Now from the backer repository run these commands!
```
> cargo wasm
> terrad tx wasm store artifacts/backer.wasm --from user1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block
```
You should get an output like this
```
code: 0
codespace: ""
data: 0A260A202F74657272612E7761736D2E763162657461312E4D736753746F7265436F646512020803
gas_used: "808974"
gas_wanted: "810495"
height: "855"
info: ""
logs:
- events:
  - attributes:
    - key: action
      value: /terra.wasm.v1beta1.MsgStoreCode
    - key: module
      value: wasm
    type: message
  - attributes:
    - key: sender
      value: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
    - key: code_id
      value: "3"
    type: store_code
  log: ""
  msg_index: 0
raw_log: '[{"events":[{"type":"message","attributes":[{"key":"action","value":"/terra.wasm.v1beta1.MsgStoreCode"},{"key":"module","value":"wasm"}]},{"type":"store_code","attributes":[{"key":"sender","value":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"},{"key":"code_id","value":"3"}]}]}]'
timestamp: ""
tx: null
txhash: 823A3836CCEEB2B81256CAFAE7CF1A1AF21484561F4009B469E5CBBF6A7EFAA0
```

For me the "code_id" value is 3, it could be different for you.
We have uploaded the contract now we need to initialize it.

Run these commands. Make sure to use the correct code_id!
```
> terrad tx wasm instantiate 3 '{"pool":"terra13nfhuh80ppf5lqsrafdnlw9n9mxrpg3rzf6wk9"}' --from user1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block
```

## Interact with the contract

We will now create a new project. Note the contract address frorm the previous command - mine was "terra19zpyd046u4swqpksr3n44cej4j8pg6ah2y6dcg".
Run the command with proper contract address!
```
> terrad tx wasm execute terra19zpyd046u4swqpksr3n44cej4j8pg6ah2y6dcg '{"create_project":{"title":"backer","description":"decentralized crowd funding!","funding_requested":1000000,"denom":"uusd","legal_contract":"","lockup_period":0,"thumbnail":""}}' --from user1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block
```
Note the project_id - mine is "0".

Congrats you created a project! Lets query to get project details!

```
> terrad query wasm contract-store terra1pcknsatx5ceyfu6zvtmz3yr8auumzrdts4ax4a '{"get_project":{"id":0}}'
```
You should see something like this!
```
query_result:
  denom: uusd
  description: decentralized crowd funding!
  funding_raised: []
  funding_requested: 1e+06
  id: 0
  legal_contract: ""
  lockup_period: 0
  thumbnail: ""
  title: backer
```

Lets back the project!

```
```