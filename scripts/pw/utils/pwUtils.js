require('dotenv').config();
const sleep = require('p-sleep');
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { stringToU8a, u8aToHex } = require('@polkadot/util');

const rootPrivkey = process.env.ROOT_PRIVKEY;
const userPrivkey = process.env.USER_PRIVKEY;
const overlordPrivkey = process.env.OVERLOAD_PRIVKEY;
const endpoint = process.env.ENDPOINT;

const keyring = new Keyring({type: 'sr25519'});
const overlord = keyring.addFromUri(overlordPrivkey);

function getSignedMetadata(metadata) {
    const metadataType = api.createType('BoundedVec<u8, T::StringLimit>', metadata);
    return overlord.sign(metadataType.toU8a());
}

function getSignedSpiritMetadata() {
    const metadata = 'I am Spirit';
    return getSignedMetadata(metadata);
}

function getSignedLegendaryMetadata() {
    const metadata = 'I am Legendary';
    return getSignedMetadata(metadata);
}

function getSignedMagicMetadata() {
    const metadata = 'I am Magic';
    return getSignedMetadata(metadata);
}

function getSignedHeroMetadata() {
    const metadata = 'I am Hero';
    return getSignedMetadata(metadata);
}