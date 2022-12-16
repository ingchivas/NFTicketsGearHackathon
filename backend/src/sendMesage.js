import { GearApi, getWasmMetadata, GearKeyring } from "@gear-js/api";
import fs from "fs";

export default async function sendMessage(payload) {
  const destination = process.env.PROGRAM_ID
  const gearApi = await GearApi.create({
    providerAddress: "wss://rpc-node.gear-tech.io"
  });

  const meta = await getWasmMetadata(fs.readFileSync("./contract/event.meta.wasm"));
  const keyring = await GearKeyring.fromSuri('//Alice'); //Default Accounts

  try {
    const message = {
      destination: destination, // programId
      payload,
      gasLimit: 10000000,
      value: 1000,
    };

    // In that case payload will be encoded using meta.handle_input type
    let extrinsic = gearApi.message.send(message, meta);

    // So if you want to use another type you can specify it
    extrinsic = gearApi.message.send(message, meta, meta.async_handle_input);

    await extrinsic.signAndSend(keyring, (event) => {
      const human = event.toHuman();

      console.log(human);
      return human;
    });
  } catch (error) {
    console.error(error);
  }
}
