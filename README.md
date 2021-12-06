# STARBREEDER Governance

**NOTE**: Reference documentation for this contract is available [here](https://docs.mirror.finance/contracts/gov).

The Gov Contract contains logic for holding polls and STAR Token (STAR) staking, and allows the STARBREEDER Protocol to be governed by its users in a decentralized manner. After the initial bootstrapping of STARBREEDER Protocol contracts, the Gov Contract is assigned to be the owner of itself and STARBREEDER.

New proposals for change are submitted as polls, and are voted on by STAR stakers through the voting procedure. Polls can contain messages that can be executed directly without changing the STARBREEDER Protocol code.

The Gov Contract keeps a balance of STARBREEDER tokens, which it uses to reward stakers with funds it receives from trading fees sent by the STARBREEDER Collector and user deposits from creating new governance polls. This balance is separate from the Community Pool, which is held by the Community contract (owned by the Gov contract).
