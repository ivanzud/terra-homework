import { client, wallets } from "../library.js";

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1nue8z238kq683yqe5806wthec9l3rpj50zdfkw";
const walletAddress = wallets.myKey.key.accAddress;

const response = await client.wasm.contractQuery(cw20Contract, {
  balance: { address: walletAddress },
});

console.log(response);
