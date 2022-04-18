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
            },
            NftSaleMetadata: {
                metadata: "BoundedString",
                signature: "sr25519::Signature"
            }
        }
    });
    const keyring = new Keyring({type: 'sr25519'});

    const root = keyring.addFromUri(rootPrivkey);
    const user = keyring.addFromUri(userPrivkey);
    const overlord = keyring.addFromUri(overlordPrivkey);

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

    // // produce whitelist
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
        const metadata = 'I am Spirit';
        const metadataType = api.createType('BoundedVec<u8, T::StringLimit>', metadata).toU8a();
        const metadataSig = overlord.sign(metadataType);
        const isValid = overlord.verify(metadata, metadataSig, overlord.address);
        const nftSignedMetadata = api.createType('NftSaleMetadata', {'metadata': metadataType, 'signature': metadataSig});

        // output the result
        console.log(`${u8aToHex(metadataSig)}\n${u8aToHex(metadataType)} is ${isValid ? 'valid' : 'invalid'}`);
        // Mint a Spirit
        //await api.tx.phalaWorld.claimSpirit(null, nftSignedMetadata).signAndSend(user);
    }

    // mint spirit nft
    {
        // const serialId = 1;
        // const signature = '0xAABB';
        // const metadata = '0xCCDD'
        // const metadataSig = overlord.sign(metadata);
        // u8aToHex(metadataSig);
        // await api.tx.phalaWorld.claimSpirit(null, nftSignedMetadata).signAndSend(user);
    }

    // purchase rare origin of shell
    {
        // OriginOfShellType ['Legendary', 'Magic', 'Hero']
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        // metadata '0x2813308004'
        // const metadataSig = overlord.sign(metadata);
        // u8aToHex(metadataSig);
        await api.tx.phalaWorld.buyRareOriginOfShell()
            .signAndSend(user);
    }

    // purchase whitelist hero origin of shell
    {
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        // whitelistClaim createType('(AccountId,Vec<u8>)', [claimer, metadata]);
        // const sig = overlord.sign(message.toU8a());
        // u8aToHex(sig);
        // metadata '0x2813308004'
        // const metadataSig = overlord.sign(metadata);
        // u8aToHex(metadataSig);
        await api.tx.phalaWorld.buyHeroOriginOfShell()
            .signAndSend(user);
    }

    // preorder origin of shell
    {
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        // metadata '0x2813308004'
        // const metadataSig = overlord.sign(metadata);
        // u8aToHex(metadataSig);
        await api.tx.phalaWorld.preorderOriginOfShell()
            .signAndSend(user);
    }

    // privileged function preorder status to declare Chosen or NotChosen preorders
    {
        // Preorder id
        // status ['Chosen', 'NotChosen']
        await api.tx.phalaWorld.setPreorderStatus()
            .signAndSend(overlord);
    }

    // Claim chosen preorders
    {
        await api.tx.phalaWorld.claimChosenPreorders()
            .signAndSend(user);
    }

    // Claim refund for not chosen preorders
    {
        await api.tx.phalaWorld.claimRefundPreorders()
            .signAndSend(user);
    }
}