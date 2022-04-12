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
        const spiritCollectionId = await api.query.phalaWorld.spiritCollectionId();
        if (spiritCollectionId.isSome()) {
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
        } else {
            throw new Error(
                'Spirit Collection ID not configured'
            )
        }
    }

    // list origin of shells
    {
        const originOfShellCollectionId = await api.query.phalaWorld.OriginOfShellCollectionId();
        if (originOfShellCollectionId.isSome()) {
            const spirit = await api.query.uniques.account.entries(user.address, originOfShellCollectionId);
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
        } else {
            throw new Error(
                'Origin of Shell Collection ID not configured'
            )
        }
    }

    // List all preorders before drawing winners
    {
        let x = 0;
        const preorderIndex = await api.query.phalaWorld.preorderIndex();
        console.log(`Current preorder index: ${preorderIndex}`);
        if (x++ < preorderIndex) {
            const preorder = await api.query.phalaWorld.preorders(x);
            if (preorder.isSome()) {
                //console.log() TODO list preorders info
            }
        }
    }

    // List of Preorder ids for a user
    {
        const userPreorderResults = await api.query.phalaWorld.PreorderResults.keys(user.address);
        userPreorderResults.forEach(([{ args: [accountId, preorderId] }, _value]) => {
           console.log(`Account: ${accountId} Preorder ID: ${preorderId} Preorder results:`)
        });
    }


    // List the current Era
    {
        const currentEra = await api.query.phalaWorld.era();
        console.log(`Current era: ${currentEra}`);
    }

    // List Zero Day Timestamp
    {
        const zeroDayTimestamp = await api.query.phalaWorld.zeroDay();
        console.log(`Zero Day: ${zeroDayTimestamp}`);
    }

    // List all race types
    {
        //const originOfShelltype = await api.query.phalaWorld.originOfShellType()
    }

    // Can users claim spirit?
    {
        const canClaimSpirits = await api.query.phalaWorld.canClaimSpirits();
        console.log(`Can claim spirit states: ${canClaimSpirits}`);
    }

    // Can users purchase rare origin of shell?
    {
        const canPurchaseRareOriginOfShells = await api.query.phalaWorld.canPurchaseRareOriginOfShells();
        console.log(`Can purchase rare origin of shells: ${canPurchaseRareOriginOfShells}`);
    }

    // Can users on whitelist purchase hero origin of shell?
    {
        const canPurchaseHer0OriginOfShells = await api.query.phalaWorld.canPurchaseHeroOriginOfShells();
        console.log(`Can whitelist purchase hero origin of shells: ${canPurchaseHeroOriginOfShells}`);
    }

    // Can users preorder origin of shell?
    {
        const canPreorderOriginOfShells = await api.query.phalaWorld.canPreorderOriginOfShells();
        console.log(`Can preorder origin of shells: ${canPreorderOriginOfShells}`);
    }

    // Is last day of sale?
    {
        const isLastDayOfSale = await api.query.phalaWorld.lastDayOfSale();
        console.log(`Is last day of sale: ${isLastDayOfSale}`);
    }
}

main().catch(console.error).finally(() => process.exit());