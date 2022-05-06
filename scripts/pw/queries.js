require('dotenv').config();
const sleep = require('p-sleep');
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { stringToU8a, u8aToHex } = require('@polkadot/util');


const rootPrivkey = process.env.ROOT_PRIVKEY;
const userPrivkey = process.env.USER_PRIVKEY;
const overlordPrivkey = process.env.OVERLOAD_PRIVKEY;
const ferdiePrivkey = process.env.FERDIE_PRIVKEY;
const charliePrivkey = process.env.CHARLIE_PRIVKEY;
const davidPrivkey = process.env.DAVID_PRIVKEY;
const evePrivkey = process.env.EVE_PRIVKEY;
const endpoint = process.env.ENDPOINT;

async function main() {
    const wsProvider = new WsProvider(endpoint);
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: {
            RaceType: {
                _enum: ['Cyborg', 'AISpectre', 'XGene', 'Pandroid']
            },
            CareerType: {
                _enum: ['HardwareDruid', 'RoboWarrior', 'TradeNegotiator', 'HackerWizard', 'Web3Monk']
            },
            StatusType: {
                _enum: ['ClaimSpirits', 'PurchaseRareOriginOfShells', 'PurchaseHeroOriginOfShells', 'PreorderOriginOfShells']
            },
            OriginOfShellType: {
                _enum: ['Hero', 'Magic', 'Legendary']
            },
            PreorderStatus: {
                _enum: ['Pending', 'Chosen', 'NotChosen']
            },
            PreorderInfo: {
                owner: "AccountId",
                race: "RaceType",
                career: "CareerType",
                metadata: "BoundedString",
                preorder_status: "PreorderStatus",
            },
            NftSaleInfo: {
                race_count: "u32",
                race_for_sale_count: "u32",
                race_giveaway_count: "u32",
                race_reserved_count: "u32",
            }
        }
    });
    const keyring = new Keyring({ type: 'sr25519' });

    const root = keyring.addFromUri(rootPrivkey);
    const user = keyring.addFromUri(userPrivkey);
    const ferdie = keyring.addFromUri(ferdiePrivkey);
    const overlord = keyring.addFromUri(overlordPrivkey);
    const charlie = keyring.addFromUri(charliePrivkey);
    const david = keyring.addFromUri(davidPrivkey);
    const eve = keyring.addFromUri(evePrivkey);

    // StatusType
    const claimSpirits = api.createType('StatusType', 'ClaimSpirits');
    const purchaseRareOriginOfShells = api.createType('StatusType', 'PurchaseRareOriginOfShells');
    const purchaseHeroOriginOfShells = api.createType('StatusType', 'PurchaseHeroOriginOfShells');
    const preorderOriginOfShells = api.createType('StatusType', 'PreorderOriginOfShells');

    // OriginOfShellTypes
    const legendary = api.createType('OriginOfShellType', 'Legendary');
    const magic = api.createType('OriginOfShellType', 'Magic');
    const hero = api.createType('OriginOfShellType', 'Hero');

    // RaceTypes
    const cyborg = api.createType('RaceType', 'Cyborg');
    const aiSpectre = api.createType('RaceType', 'AISpectre');
    const xGene = api.createType('RaceType', 'XGene');
    const pandroid = api.createType('RaceType', 'Pandroid');

    // CareerTypes
    const hardwareDruid = api.createType('CareerType', 'HardwareDruid');
    const roboWarrior = api.createType('CareerType', 'RoboWarrior');
    const tradeNegotiator = api.createType('CareerType', 'TradeNegotiator');
    const hackerWizard = api.createType('CareerType', 'HackerWizard');
    const web3Monk = api.createType('CareerType', 'Web3Monk');

    // PreorderStatus
    const pending = api.createType('PreorderStatus', 'Pending');
    const chosen = api.createType('PreorderStatus', 'Chosen');
    const notChosen = api.createType('PreorderStatus', 'NotChosen');

    // list spirit
    {
        const spiritCollectionId = await api.query.pwNftSale.spiritCollectionId();
        if (spiritCollectionId.isSome) {
            const spirit = await api.query.uniques.account.entries(user.address, spiritCollectionId.unwrap());
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
        const originOfShellCollectionId = await api.query.pwNftSale.originOfShellCollectionId();
        if (originOfShellCollectionId.isSome) {
            const spirit = await api.query.uniques.account.entries(user.address, originOfShellCollectionId.unwrap());
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
        const preorderIndex = await api.query.pwNftSale.preorderIndex();
        console.log(`Current preorder index: ${preorderIndex}`);
        const preorderKeys = await api.query.pwNftSale.preorders.entries();
        preorderKeys
            .map(([key, value]) =>
                [key.args[0].toNumber(), value.toHuman()]
            ).forEach(([preorderId, preorderInfo]) => {
            console.log({
                preorderId,
                preorderInfo,
            })
        })
    }

    // List of Preorder ids for a user
    // Example output
    // {
    //     account: '5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL',
    //         preorderId: 0,
    //     preorderInfo: {
    //     owner: '5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL',
    //         race: 'Pandroid',
    //         career: 'HackerWizard',
    //         metadata: 'I am Hero',
    //         preorderStatus: 'Chosen'
    //      }
    // }
    {
        const userPreorderResults = await api.query.pwNftSale.preorderResults.entries(ferdie.address);
        userPreorderResults
            .map(([key, value]) =>
                [key.args[0].toString(), key.args[1].toNumber(), value.toHuman()]
            ).forEach(([account, preorderId, preorderInfo]) => {
            console.log({
                account,
                preorderId,
                preorderInfo,
            })
        })
    }


    // List the current Era
    {
        const currentEra = await api.query.pwNftSale.era();
        console.log(`Current era: ${currentEra}`);
    }

    // List Zero Day Timestamp
    {
        const zeroDayTimestamp = await api.query.pwNftSale.zeroDay();
        console.log(`Zero Day: ${zeroDayTimestamp}`);
    }

    // List all OriginOfShellsInventory
    {
        const originOfShellsInventoryLegendary = await api.query.pwNftSale.OriginOfShellsInventory.keys('Legendary');
        originOfShellsInventoryLegendary.forEach(([{ args: race }, _value]) => {
           console.log(`Origin of Shell Type: Legendary\nRace Type: {}`)
        });
    }

    // Can users claim spirit?
    {
        const canClaimSpirits = await api.query.pwNftSale.canClaimSpirits();
        console.log(`Can claim spirit states: ${canClaimSpirits}`);
    }

    // Can users purchase rare origin of shell?
    {
        const canPurchaseRareOriginOfShells = await api.query.pwNftSale.canPurchaseRareOriginOfShells();
        console.log(`Can purchase rare origin of shells: ${canPurchaseRareOriginOfShells}`);
    }

    // Can users on whitelist purchase hero origin of shell?
    {
        const canPurchaseHer0OriginOfShells = await api.query.pwNftSale.canPurchaseHeroOriginOfShells();
        console.log(`Can whitelist purchase hero origin of shells: ${canPurchaseHer0OriginOfShells}`);
    }

    // Can users preorder origin of shell?
    {
        const canPreorderOriginOfShells = await api.query.pwNftSale.canPreorderOriginOfShells();
        console.log(`Can preorder origin of shells: ${canPreorderOriginOfShells}`);
    }

    // Is last day of sale?
    {
        const isLastDayOfSale = await api.query.pwNftSale.lastDayOfSale();
        console.log(`Is last day of sale: ${isLastDayOfSale}`);
    }
}

main().catch(console.error).finally(() => process.exit());