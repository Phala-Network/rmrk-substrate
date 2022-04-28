# Phala World Scripts
## Initial Configuration (Spirit Claim)
By default, the `init.js` script will assign the `root` account as `ALICE`, the `user` account as `BOB` and `overlord` account set as `BOB`. Running the `init.js` script performs the following:
> Execute the init.js script:
>```shell
>node ./init.js
>```
### Set Overlord account using sudo
Sudo transaction from `root` account aka `ALICE` that will call the privileged transaction `setOverlord` to set the `overlord` account to `BOB`
```javascript
await api.tx.sudo.sudo(
    api.tx.phalaWorld.setOverlord(overlord.address)
).signAndSend(root, {nonce: -1});
await sleep(6000);
```

### Initialize Phala World Clock
Overlord account then starts the Phala World Clock to initialize the `ZeroDay` Timestamp in transaction `initializeWorldClock()` to signify the official beginning of Phala World
```javascript
await api.tx.phalaWorld.initializeWorldClock()
    .signAndSend(overlord, {nonce: -1});
```

### Enable Spirit Claims
Overlord account will then enable the Spirit Claims for accounts to claim a non-transferable NFT representing their Spirit. This is done by calling transaction `setStatusType` with parameters `bool, StatusType` where `StatusType` is an enum containing `ClaimSpirits, PurchaseRareOriginOfShells, PurchaseHeroOriginOfShells, PreorderOriginOfShells, LastDayOfSale`. Here we use `ClaimSpirits` and set it to `true`.
```javascript
await api.tx.phalaWorld.setStatusType(true, 'ClaimSpirits')
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
```

### Initialize the Inventory counts
In the `init.js` script there is a transaction that will set the starting inventory counts for the initial sales until the preorder phase. This script will populate the StorageDoubleMap called `originOfShellsInventory`.
```javascript
await api.tx.phalaWorld.initOriginOfShellTypeCounts().signAndSend(overlord, {nonce: -1});
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
await api.tx.phalaWorld.claimSpirit(null, nftSignedMetadata).signAndSend(ferdie);
```

## Status Types
There are a few `StatusType` to note with different meanings. These status types can be changed by utilizing the `Overlord` admin account to execute a transaction called `setStatusType(bool, StatusType)`.
Here is an example of enabling Spirit claims:
```javascript
await api.tx.phalaWorld.setStatusType(true, 'claimSpirits')
    .signAndSend(overlord, {nonce: -1});
```
Next we will go into some details on the different `StatusType`:
- `ClaimSpirits`: Determines the status of the current Spirit Claim process. When this is set, there is a `bool` in storage to signify if Spirits can be claimed at the given moment.
- `PurchaseRareOriginOfShells`: Determines the status of Rare (Legendary or Magic) Origin of Shells being able to be purchased. A `bool` in storage represents the current status.
- `PurchaseHeroOriginOfShells`: Determines the status of Whitelist accounts being able to purchase a Hero Origin of Shell. This is mapped to a `bool` in storage to determine if Whitelist users can purchase.
- `PreorderOriginOfShells`: Determines the status of the Preorders for a chance to mint an Origin of Shell. A `bool` in storage represents the current status.
- `LastDayOfSale`: Determines if the last day of Origin of Shell sales is true and allows for unlimited purchases of Origin of Shell without previous restrictions based on available quantity.