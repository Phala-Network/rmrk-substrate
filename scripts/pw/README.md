# Phala World Scripts
## [0] Initial Configuration (Spirit Claim)
By default, the `init.js` script will assign the `root` account as `ALICE`, the `user` account as `BOB` and `overlord` account set as `BOB`. Running the `init.js` script performs the following:
> Execute the init.js script:
>```shell
>node ./init.js
>```
### Set Overlord account using sudo
Sudo transaction from `root` account aka `ALICE` that will call the privileged transaction `setOverlord` to set the `overlord` account to `BOB`
```javascript
await api.tx.sudo.sudo(
    api.tx.pwNftSale.setOverlord(overlord.address)
).signAndSend(root, {nonce: -1});
await sleep(6000);
```

### Initialize Phala World Clock
Overlord account then starts the Phala World Clock to initialize the `ZeroDay` Timestamp in transaction `initializeWorldClock()` to signify the official beginning of Phala World
```javascript
await api.tx.pwNftSale.initializeWorldClock()
    .signAndSend(overlord, {nonce: -1});
```

### Enable Spirit Claims
Overlord account will then enable the Spirit Claims for accounts to claim a non-transferable NFT representing their Spirit. This is done by calling transaction `setStatusType` with parameters `bool, StatusType` where `StatusType` is an enum containing `ClaimSpirits, PurchaseRareOriginOfShells, PurchaseHeroOriginOfShells, PreorderOriginOfShells, LastDayOfSale`. Here we use `ClaimSpirits` and set it to `true`.
```javascript
await api.tx.pwNftSale.setStatusType(true, 'ClaimSpirits')
    .signAndSend(overlord, {nonce: -1});
```

### Create Spirit & Origin of Shell Collection IDs
Overlord creates the Spirit & Origin of Shell Collection IDs for the two NFT collections then sets the Collection IDs for each in Storage
```javascript
// mint spirits NFTs with overlord
// collection 0: spirits
await api.tx.rmrkCore.createCollection(
    '0x',
    null,
    'PWSPRT'
).signAndSend(overlord, {nonce: -1});
// set the spirits collection id
await api.tx.pwNftSale.setSpiritCollectionId(
    0
).signAndSend(overlord, {nonce: -1});
// collection 1: origin of shells
await api.tx.rmrkCore.createCollection(
    '0x',
    null,
    'PWOAS'
).signAndSend(overlord, {nonce: -1});
// set the origin of shell collection id
await api.tx.pwNftSale.setOriginOfShellCollectionId(
    1
).signAndSend(overlord, {nonce: -1});
```

### Initialize the Inventory counts
In the `init.js` script there is a transaction that will set the starting inventory counts for the initial sales until the preorder phase. This script will populate the StorageDoubleMap called `originOfShellsInventory`.
```javascript
await api.tx.pwNftSale.initOriginOfShellTypeCounts().signAndSend(overlord, {nonce: -1});
```

### Signing Metadata
To avoid accounts transacting with the runtime directly via polkadot.js or scripts, the metadata is signed by the `overlord` account. This will allow for the backend to verify the metadata and proceed with minting of a given NFT. Here is an example of signing metadata and storing it in a new type called `NftSaleMetadata`. The `nftSignedMetadata` is used in the next section to claim a spirit.
```javascript
const metadata = 'I am Spirit';
const metadataType = api.createType('BoundedVec<u8, T::StringLimit>', metadata).toU8a();
const metadataSig = overlord.sign(metadataType);
const nftSignedMetadata = api.createType('NftSaleMetadata', {
    'metadata': metadataType,
    'signature': metadataSig
});
```

### Claim a Spirit
This is an example of generating the signed metadata for a Spirit NFT with `overlord` account then using the `ferdie` account to claim the spirit.
```javascript
const metadata = 'I am Spirit';
const metadataType = api.createType('BoundedVec<u8, T::StringLimit>', metadata).toU8a();
const metadataSig = overlord.sign(metadataType);
const isValid = overlord.verify(metadata, metadataSig, overlord.address);
const nftSignedMetadata = api.createType('NftSaleMetadata', {'metadata': metadataType, 'signature': metadataSig});
// Mint a Spirit
await api.tx.pwNftSale.claimSpirit(null, nftSignedMetadata).signAndSend(ferdie);
```

### Status Types
There are a few `StatusType` to note with different meanings. These status types can be changed by utilizing the `Overlord` admin account to execute a transaction called `setStatusType(bool, StatusType)`.
Here is an example of enabling Spirit claims:
```javascript
await api.tx.pwNftSale.setStatusType(true, 'ClaimSpirits')
    .signAndSend(overlord);
```
Next we will go into some details on the different `StatusType`:
- `ClaimSpirits`: Determines the status of the current Spirit Claim process. When this is set, there is a `bool` in storage to signify if Spirits can be claimed at the given moment.
- `PurchaseRareOriginOfShells`: Determines the status of Rare (Legendary or Magic) Origin of Shells being able to be purchased. A `bool` in storage represents the current status.
- `PurchaseHeroOriginOfShells`: Determines the status of Whitelist accounts being able to purchase a Hero Origin of Shell. This is mapped to a `bool` in storage to determine if Whitelist users can purchase.
- `PreorderOriginOfShells`: Determines the status of the Preorders for a chance to mint an Origin of Shell. A `bool` in storage represents the current status.
- `LastDayOfSale`: Determines if the last day of Origin of Shell sales is true and allows for unlimited purchases of Origin of Shell without previous restrictions based on available quantity.

## [1] Enable Rare Origin of Shells Sale
Next, there will be a phase that allows accounts to purchase a rare Origin of Shell NFT. First the `overlord` account will enable the `StatusType` `PurchaseRareOriginOfShells`.
```javascript
await api.tx.pwNftSale.setStatusType(true, 'PurchaseRareOriginOfShells')
    .signAndSend(overlord);
```
Here is an example of a user executing a transaction called `buyRareOriginOfShell` to purchase a rare Origin of Shell NFT. The Parameters can be as follows:
- `OriginOfShellType`: Origin of Shell Type and in this case the 2 acceptable values are `'Legendary'` or `'Magic'`.
- `RaceType`: A pick of any of the 4 Races `'Cyborg'`, `'AISpectre'`, `'Pandroid'`, `'XGene'`.
- `CareerType`: A pick of any of the 5 Careers `'HardwareDruid'`, `'RoboWarrior'`, `'TradeNegotiator'`, `'HackerWizard'`, `'Web3Monk'`.
- `NftSaleMetadata`: Metadata and the `Signature` from the `overlord` account to validate metdata.
```javascript
const metadata = 'I am Legendary';
const metadataType = api.createType('BoundedVec<u8, T::StringLimit>', metadata).toU8a();
const metadataSig = overlord.sign(metadataType);
const nftSignedMetadata = api.createType('NftSaleMetadata', {'metadata': metadataType, 'signature': metadataSig});
// Purchase rare Origin of Shell
await api.tx.pwNftSale.buyRareOriginOfShell('Legendary', 'Cyborg', 'HackerWizard', nftSignedMetadata)
    .signAndSend(user);
```

## [2] Enable Whitelist Sale
After the rare Origin of Shell purchases, we will then move to the Whitelist purchases. This will involve another validation effort by the `overlord` account signing some metadata along with the whitelisted account ID. This will be a new type called `WhitelistClaim` and will be passed into the transaction called `buyHeroOriginOfShell`. First, the `StatusType` `PurchaseHeroOriginOfShells` before proceeding.
```javascript
await api.tx.pwNftSale.setStatusType(true, 'PurchaseHeroOriginOfShells')
    .signAndSend(overlord);
```
Here is an example of creating a `WhitelistClaim` for the `ferdie` account. This is what `ferdie` will use to pass into the `buyHeroOriginOfShell` function.
```javascript
const metadata = 'Whitelist for FERDIE';
const metadataType = api.createType('BoundedVec<u8, T::StringLimit>', metadata);
const userMetadataType = api.createType('(AccountId,BoundedVec<u8, T::StringLimit>)', [ferdie.address, metadataType]).toU8a();
const metadataSign = overlord.sign(userMetadataType);
const whitelistType = api.createType('WhitelistClaim', {
    'account': ferdie.address,
    'metadata': metadataType,
    'signature': metadataSign,
});
```
This will enable `ferdie` to call `PurchaseHeroOriginOfShells` and here is an explanation of the valid parameters:
- `WhitelistClaim`: a `signature` of the `&(account, metadata)` by the `overlord` account to validate the whitelist claim by a given account.
- `RaceType`: A pick of any of the 4 Races `'Cyborg'`, `'AISpectre'`, `'Pandroid'`, `'XGene'`.
- `CareerType`: A pick of any of the 5 Careers `'HardwareDruid'`, `'RoboWarrior'`, `'TradeNegotiator'`, `'HackerWizard'`, `'Web3Monk'`.
- `NftSaleMetadata`: Metadata and the `Signature` from the `overlord` account to validate metdata.
```javascript
const metadataHero = 'I am Hero';
const metadataHeroType = api.createType('BoundedVec<u8, T::StringLimit>', metadataHero).toU8a();
const metadataHeroSig = overlord.sign(metadataHeroType);
const nftSignedMetadata = api.createType('NftSaleMetadata', {
    'metadata': metadataHeroType,
    'signature': metadataHeroSig
});
await api.tx.pwNftSale.buyHeroOriginOfShell(whitelistType, 'Cyborg', 'HackerWizard', nftSignedMetadata)
    .signAndSend(ferdie);
```

## [3] Enable Preorders of Origin of Shell
Preorders will be similar in simplicity like the rare Origin of Shell purchases. First, enable the `StatusType` `PreorderOriginOfShells`.
```javascript
await api.tx.pwNftSale.setStatusType(true, 'PreorderOriginOfShells')
    .signAndSend(overlord);
```
Here is an example of a Preorder transaction `preorderOriginOfShell`:
```javascript
const metadata = 'I am Hero';
const metadataType = api.createType('BoundedVec<u8, T::StringLimit>', metadata).toU8a();
const metadataSig = overlord.sign(metadataType);
const nftSignedMetadata = api.createType('NftSaleMetadata', {'metadata': metadataType, 'signature': metadataSig});
await api.tx.pwNftSale.preorderOriginOfShell('Pandroid', 'HackerWizard', nftSignedMetadata)
    .signAndSend(ferdie);
```
### After Preorders Are Finalized
After the preorders are finalized, disable the `StatusType` `PreorderOriginOfShells`. Then run a query on all the Preorders in storage.
```javascript
await api.tx.pwNftSale.setStatusType(false, 'PreorderOriginOfShells')
    .signAndSend(overlord);
// Query all preorders
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
```
Next create a script to randomly select `Chosen` Preorder IDs. This transaction will allow for the Preorder to change its `PreorderStatus`
- `PreorderId`: A number ID mapped to the preorder.
- `PreorderStatus`: A status with a value of either `Chosen` or `NotChosen`
```javascript
await api.tx.pwNftSale.setPreorderStatus(0, 'Chosen')
    .signAndSend(overlord);
```
After assigning the preorder statuses, query for all the Preorders sorted by account.
```javascript
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
```