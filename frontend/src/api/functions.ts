import { GearApi, getWasmMetadata, GearKeyring } from "@gear-js/api";
import fs from "fs";

export async function upload() {
  const gearApi = await GearApi.create();
  const code = fs.readFileSync('demo_ping.opt.wasm');
  const meta = await getWasmMetadata(fs.readFileSync('demo_async.opt.wasm'));
  const keyring = await GearKeyring.fromSuri('//Alice'); //Default Accounts

  const gas = await gearApi.program.calculateGas.initUpload(
    '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d', // source id
    code,
    '0x00', // payload
    0, //value
    true, // allow other panics
  );

  console.log("Gas:", gas.toHuman());

  const program = {
    code,
    gasLimit: 1000000,
    value: 1000,
    /*
    initPayload: somePayload,
    */
  };

  try {
    const { programId, codeId, salt, extrinsic } = gearApi.program.upload(
      program,
      meta,
    );

    await extrinsic.signAndSend(keyring, (event) => {
      console.log(event.toHuman());
    });
  } catch (error) {
    console.error(error);
  }
}
