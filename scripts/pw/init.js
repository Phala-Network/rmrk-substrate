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
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: {
            RaceType: {
                _enum: ['Cybord', 'AISpectre', 'XGene', 'Pandroid']
            },
            CareerType: {
                _enum: ['HardwareDruid', 'RoboWarrior', 'TradeNegotiator', 'HackerWizard', 'Web3Monk']
            },
            StatusType: {
                _enum: ['ClaimSpirits', 'PurchaseRareOriginOfShells', 'PurchaseHeroOriginOfShells', 'PreorderOriginOfShells']
            }
        }
    });
    const keyring = new Keyring({ type: 'sr25519' });
    // status types
    const claimSpirits = api.createType('StatusType', 'ClaimSpirits');
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

        // available states:
        // ClaimSpirits,
        // PurchaseRareOriginOfShells,
        // PurchaseHeroOriginOfShells,
        // PreorderOriginOfShells,
        await api.tx.phalaWorld.setStatusType(true, claimSpirits)
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

    // sign metadata
    {
        const metadata = '0x1234';
        const metadataSig = overlord.sign(metadata);
        u8aToHex(metadataSig);
        console.log(metadataSig);
    }

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