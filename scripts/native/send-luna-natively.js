import { MsgSend } from "@terra-money/terra.js";
import { client, wallets } from "../library.js";

const send = new MsgSend(
  wallets.myKey.key.accAddress, // from
  wallets.wallet1.key.accAddress, // to
  { lemon: "6000" }
);

const tx = await wallets.myKey.createAndSignTx({ msgs: [send] });
const result = await client.tx.broadcast(tx);

console.log(result);
