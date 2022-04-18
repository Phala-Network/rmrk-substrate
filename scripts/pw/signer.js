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
            },
            NftSaleMetadata: {
                metadata: "BoundedVec<u8, T::StringLimit>",
                signature: "sr25519::Signature"
            }
        }
    });
    const keyring = new Keyring({type: 'sr25519'});
    // status types
    const claimSpirits = api.createType('StatusType', 'ClaimSpirits');
    const root = keyring.addFromUri(rootPrivkey);
    const user = keyring.addFromUri(userPrivkey);
    const overlord = keyring.addFromUri(overlordPrivkey);

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

main().catch(console.error).finally(() => process.exit());