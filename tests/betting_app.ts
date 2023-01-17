import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BettingApp } from "../target/types/betting_app";
import { expect, assert } from "chai";
import { PublicKey } from "@solana/web3.js";

async function initialize(
  program: Program<BettingApp>,
  contract,
  owner,
) {
  await program.methods
    .initialize()
    .accounts({
      owner: owner.publicKey,
      contract,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function collectTaxes(
  program: Program<BettingApp>,
  contract: any,
  owner: anchor.web3.Signer,
) {
  await program.methods
    .collectTaxes()
    .accounts({
      owner: owner.publicKey,
      contract,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function addScheduledGame(
  program: Program<BettingApp>,
  contract: any,
  owner: anchor.web3.Signer,
  gameId
) {
  await program.methods
    .addScheduledGame(gameId)
    .accounts({
      owner: owner.publicKey,
      contract,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function setGameState(
  program: Program<BettingApp>,
  contract: any,
  owner: anchor.web3.Signer,
  gameId,
  state,
  result,
) {
  await program.methods
    .setGameState(gameId, state, result)
    .accounts({
      owner: owner.publicKey,
      contract,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function deleteGame(
  program: Program<BettingApp>,
  contract: any,
  owner: anchor.web3.Signer,
  gameId,
) {
  await program.methods
    .deleteGame(gameId)
    .accounts({
      owner: owner.publicKey,
      contract,
    })
    .signers(owner instanceof (anchor.Wallet as any) ? [] : [owner])
    .rpc();
}

async function createUserStats(
  program: Program<BettingApp>, 
  user: anchor.web3.Signer
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
  user: anchor.web3.Signer,
  gameId,
  amount,
  prediction,
) {
  const [userStatsPDA, _] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user-stats"),
      user.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .placeWager(gameId, amount, prediction)
    .accounts({
      user: user.publicKey,
      contract,
      userStats: userStatsPDA,
    })
    .signers(user instanceof (anchor.Wallet as any) ? [] : [user])
    .rpc();
}

async function withdrawWager(
  program: Program<BettingApp>,
  contract: any,
  user: anchor.web3.Signer,
  gameId,
) {
  const [userStatsPDA, _] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user-stats"),
      user.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .withdrawWager(gameId)
    .accounts({
      user: user.publicKey,
      contract,
      userStats: userStatsPDA,
    })
    .signers(user instanceof (anchor.Wallet as any) ? [] : [user])
    .rpc();
}

async function collectWager(
  program: Program<BettingApp>,
  contract: any,
  user: anchor.web3.Signer,
  gameId,
) {
  const [userStatsPDA, _] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("user-stats"),
      user.publicKey.toBuffer(),
    ],
    program.programId
  );

  await program.methods
    .collectWager(gameId)
    .accounts({
      user: user.publicKey,
      contract,
      userStats: userStatsPDA,
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


describe("betting_app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.BettingApp as Program<BettingApp>;
  const contract = anchor.web3.Keypair.generate().publicKey;
  const owner = (program.provider as anchor.AnchorProvider).wallet;
  console.log(owner.publicKey);

  before(async () => {
    try {
      await initialize(program, contract, owner);
    } catch(_err) {
      console.error(_err);
      assert(false, "No error should occur during initialization");
    }
  });

  it("add game", async () => {
    expect(true);
  });
});
