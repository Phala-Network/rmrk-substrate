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

    // prep
    {
        await api.tx.sudo.sudo(
            api.tx.phalaWorld.setOverlord(overlord.address)
        ).signAndSend(root, {nonce: -1});
        await sleep(6000);

        await api.tx.phalaWorld.initializeWorldClock()
            .signAndSend(overlord, {nonce: -1});

        // availbe states:
        // ClaimSpirits,
        // PurchaseRareOriginOfShells,
        // PurchaseHeroOriginOfShells,
        // PreorderOriginOfShells,
        await api.tx.phalaWorld.setStatusType(true, 'ClaimSpirits')
            .signAndSend(overlord, {nonce: -1});

        // mint spirits NFTs with overlord
        // collection 0: spirits
        await api.tx.rmrkCore.createCollection(
            '0x',
            null,
            'PWSPRT'
        ).signAndSend(overlord, {nonce: -1});
        // set the spirits collection id
        await api.tx.phalaWorld.setSpiritCollectionId(
            0
        ).signAndSend(overlord, {nonce: -1});
        // collection 1: origin of shells
        await api.tx.rmrkCore.createCollection(
            '0x',
            null,
            'PWOAS'
        ).signAndSend(overlord, {nonce: -1});
        // set the origin of shell collection id
        await api.tx.phalaWorld.setOriginOfShellCollectionId(
            1
        ).signAndSend(overlord, {nonce: -1});
    }

    // // produce spirit whitelist
    // {
    //   const claimer = user.address;
    //   const metadata = '0xDEADBEEF';
    //   const message = api.createType('(AccountId,Vec<u8>)', [claimer, metadata]);
    //   const sig = overlord.sign(message.toU8a());
    //   u8aToHex(sig);
    //   console.log(sig)
    // }
    // return;

    // mint spirit
    {
        // const serialId = 1;
        // const signature = '0xAABB';
        // const metadata = '0xCCDD'
        await api.tx.phalaWorld.claimSpirit()
            .signAndSend(user, {nonce: -1});
    }
}

main().catch(console.error).finally(() => process.exit());