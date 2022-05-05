# Phala World

## Types
```rust
pub enum OriginOfShellType {
    /// Origin of Shell type is Hero
    Hero,
    /// Origin of Shell type is Magic
    Magic,
    /// Origin of Shell is Legendary
    Legendary,
}

/// Four Races to choose from
#[derive(Encode, Decode, Clone, PartialEq)]
pub enum RaceType {
    Cyborg,
    AISpectre,
    XGene,
    Pandroid,
}

/// Five Careers to choose from
#[derive(Encode, Decode, Clone, PartialEq)]
pub enum CareerType {
    HardwareDruid,
    RoboWarrior,
    TradeNegotiator,
    HackerWizard,
    Web3Monk,
}
```

### Constants
```rust
/// Seconds per Era that will increment the Era storage value every interval
#[pallet::constant]
type SecondsPerEra: Get<u64>;
/// Price of Legendary Origin of Shell
#[pallet::constant]
type LegendaryOriginOfShellPrice: Get<BalanceOf<Self>>;
/// Price of Magic Origin of Shell
#[pallet::constant]
type MagicOriginOfShellPrice: Get<BalanceOf<Self>>;
/// Price of Hero Origin of Shell
#[pallet::constant]
type HeroOriginOfShellPrice: Get<BalanceOf<Self>>;
/// Max mint per Race
#[pallet::constant]
type MaxMintPerRace: Get<u32>;
/// Max mint per Career
#[pallet::constant]
type MaxMintPerCareer: Get<u32>;
/// Amount of food per Era
#[pallet::constant]
type FoodPerEra: Get<u8>;
/// Max food an Origin Of Shell can be fed per day
#[pallet::constant]
type MaxFoodFedPerEra: Get<u16>;
/// Max food to feed your own Origin Of Shell
#[pallet::constant]
type MaxFoodFeedSelf: Get<u8>;
```

## Storage
```rust
/// Stores all of the valid claimed spirits from the airdrop by serial id & bool true if claimed
#[pallet::storage]
#[pallet::getter(fn claimed_spirits)]
pub type ClaimedSpirits<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

/// Stores all of the valid claimed Origin Of Shells from the whitelist or preorder
#[pallet::storage]
#[pallet::getter(fn claimed_origin_of_shells)]
pub type ClaimedOriginOfShells<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

/// Preorder index that is the key to the Preorders StorageMap
#[pallet::storage]
#[pallet::getter(fn preorder_index)]
pub type PreorderIndex<T: Config> = StorageValue<_, PreorderId, ValueQuery>;

/// Preorder info map for user preorders
#[pallet::storage]
#[pallet::getter(fn preorders)]
pub type Preorders<T: Config> = StorageMap<_, Twox64Concat, PreorderId, PreorderInfoOf<T>>;

/// Food per Owner where an owner gets 5 food per era
#[pallet::storage]
#[pallet::getter(fn get_food_by_owner)]
pub type FoodByOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u8>;

/// Phala World Zero Day `BlockNumber` this will be used to determine Eras
#[pallet::storage]
#[pallet::getter(fn zero_day)]
pub(super) type ZeroDay<T: Config> = StorageValue<_, u64>;

/// The current Era from the initial ZeroDay BlockNumber
#[pallet::storage]
#[pallet::getter(fn era)]
pub type Era<T: Config> = StorageValue<_, u64, ValueQuery>;

/// Spirits can be claimed
#[pallet::storage]
#[pallet::getter(fn can_claim_spirits)]
pub type CanClaimSpirits<T: Config> = StorageValue<_, bool, ValueQuery>;

/// Rare Origin of Shells can be purchased
#[pallet::storage]
#[pallet::getter(fn can_purchase_rare_origin_of_shells)]
pub type CanPurchaseRareOriginOfShells<T: Config> = StorageValue<_, bool, ValueQuery>;

/// Origin of Shells can be preordered
#[pallet::storage]
#[pallet::getter(fn can_preorder_origin_of_shells)]
pub type CanPreorderOriginOfShells<T: Config> = StorageValue<_, bool, ValueQuery>;

/// Race Type count
#[pallet::storage]
#[pallet::getter(fn race_type_count)]
pub type RaceTypeLeft<T: Config> = StorageMap<_, Twox64Concat, RaceType, u32, ValueQuery>;

/// Race StorageMap count
#[pallet::storage]
#[pallet::getter(fn career_type_count)]
pub type CareerTypeLeft<T: Config> = StorageMap<_, Twox64Concat, CareerType, u32, ValueQuery>;

/// Overlord Admin account of Phala World
#[pallet::storage]
#[pallet::getter(fn overlord)]
pub(super) type Overlord<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

/// Spirit Collection ID
#[pallet::storage]
#[pallet::getter(fn spirit_collection_id)]
pub type SpiritCollectionId<T: Config> = StorageValue<_, CollectionId, OptionQuery>;

/// Origin of Shell Collection ID
#[pallet::storage]
#[pallet::getter(fn origin_of_shell_collection_id)]
pub type OriginOfShellCollectionId<T: Config> = StorageValue<_, CollectionId, OptionQuery>;
```

## Errors
```rust
/// Errors displayed to inform users something went wrong
#[pallet::error]
pub enum Error<T> {
    WorldClockAlreadySet,
    SpiritClaimNotAvailable,
    RareOriginOfShellPurchaseNotAvailable,
    PreorderOriginOfShellNotAvailable,
    SpiritAlreadyClaimed,
    ClaimVerificationFailed,
    InvalidPurchase,
    NoAvailablePreorderId,
    RaceMintMaxReached,
    CareerMintMaxReached,
    CannotHatchOriginOfShell,
    CannotSendFoodToOriginOfShell,
    NoFoodAvailable,
    OverlordNotSet,
    RequireOverlordAccount,
    InvalidStatusType,
    SpiritCollectionNotSet,
    SpiritCollectionIdAlreadySet,
    OriginOfShellCollectionNotSet,
    OriginOfShellCollectionIdAlreadySet,
}
```

## Calls
### claim_spirit
Claim a spirit for users that are on the whitelist.
```rust
origin: OriginFor<T>,
ticket: Option<ClaimSpiritTicket<T::AccountId>>,
metadata: NftSaleMetadata<BoundedVec<u8, T::StringLimit>>,
```

### buy_rare_origin_of_shell
Buy a rare origin of shell of either type Magic or Legendary.
```rust
origin: OriginFor<T>,
origin_of_shell_type: OriginOfShellType,
race: RaceType,
career: CareerType,
metadata: NftSaleMetadata<BoundedVec<u8, T::StringLimit>>,
```

### purchase_hero_origin_of_shell
```rust
origin: OriginFor<T>,
whitelist_claim: WhitelistClaim<T::AccountId, BoundedVec<u8, T::StringLimit>>,
race: RaceType,
career: CareerType,
metadata: NftSaleMetadata<BoundedVec<u8, T::StringLimit>>,
```


### preorder_origin_of_shell
Preorder an OriginOfShell for eligible users
```rust
origin: OriginFor<T>,
race: RaceType,
career: CareerType,
metadata: NftSaleMetadata<BoundedVec<u8, T::StringLimit>>,
```

### mint_origin_of_shells
This is an admin only function that will be called to do a bulk minting of all preordered origin of shell
```rust
origin: OriginFor<T>
```

### start_incubation
Initiate the incubation phase for an owner's OriginOfShell
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
nft_id: NftId,
```

### feed_origin_of_shell
Feed another origin of shell to the current origin of shell being incubated.
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
nft_id: NftId,
```

### awaken_origin_of_shell
Hatch the origin of shell that is currently being incubated.
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
nft_id: NftId,
```

### update_incubation_time
This is an admin function to update origin of shells incubation times based on being in the top 10 of fed origin of shells within that era
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
nft_id: NftId,
reduced_time: u64,
```

### set_overlord
Privileged function set the Overlord Admin account of Phala World.
```rust
origin: OriginFor<T>,
new_overlord: T::AccountId,
```

### initialize_world_clock
Privileged function where Phala World Zero Day is set to begin the tracking of the official time starting at the current timestamp.
```rust
origin: OriginFor<T>,
```

### set_status_type
Privileged function to set the status for one of the defined StatusTypes like ClaimSpirits, PurchaseRareOriginOfShells, or PreorderOriginOfShells
```rust
origin: OriginFor<T>,
status: bool,
status_type: StatusType,
```

### set_spirit_collection_id
Privileged function to set the collection id for the Spirits collection
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
```

### set_origin_of_shell_collection_id
Privileged function to set the collection id for the OriginOfShell collection
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
```