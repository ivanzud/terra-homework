<<<<<<< HEAD
import { client, wallets } from "../library.js";
=======
import { client, wallets } from '../library.js';
>>>>>>> b52d585d02aabfd277c0e5f9821f8c73a7040015

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1nue8z238kq683yqe5806wthec9l3rpj50zdfkw";
const walletAddress = wallets.myKey.key.accAddress;

<<<<<<< HEAD
const response = await client.wasm.contractQuery(cw20Contract, {
  balance: { address: walletAddress },
});
=======
const response = await client.wasm.contractQuery(cw20Contract, { balance: { address: walletAddress }});
>>>>>>> b52d585d02aabfd277c0e5f9821f8c73a7040015

console.log(response);
