## Configuration
By default, the `init.js` script will assign the `root` account as `ALICE`, the `user` account as `BOB` and `overlord` account set as `BOB`. Running the `init.js` script performs the following:

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
await api.tx.phalaWorld.setStatusType(true, 'claimSpirits')
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