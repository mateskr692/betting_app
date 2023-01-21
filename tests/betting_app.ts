import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BettingApp } from "../target/types/betting_app";
import { expect, assert, use } from "chai";
import { PublicKey } from "@solana/web3.js";

async function initialize(
  program: Program<BettingApp>,
  contract: any,
  owner: any,
) {
  const [programPDA, _] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("program-wallet"),
      contract.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .initialize()
    .accounts({
      owner: owner.publicKey,
      contract: contract.publicKey,
      programWallet: programPDA,
    })
    .signers([contract])
    .rpc();
}

async function reserveSpace(
  program: Program<BettingApp>,
  contract: any,
  owner: any,
) {
  await program.methods
    .reserveSpace()
    .accounts({
      owner: owner.publicKey,
      contract: contract.publicKey,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function collectTaxes(
  program: Program<BettingApp>,
  contract: any,
  owner: any
) {
  const [programPDA, _pb] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("program-wallet"),
      contract.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .collectTaxes()
    .accounts({
      owner: owner.publicKey,
      contract: contract.publicKey,
      programWallet: programPDA,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function addScheduledGame(
  program: Program<BettingApp>,
  contract: any,
  owner: any,
  gameId,
) {
  await program.methods
    .addScheduledGame(gameId)
    .accounts({
      owner: owner.publicKey,
      contract: contract.publicKey,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function setGameState(
  program: Program<BettingApp>,
  contract: any,
  owner: any,
  gameId,
  state: string,
  result: string,
) {
  await program.methods
    .setGameState(gameId, state, result)
    .accounts({
      owner: owner.publicKey,
      contract: contract.publicKey,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function deleteGame(
  program: Program<BettingApp>,
  contract: any,
  owner: any,
  gameId,
) {
  await program.methods
    .deleteGame(gameId)
    .accounts({
      owner: owner.publicKey,
      contract: contract.publicKey,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function createUserStats(
  program: Program<BettingApp>, 
  user: any
) {
  const [userStatsPDA, _] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user-stats"),
      user.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .createUserStats()
    .accounts({
      user: user.publicKey,
      userStats: userStatsPDA,
    })
    .signers(user instanceof (anchor.Wallet as any) ? [] : [user])
    .rpc();

  // expect(
  //   (await program.account.userStats.fetch(userStatsPDA)).gamesWon
  // ).to.equal(0);
}

async function placeWager(
  program: Program<BettingApp>,
  contract: any,
  user: any,
  gameId,
  amount,
  prediction,
) {
  const [userStatsPDA, _ub] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user-stats"),
      user.publicKey.toBuffer(),
    ],
    program.programId
  );
  const [programPDA, _pb] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("program-wallet"),
      contract.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .placeWager(gameId, amount, prediction)
    .accounts({
      user: user.publicKey,
      contract: contract.publicKey,
      programWallet: programPDA,
      userStats: userStatsPDA,
    })
    .signers(user instanceof (anchor.Wallet as any) ? [] : [user])
    .rpc();
}

async function withdrawWager(
  program: Program<BettingApp>,
  contract: any,
  user: any,
  gameId,
) {
  const [userStatsPDA, _ub] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user-stats"),
      user.publicKey.toBuffer(),
    ],
    program.programId
  );
  const [programPDA, _pb] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("program-wallet"),
      contract.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .withdrawWager(gameId)
    .accounts({
      user: user.publicKey,
      contract: contract.publicKey,
      programWallet: programPDA,
      userStats: userStatsPDA,
    })
    .signers(user instanceof (anchor.Wallet as any) ? [] : [user])
    .rpc();
}

async function collectWager(
  program: Program<BettingApp>,
  contract: anchor.web3.Signer,
  user: anchor.web3.Signer,
  gameId,
) {
  const [userStatsPDA, _ub] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user-stats"),
      user.publicKey.toBuffer(),
    ],
    program.programId
  );
  const [programPDA, _pb] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("program-wallet"),
      contract.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .collectWager(gameId)
    .accounts({
      user: user.publicKey,
      contract: contract.publicKey,
      userStats: userStatsPDA,
      programWallet: programPDA,
    })
    .signers(user instanceof (anchor.Wallet as any) ? [] : [user])
    .rpc();
}

const airdropToAddress = async (provider, address: PublicKey, amount) => {
  const airdropSignature = await provider.connection.requestAirdrop(
    address,
    1000000000 * amount
  );
  await provider.connection.confirmTransaction(airdropSignature);
};

function getUserPDA(program, user) {
  const [userStatsPDA, _] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user-stats"),
      user.publicKey.toBuffer(),
    ],
    program.programId
  );
  return userStatsPDA;
}

function getProgramWallet(program, contract) {
  const [programPDA, _] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("program-wallet"),
      contract.publicKey.toBuffer(),
    ],
    program.programId
  );
  return programPDA;
}


describe("betting_app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BettingApp as Program<BettingApp>;
  const owner = (program.provider as anchor.AnchorProvider).wallet;
  const contract = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();
  const user2 = anchor.web3.Keypair.generate();

  before(async () => {
    await initialize(program, contract, owner);
    await reserveSpace(program, contract, owner);
    await reserveSpace(program, contract, owner);
    await reserveSpace(program, contract, owner);
    await reserveSpace(program, contract, owner);

    await airdropToAddress(program.provider, user.publicKey, 10);
    await airdropToAddress(program.provider, user2.publicKey, 10);
    await createUserStats(program, user);
    await createUserStats(program, user2);
    expect(true);
  });

  it("Check if user created his PDA", async () => {
    const newUser = anchor.web3.Keypair.generate();
    const info = await program.provider.connection.getAccountInfo(user.publicKey);
    const newUserInfo = await program.provider.connection.getAccountInfo(newUser.publicKey);
    expect(info).to.not.be.null;
    expect(newUserInfo).to.be.null;
  });

  it("Owner adds scheduled games", async () => {
    let gameId = 25412;
    await addScheduledGame(program, contract, owner, gameId);
    let state = await program.account.programContract.fetch(contract.publicKey);
    expect(state.activeGames).to.have.lengthOf(1);
    expect(state.activeGames[0].id).to.equal(gameId);
    expect(state.activeGames[0].wagers).to.have.lengthOf(0);
    expect(state.activeGames[0].result).to.equal(null);

    gameId = 82671;
    await addScheduledGame(program, contract, owner, gameId);
    state = await program.account.programContract.fetch(contract.publicKey);
    expect(state.activeGames).to.have.lengthOf(2);
    expect(state.activeGames[1].id).to.equal(gameId);
    expect(state.activeGames[1].wagers).to.have.lengthOf(0);
    expect(state.activeGames[1].result).to.equal(null);
  });

  it("Owner fails to add existing game", async () => {
    let state = await program.account.programContract.fetch(contract.publicKey);
    const gameId = state.activeGames[0].id;
    try {
      await addScheduledGame(program, contract, owner, gameId);
      expect(false, "Error should've been thrown");
    } catch(_err) {
      expect(_err.error.errorCode.code).to.equal("GameAlreadyExists");
    }
  });

  it("User fails to add game", async () => {
    const gameId = 967841;
    try {
      await addScheduledGame(program, contract, user, gameId);
      expect(false, "Error should've been thrown");
    } catch(_err) {
      expect(_err.error.errorCode.code).to.equal("InstructionNotPermitted");
    }
  });

  it("User bets on a game", async () => {
    let state = await program.account.programContract.fetch(contract.publicKey);
    const gameId = state.activeGames[0].id;
    const prediction = "HomeVictory"
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);

    const balance_before = await program.provider.connection.getBalance(getProgramWallet(program, contract));
    await placeWager(program, contract, user, gameId, amount, prediction);
    const balance_after = await program.provider.connection.getBalance(getProgramWallet(program, contract));

    state = await program.account.programContract.fetch(contract.publicKey);
    const gameState = state.activeGames[0];
    expect(gameState.wagers).to.have.lengthOf(1);
    expect(gameState.wagers[0].user).to.deep.equal(user.publicKey);
    expect(gameState.wagers[0].prediction).to.deep.equal( { homeVictory: {} } );
    expect(gameState.wagers[0].lamports.lt(amount));
    expect(state.taxesAccumulated.gt(new anchor.BN(0)));
    expect(balance_after).to.be.greaterThan(balance_before);

    const stats =  await program.account.userStats.fetch(getUserPDA(program, user));
    expect(stats.history).to.have.lengthOf(1);
    expect(stats.history[0].gameId).to.equal(gameId);
    expect(stats.history[0].predictedResult).to.deep.equal( { homeVictory: {} } );
    expect(stats.history[0].actuallResult).to.equal(null);
    expect(stats.history[0].lamportsBet.lt(amount));
    expect(stats.history[0].lamportsWon.eqn(0));
    console.log("program balance increased by %d", balance_after - balance_before);
  });

  it("User fails to bet on a game twice", async () => {
    const stats =  await program.account.userStats.fetch(getUserPDA(program, user));
    const gameId = stats.history[0].gameId;
    const prediction = "AwayVictory";
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    try {
      await placeWager(program, contract, user, gameId, amount, prediction);
      expect(false, "Shouldnt be able to bet twice");
    } catch(_err) {
      expect(_err.error.errorCode.code).to.equal("WagerAlreadyPlaced");
    }
  });

  it("User fails to bet on a non existing game", async () => {
    const gameId = 9182;
    const prediction = "Tie";
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    try {
      await placeWager(program, contract, user, gameId, amount, prediction);
      expect(false, "Shouldnt be able to bet");
    } catch(_err) {
      expect(_err.error.errorCode.code).to.equal("InvalidGameId");
    }
  });

  it("User withdraws a bet", async () => {
    let stats =  await program.account.userStats.fetch(getUserPDA(program, user));
    const gameId = stats.history[0].gameId;

    const balance_before = await program.provider.connection.getBalance(getProgramWallet(program, contract));
    await withdrawWager(program, contract, user, gameId);
    const balance_after = await program.provider.connection.getBalance(getProgramWallet(program, contract));
    const state = await program.account.programContract.fetch(contract.publicKey);
    stats = await program.account.userStats.fetch(getUserPDA(program, user));
    
    expect(state.activeGames[0].wagers).to.be.lengthOf(0);
    expect(state.taxesAccumulated.gtn(0));
    expect(stats.history).to.be.lengthOf(0);
    expect(balance_after).to.be.lessThan(balance_before);
    console.log("program balance decreased by %d", balance_before - balance_after);
  });

  it("User fails to withdraw a non existing bet", async () => {``
    const state = await program.account.programContract.fetch(contract.publicKey);
    const gameId = state.activeGames[0].id;
    try {
      await withdrawWager(program, contract, user, gameId);
      expect(false, "Shouldnt be able to bet twice");
    } catch(_err) {
      expect(_err.error.errorCode.code).to.equal("WagerNotPlaced");
    }
  });

  it("Owner changes status of a game", async () => {
    let state = await program.account.programContract.fetch(contract.publicKey);
    const gameId = state.activeGames[0].id;
    const gameState = "Live"
    const result = "None";

    await setGameState(program, contract, owner, gameId, gameState, result );
    state = await program.account.programContract.fetch(contract.publicKey);
    expect(state.activeGames[0].state).to.deep.equal({ live: {} });
    expect(state.activeGames[0].result).to.equal(null);

  });

  it("User fails to bet on a Live game", async () => {
    const state = await program.account.programContract.fetch(contract.publicKey);
    const gameId = state.activeGames[0].id;
    const prediction = "AwayVictory";
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    try {
      await placeWager(program, contract, user, gameId, amount, prediction);
      expect(false, "Shouldnt be able to bet twice");
    } catch(_err) {
      expect(_err.error.errorCode.code).to.equal("GameAlreadyStarted");
    }

  });

  it("Owner sets result of a game", async () => {
    let state = await program.account.programContract.fetch(contract.publicKey);
    const gameId = state.activeGames[0].id;
    const gameState = "Finished"
    const result = "HomeVictory";

    await setGameState(program, contract, owner, gameId, gameState, result );
    state = await program.account.programContract.fetch(contract.publicKey);
    expect(state.activeGames[0].state).to.deep.equal({ finished: {} });
    expect(state.activeGames[0].result).to.deep.equal( { homeVictory: {} });
  });

  it("Owner deletes finished game", async () => {
    let state = await program.account.programContract.fetch(contract.publicKey);
    let gameId = state.activeGames[0].id;
    let gameCount = state.activeGames.length;

    await deleteGame(program, contract, owner, gameId);
    state = await program.account.programContract.fetch(contract.publicKey);
    expect(state.activeGames.length).to.equal(gameCount - 1);
  });

  it("Users bet on a game, 1 win, 1 loses", async () => {
    const gameId = 3974351;
    const prediction = "HomeVictory";
    const result = "AwayVictory";
    const gameState = "Finished";
    const amount1 = new anchor.BN(0.01 * anchor.web3.LAMPORTS_PER_SOL);
    const amount2 = new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL);

    await addScheduledGame(program, contract, owner, gameId);
    await placeWager(program, contract, user, gameId, amount1, result);
    await placeWager(program, contract, user2, gameId, amount2, prediction);
    await setGameState(program, contract, owner, gameId, gameState, result);
    console.log("User1 bet %d", amount1.toNumber());
    console.log("User2 bet %d", amount2.toNumber());
    try {
      await collectWager(program, contract, user2, gameId);
      expect(false, "Shouldnt be able to collect wager on a lost bet");
    } catch (_err) {
      expect(_err.error.errorCode.code).to.equal("NoAmountOwed");
    }

    const balance_before = await program.provider.connection.getBalance(getProgramWallet(program, contract));
    await collectWager(program, contract, user, gameId);
    const balance_after = await program.provider.connection.getBalance(getProgramWallet(program, contract));
    const state = await program.account.programContract.fetch(contract.publicKey);
    const game = state.activeGames.find(x => x.id == gameId);

    expect(balance_after).to.be.lessThan(balance_before);
    expect(game.wagers[0].collectedReward).to.be.true;
    console.log("User1 won %d", balance_before - balance_after);

    try {
      await collectWager(program, contract, user, gameId);
      expect(false, "Shouldnt be able to collect reward twice");
    } catch (_err) {
      expect(_err.error.errorCode.code).to.equal("NoAmountOwed");
    }

  });

  it("Owner collects taxes", async () => {

    const balance_before = await program.provider.connection.getBalance(getProgramWallet(program, contract));
    expect(balance_before).to.not.equal(0);
    await collectTaxes(program, contract, owner);
    const balance_after = await program.provider.connection.getBalance(getProgramWallet(program, contract));

    let state = await program.account.programContract.fetch(contract.publicKey);
    expect(state.taxesAccumulated.eqn(0));
    console.log("program balance decreased by %d", balance_before - balance_after);
  });

});
