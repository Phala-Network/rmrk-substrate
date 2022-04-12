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
    const api = await ApiPromise.create({provider: wsProvider});
    const keyring = new Keyring({type: 'sr25519'});

    const root = keyring.addFromUri(rootPrivkey);
    const user = keyring.addFromUri(userPrivkey);
    const overlord = keyring.addFromUri(overlordPrivkey);

    // mint spirit nft
    {
        // const serialId = 1;
        // const signature = '0xAABB';
        // const metadata = '0xCCDD'
        await api.tx.phalaWorld.claimSpirit()
            .signAndSend(user);
    }

    // purchase rare origin of shell
    {
        // OriginOfShellType ['Legendary', 'Magic', 'Hero']
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        // metadata '0x2813308004'
        await api.tx.phalaWorld.buyRareOriginOfShell()
            .signAndSend(user);
    }

    // purchase whitelist hero origin of shell
    {
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        // mcp_id
        // signature
        // metadata '0x2813308004'
        await api.tx.phalaWorld.buyHeroOriginOfShell()
            .signAndSend(user);
    }

    // preorder origin of shell
    {
        // RaceType ['AISpectre', 'Cyborg', 'Pandroid', 'XGene']
        // CareerType ['HardwareDruid', 'HackerWizard', 'RoboWarrior', 'TradeNegotiator', 'Web3Monk']
        // metadata '0x2813308004'
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