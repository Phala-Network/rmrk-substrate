require('dotenv').config();
const sleep = require('p-sleep');
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { stringToU8a, u8aToHex } = require('@polkadot/util');


const rootPrivkey = process.env.ROOT_PRIVKEY;
const userPrivkey = process.env.USER_PRIVKEY;
const overlordPrivkey = process.env.OVERLOAD_PRIVKEY;
const endpoint = process.env.ENDPOINT;

async function main() {
  const wsProvider = new WsProvider(endpoint);
  const api = await ApiPromise.create({ provider: wsProvider });
  const keyring = new Keyring({ type: 'sr25519' });

  const root = keyring.addFromUri(rootPrivkey);
  const user = keyring.addFromUri(userPrivkey);
  const overlord = keyring.addFromUri(overlordPrivkey);

  // list spirit
  {
    const spiritCollectionId = 0;
    const spirit = await api.query.uniques.account.entries(user.address, spiritCollectionId);
    spirit
      .map(([key, _value]) =>
        [key.args[0].toString(), key.args[1].toNumber(), key.args[2].toNumber()]
      ).forEach(([account, collectionId, nftId]) => {
        console.log({
          account,
          collectionId,
          nftId,
        })
    })
  }

  // list egg shells
  {
    const eggCollectionId = 1;
    const spirit = await api.query.uniques.account.entries(user.address, eggCollectionId);
    spirit
      .map(([key, _value]) =>
        [key.args[0].toString(), key.args[1].toNumber(), key.args[2].toNumber()]
      ).forEach(([account, collectionId, nftId]) => {
        console.log({
          account,
          collectionId,
          nftId,
        })
    })
  }
}

main().catch(console.error).finally(() => process.exit());
