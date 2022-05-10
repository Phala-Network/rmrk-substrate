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
                _enum: ['ClaimSpirits', 'PurchaseRareOriginOfShells', 'PurchasePrimeOriginOfShells', 'PreorderOriginOfShells']
            },
            OriginOfShellType: {
                _enum: ['Prime', 'Magic', 'Legendary']
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
            MessageType: {
                _enum: ['RedeemSpirit', 'Whitelist']
            },
            OverlordMessage: {
                account: "AccountId",
                purpose: "MessageType",
            },
        }
    });
    const keyring = new Keyring({type: 'sr25519'});

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
    const purchasePrimeOriginOfShells = api.createType('StatusType', 'PurchasePrimeOriginOfShells');
    const preorderOriginOfShells = api.createType('StatusType', 'PreorderOriginOfShells');

    // OriginOfShellTypes
    const legendary = api.createType('OriginOfShellType', 'Legendary');
    const magic = api.createType('OriginOfShellType', 'Magic');
    const prime = api.createType('OriginOfShellType', 'Prime');

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

    // Create OverlordMessage for RedeemSpirit
    {
        const messageType = api.createType('MessageType', 'RedeemSpirit');
        const overlordMessage = api.createType('OverlordMessage', {'account': ferdie.address, 'purpose': messageType});
        const metadataSig = overlord.sign(overlordMessage.toU8a());
        //const isValid = overlord.verify(overlordMessage, metadataSig, overlord.publicKey);

        // Mint a Spirit
        //await api.tx.pwNftSale.claimSpirit().signAndSend(user);
        await api.tx.pwNftSale.redeemSpirit(metadataSig, overlordMessage).signAndSend(ferdie);
    }

    // mint spirit nft
    {
        // const serialId = 1;
        // const signature = '0xAABB';
        // const metadata = '0xCCDD'
        // const metadataSig = overlord.sign(metadata);
        // u8aToHex(metadataSig);
        // await api.tx.pwNftSale.claimSpirit(null, nftSignedMetadata).signAndSend(user);
    }

    // purchase rare origin of shell
    {
        // OriginOfShellType ['Legendary', 'Magic', 'Prime']
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        // metadata '0x2813308004'
        // const metadataSig = overlord.sign(metadata);
        // u8aToHex(metadataSig);
        await api.tx.pwNftSale.buyRareOriginOfShell('Legendary', 'Cyborg', 'HackerWizard')
            .signAndSend(user);
    }

    // purchase whitelist prime origin of shell
    {
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        // whitelistClaim createType('(AccountId,Vec<u8>)', [claimer, metadata]);
        // const sig = overlord.sign(message.toU8a());
        // u8aToHex(sig);
        // metadata '0x2813308004'
        // const metadataSig = overlord.sign(metadata);
        // u8aToHex(metadataSig);
        const messageType = api.createType('MessageType', 'Whitelist');
        const overlordMessage = api.createType('OverlordMessage', {'account': ferdie.address, 'purpose': messageType});
        const overlordSig = overlord.sign(overlordMessage.toU8a());

        //await api.tx.pwNftSale.setStatusType(true, 'PurchasePrimeOriginOfShells')
        //    .signAndSend(overlord, {nonce: -1});
        // Mint Prime Origin of Shell
        await api.tx.pwNftSale.buyPrimeOriginOfShell(overlordSig, overlordMessage, 'Cyborg', 'HackerWizard')
            .signAndSend(ferdie);
    }

    // preorder origin of shell
    {
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        await api.tx.pwNftSale.preorderOriginOfShell('Pandroid', 'HackerWizard')
            .signAndSend(ferdie);
    }

    // privileged function preorder status to declare Chosen or NotChosen preorders
    {
        // Preorder id
        // status ['Chosen', 'NotChosen']
        await api.tx.pwNftSale.setPreorderStatus()
            .signAndSend(overlord);
    }

    // Claim chosen preorders
    {
        await api.tx.pwNftSale.claimChosenPreorders()
            .signAndSend(user);
    }

    // Claim refund for not chosen preorders
    {
        await api.tx.pwNftSale.claimRefundPreorders()
            .signAndSend(user);
    }

    // Update the Prime Origin of Shell NFTs based on the number of Whitelist NFTs claimed
    // This is called AFTER the Whitelist Sale is complete. Must Disable Whitelist sale before updating to ensure numbers
    // do not fluctuate.
    {
        await api.tx.pwNftSale.updateOriginOfShellTypeCounts('Prime', 900, 50).signAndSend(overlord);
    }
}