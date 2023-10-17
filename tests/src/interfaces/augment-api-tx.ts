// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';
import type { Bytes, Compact, Option, Vec, bool, u128, u16, u32, u64 } from '@polkadot/types-codec';
import type { AnyNumber, IMethod, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, MultiAddress, Permill } from '@polkadot/types/interfaces/runtime';
import type { PalletUniquesDestroyWitness, RmrkSubstrateRuntimeOriginCaller, RmrkTraitsNftAccountIdOrCollectionNftTuple, RmrkTraitsPartEquippableList, RmrkTraitsPartPartType, RmrkTraitsResourceBasicResource, RmrkTraitsResourceComposableResource, RmrkTraitsResourceResourceInfoMin, RmrkTraitsResourceResourceTypes, RmrkTraitsResourceSlotResource, RmrkTraitsTheme, SpConsensusGrandpaEquivocationProof, SpCoreVoid, SpWeightsWeightV2Weight } from '@polkadot/types/lookup';

declare module '@polkadot/api-base/types/submittable' {
  export interface AugmentedSubmittables<ApiType extends ApiTypes> {
    balances: {
      /**
       * See [`Pallet::force_set_balance`].
       **/
      forceSetBalance: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, newFree: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::force_transfer`].
       **/
      forceTransfer: AugmentedSubmittable<(source: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::force_unreserve`].
       **/
      forceUnreserve: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, u128]>;
      /**
       * See [`Pallet::transfer_all`].
       **/
      transferAll: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, keepAlive: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, bool]>;
      /**
       * See [`Pallet::transfer_allow_death`].
       **/
      transferAllowDeath: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::transfer_keep_alive`].
       **/
      transferKeepAlive: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * See [`Pallet::upgrade_accounts`].
       **/
      upgradeAccounts: AugmentedSubmittable<(who: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<AccountId32>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    grandpa: {
      /**
       * See [`Pallet::note_stalled`].
       **/
      noteStalled: AugmentedSubmittable<(delay: u32 | AnyNumber | Uint8Array, bestFinalizedBlockNumber: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::report_equivocation`].
       **/
      reportEquivocation: AugmentedSubmittable<(equivocationProof: SpConsensusGrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: SpCoreVoid | null) => SubmittableExtrinsic<ApiType>, [SpConsensusGrandpaEquivocationProof, SpCoreVoid]>;
      /**
       * See [`Pallet::report_equivocation_unsigned`].
       **/
      reportEquivocationUnsigned: AugmentedSubmittable<(equivocationProof: SpConsensusGrandpaEquivocationProof | { setId?: any; equivocation?: any } | string | Uint8Array, keyOwnerProof: SpCoreVoid | null) => SubmittableExtrinsic<ApiType>, [SpConsensusGrandpaEquivocationProof, SpCoreVoid]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    rmrkCore: {
      /**
       * See [`Pallet::accept_nft`].
       **/
      acceptNft: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, newOwner: RmrkTraitsNftAccountIdOrCollectionNftTuple | { AccountId: any } | { CollectionAndNftTuple: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsNftAccountIdOrCollectionNftTuple]>;
      /**
       * See [`Pallet::accept_resource`].
       **/
      acceptResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * See [`Pallet::accept_resource_removal`].
       **/
      acceptResourceRemoval: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * See [`Pallet::add_basic_resource`].
       **/
      addBasicResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resource: RmrkTraitsResourceBasicResource | { metadata?: any } | string | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsResourceBasicResource, u32]>;
      /**
       * See [`Pallet::add_composable_resource`].
       **/
      addComposableResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resource: RmrkTraitsResourceComposableResource | { parts?: any; base?: any; metadata?: any; slot?: any } | string | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsResourceComposableResource, u32]>;
      /**
       * See [`Pallet::add_slot_resource`].
       **/
      addSlotResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resource: RmrkTraitsResourceSlotResource | { base?: any; metadata?: any; slot?: any } | string | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsResourceSlotResource, u32]>;
      /**
       * See [`Pallet::burn_nft`].
       **/
      burnNft: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::change_collection_issuer`].
       **/
      changeCollectionIssuer: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, newIssuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::create_collection`].
       **/
      createCollection: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, metadata: Bytes | string | Uint8Array, max: Option<u32> | null | object | string | Uint8Array, symbol: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes, Option<u32>, Bytes]>;
      /**
       * See [`Pallet::destroy_collection`].
       **/
      destroyCollection: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::lock_collection`].
       **/
      lockCollection: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::mint_nft`].
       **/
      mintNft: AugmentedSubmittable<(owner: Option<AccountId32> | null | object | string | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, collectionId: u32 | AnyNumber | Uint8Array, royaltyRecipient: Option<AccountId32> | null | object | string | Uint8Array, royalty: Option<Permill> | null | object | string | Uint8Array, metadata: Bytes | string | Uint8Array, transferable: bool | boolean | Uint8Array, resources: Option<Vec<RmrkTraitsResourceResourceInfoMin>> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<AccountId32>, u32, u32, Option<AccountId32>, Option<Permill>, Bytes, bool, Option<Vec<RmrkTraitsResourceResourceInfoMin>>]>;
      /**
       * See [`Pallet::mint_nft_directly_to_nft`].
       **/
      mintNftDirectlyToNft: AugmentedSubmittable<(owner: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], nftId: u32 | AnyNumber | Uint8Array, collectionId: u32 | AnyNumber | Uint8Array, royaltyRecipient: Option<AccountId32> | null | object | string | Uint8Array, royalty: Option<Permill> | null | object | string | Uint8Array, metadata: Bytes | string | Uint8Array, transferable: bool | boolean | Uint8Array, resources: Option<Vec<RmrkTraitsResourceResourceInfoMin>> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [ITuple<[u32, u32]>, u32, u32, Option<AccountId32>, Option<Permill>, Bytes, bool, Option<Vec<RmrkTraitsResourceResourceInfoMin>>]>;
      /**
       * See [`Pallet::reject_nft`].
       **/
      rejectNft: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::remove_resource`].
       **/
      removeResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * See [`Pallet::replace_resource`].
       **/
      replaceResource: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, resource: RmrkTraitsResourceResourceTypes | { Basic: any } | { Composable: any } | { Slot: any } | string | Uint8Array, resourceId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsResourceResourceTypes, u32]>;
      /**
       * See [`Pallet::send`].
       **/
      send: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, newOwner: RmrkTraitsNftAccountIdOrCollectionNftTuple | { AccountId: any } | { CollectionAndNftTuple: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsNftAccountIdOrCollectionNftTuple]>;
      /**
       * See [`Pallet::set_priority`].
       **/
      setPriority: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, priorities: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [u32, u32, Vec<u32>]>;
      /**
       * See [`Pallet::set_property`].
       **/
      setProperty: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, maybeNftId: Option<u32> | null | object | string | Uint8Array, key: Bytes | string | Uint8Array, value: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Option<u32>, Bytes, Bytes]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    rmrkEquip: {
      /**
       * See [`Pallet::change_base_issuer`].
       **/
      changeBaseIssuer: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, newIssuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::create_base`].
       **/
      createBase: AugmentedSubmittable<(baseType: Bytes | string | Uint8Array, symbol: Bytes | string | Uint8Array, parts: Vec<RmrkTraitsPartPartType> | (RmrkTraitsPartPartType | { FixedPart: any } | { SlotPart: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Bytes, Bytes, Vec<RmrkTraitsPartPartType>]>;
      /**
       * See [`Pallet::equip`].
       **/
      equip: AugmentedSubmittable<(item: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], equipper: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], resourceId: u32 | AnyNumber | Uint8Array, base: u32 | AnyNumber | Uint8Array, slot: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [ITuple<[u32, u32]>, ITuple<[u32, u32]>, u32, u32, u32]>;
      /**
       * See [`Pallet::equippable`].
       **/
      equippable: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, slotId: u32 | AnyNumber | Uint8Array, equippables: RmrkTraitsPartEquippableList | { All: any } | { Empty: any } | { Custom: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, RmrkTraitsPartEquippableList]>;
      /**
       * See [`Pallet::equippable_add`].
       **/
      equippableAdd: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, slotId: u32 | AnyNumber | Uint8Array, equippable: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * See [`Pallet::equippable_remove`].
       **/
      equippableRemove: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, slotId: u32 | AnyNumber | Uint8Array, equippable: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u32]>;
      /**
       * See [`Pallet::theme_add`].
       **/
      themeAdd: AugmentedSubmittable<(baseId: u32 | AnyNumber | Uint8Array, theme: RmrkTraitsTheme | { name?: any; properties?: any; inherit?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, RmrkTraitsTheme]>;
      /**
       * See [`Pallet::unequip`].
       **/
      unequip: AugmentedSubmittable<(item: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], unequipper: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array], base: u32 | AnyNumber | Uint8Array, slot: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [ITuple<[u32, u32]>, ITuple<[u32, u32]>, u32, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    rmrkMarket: {
      /**
       * See [`Pallet::accept_offer`].
       **/
      acceptOffer: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, offerer: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, AccountId32]>;
      /**
       * See [`Pallet::buy`].
       **/
      buy: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, amount: Option<u128> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Option<u128>]>;
      /**
       * See [`Pallet::list`].
       **/
      list: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, amount: u128 | AnyNumber | Uint8Array, expires: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u128, Option<u32>]>;
      /**
       * See [`Pallet::make_offer`].
       **/
      makeOffer: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array, amount: u128 | AnyNumber | Uint8Array, expires: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u128, Option<u32>]>;
      /**
       * See [`Pallet::unlist`].
       **/
      unlist: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::withdraw_offer`].
       **/
      withdrawOffer: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, nftId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    sudo: {
      /**
       * See [`Pallet::set_key`].
       **/
      setKey: AugmentedSubmittable<(updated: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * See [`Pallet::sudo`].
       **/
      sudo: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call]>;
      /**
       * See [`Pallet::sudo_as`].
       **/
      sudoAs: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Call]>;
      /**
       * See [`Pallet::sudo_unchecked_weight`].
       **/
      sudoUncheckedWeight: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array, weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, SpWeightsWeightV2Weight]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    system: {
      /**
       * See [`Pallet::kill_prefix`].
       **/
      killPrefix: AugmentedSubmittable<(prefix: Bytes | string | Uint8Array, subkeys: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, u32]>;
      /**
       * See [`Pallet::kill_storage`].
       **/
      killStorage: AugmentedSubmittable<(keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Bytes>]>;
      /**
       * See [`Pallet::remark`].
       **/
      remark: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::remark_with_event`].
       **/
      remarkWithEvent: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::set_code`].
       **/
      setCode: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::set_code_without_checks`].
       **/
      setCodeWithoutChecks: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * See [`Pallet::set_heap_pages`].
       **/
      setHeapPages: AugmentedSubmittable<(pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
      /**
       * See [`Pallet::set_storage`].
       **/
      setStorage: AugmentedSubmittable<(items: Vec<ITuple<[Bytes, Bytes]>> | ([Bytes | string | Uint8Array, Bytes | string | Uint8Array])[]) => SubmittableExtrinsic<ApiType>, [Vec<ITuple<[Bytes, Bytes]>>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    timestamp: {
      /**
       * See [`Pallet::set`].
       **/
      set: AugmentedSubmittable<(now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u64>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    uniques: {
      /**
       * See [`Pallet::approve_transfer`].
       **/
      approveTransfer: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, delegate: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, MultiAddress]>;
      /**
       * See [`Pallet::burn`].
       **/
      burn: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, checkOwner: Option<MultiAddress> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Option<MultiAddress>]>;
      /**
       * See [`Pallet::buy_item`].
       **/
      buyItem: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, bidPrice: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u128]>;
      /**
       * See [`Pallet::cancel_approval`].
       **/
      cancelApproval: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, maybeCheckDelegate: Option<MultiAddress> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Option<MultiAddress>]>;
      /**
       * See [`Pallet::clear_attribute`].
       **/
      clearAttribute: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, maybeItem: Option<u32> | null | object | string | Uint8Array, key: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Option<u32>, Bytes]>;
      /**
       * See [`Pallet::clear_collection_metadata`].
       **/
      clearCollectionMetadata: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::clear_metadata`].
       **/
      clearMetadata: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::create`].
       **/
      create: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * See [`Pallet::destroy`].
       **/
      destroy: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, witness: PalletUniquesDestroyWitness | { items?: any; itemMetadatas?: any; attributes?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PalletUniquesDestroyWitness]>;
      /**
       * See [`Pallet::force_create`].
       **/
      forceCreate: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freeHolding: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, bool]>;
      /**
       * See [`Pallet::force_item_status`].
       **/
      forceItemStatus: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, issuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freezer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freeHolding: bool | boolean | Uint8Array, isFrozen: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress, MultiAddress, MultiAddress, bool, bool]>;
      /**
       * See [`Pallet::freeze`].
       **/
      freeze: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::freeze_collection`].
       **/
      freezeCollection: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::mint`].
       **/
      mint: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, MultiAddress]>;
      /**
       * See [`Pallet::redeposit`].
       **/
      redeposit: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, items: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [u32, Vec<u32>]>;
      /**
       * See [`Pallet::set_accept_ownership`].
       **/
      setAcceptOwnership: AugmentedSubmittable<(maybeCollection: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<u32>]>;
      /**
       * See [`Pallet::set_attribute`].
       **/
      setAttribute: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, maybeItem: Option<u32> | null | object | string | Uint8Array, key: Bytes | string | Uint8Array, value: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Option<u32>, Bytes, Bytes]>;
      /**
       * See [`Pallet::set_collection_max_supply`].
       **/
      setCollectionMaxSupply: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, maxSupply: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::set_collection_metadata`].
       **/
      setCollectionMetadata: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, data: Bytes | string | Uint8Array, isFrozen: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes, bool]>;
      /**
       * See [`Pallet::set_metadata`].
       **/
      setMetadata: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, data: Bytes | string | Uint8Array, isFrozen: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Bytes, bool]>;
      /**
       * See [`Pallet::set_price`].
       **/
      setPrice: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, price: Option<u128> | null | object | string | Uint8Array, whitelistedBuyer: Option<MultiAddress> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Option<u128>, Option<MultiAddress>]>;
      /**
       * See [`Pallet::set_team`].
       **/
      setTeam: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, issuer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, admin: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, freezer: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress, MultiAddress, MultiAddress]>;
      /**
       * See [`Pallet::thaw`].
       **/
      thaw: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * See [`Pallet::thaw_collection`].
       **/
      thawCollection: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * See [`Pallet::transfer`].
       **/
      transfer: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, item: u32 | AnyNumber | Uint8Array, dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, MultiAddress]>;
      /**
       * See [`Pallet::transfer_ownership`].
       **/
      transferOwnership: AugmentedSubmittable<(collection: u32 | AnyNumber | Uint8Array, owner: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, MultiAddress]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    utility: {
      /**
       * See [`Pallet::as_derivative`].
       **/
      asDerivative: AugmentedSubmittable<(index: u16 | AnyNumber | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16, Call]>;
      /**
       * See [`Pallet::batch`].
       **/
      batch: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * See [`Pallet::batch_all`].
       **/
      batchAll: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * See [`Pallet::dispatch_as`].
       **/
      dispatchAs: AugmentedSubmittable<(asOrigin: RmrkSubstrateRuntimeOriginCaller | { system: any } | { Void: any } | string | Uint8Array, call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [RmrkSubstrateRuntimeOriginCaller, Call]>;
      /**
       * See [`Pallet::force_batch`].
       **/
      forceBatch: AugmentedSubmittable<(calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Call>]>;
      /**
       * See [`Pallet::with_weight`].
       **/
      withWeight: AugmentedSubmittable<(call: Call | IMethod | string | Uint8Array, weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, SpWeightsWeightV2Weight]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
  } // AugmentedSubmittables
} // declare module
