#![feature(const_trait_impl, derive_const)]

use std::mem::MaybeUninit;

use arrayvec::copy::ArrayVecCopy;
use const_for::const_for;
use static_assertions::const_assert_eq;

use crate::item::Item;
use crate::item::Item::*;
use crate::random::chunkrand::ChunkRand;
use crate::random::mcversion::MCVersion;

#[global_allocator]
static ALLOCATOR: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

pub mod item;
pub mod random;

pub type InventoryVec<T> = ArrayVecCopy<T, 27>;

pub fn check_seed(seed: u64, chunk_x: i32, chunk_z: i32) -> bool {
    get_loot(
        i64::from_be_bytes(seed.to_be_bytes()),
        chunk_x,
        chunk_z,
        true,
        *MCVersion::latest(),
    )
    .map(|x| x == TARGET_ITEMS)
    .unwrap_or(false)
}

#[inline(always)]
fn get_count(rand: &mut ChunkRand, min: i32, max: i32) -> i32 {
    if min >= max {
        min
    } else {
        rand.get_next_int_bound(max - min + 1) + min
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ItemStack(Item, usize);

impl ItemStack {
    #[inline(always)]
    fn get_count(&self) -> usize {
        self.1
    }

    fn split(&mut self, count: i32) -> ItemStack {
        let split_count = count.min(self.get_count() as i32);
        let item_stack = ItemStack(self.0, split_count as usize);
        self.1 -= split_count as usize;
        item_stack
    }

    const fn is_empty(&self) -> bool {
        (self.0 as isize) == (Empty as isize) || self.1 == 0
    }
}

// -4872636734044769429
const TARGET_ITEMS: InventoryVec<ItemStack> = InventoryVec {
    len: 27,
    xs: [
        MaybeUninit::new(ItemStack(LeatherChestplate, 1)),
        MaybeUninit::new(ItemStack(IronIngot, 2)),
        MaybeUninit::new(ItemStack(IronIngot, 1)),
        MaybeUninit::new(ItemStack(GoldIngot, 1)),
        MaybeUninit::new(ItemStack(GoldIngot, 1)),
        MaybeUninit::new(ItemStack(CookedCod, 1)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(IronIngot, 1)),
        MaybeUninit::new(ItemStack(IronIngot, 1)),
        MaybeUninit::new(ItemStack(Diamond, 1)),
        MaybeUninit::new(ItemStack(GoldIngot, 1)),
        MaybeUninit::new(ItemStack(CookedCod, 2)),
        MaybeUninit::new(ItemStack(GoldIngot, 1)),
        MaybeUninit::new(ItemStack(Diamond, 1)),
        MaybeUninit::new(ItemStack(HeartOfTheSea, 1)),
        MaybeUninit::new(ItemStack(CookedSalmon, 1)),
        MaybeUninit::new(ItemStack(IronIngot, 1)),
        MaybeUninit::new(ItemStack(Emerald, 6)),
        MaybeUninit::new(ItemStack(IronIngot, 1)),
        MaybeUninit::new(ItemStack(GoldIngot, 1)),
        MaybeUninit::new(ItemStack(CookedSalmon, 2)),
        MaybeUninit::new(ItemStack(TNT, 1)),
        MaybeUninit::new(ItemStack(CookedSalmon, 1)),
        MaybeUninit::new(ItemStack(GoldIngot, 1)),
        MaybeUninit::new(ItemStack(IronIngot, 1)),
        MaybeUninit::new(ItemStack(GoldIngot, 1)),
        MaybeUninit::new(ItemStack(Emerald, 1)),
    ],
};

const HEART_OF_THE_SEA_INDEX: usize = {
    const_assert_eq!(TARGET_ITEMS.len(), 27);
    let mut index: Option<usize> = None;
    const_for!(i in (0..TARGET_ITEMS.len()) => {
        if unsafe { TARGET_ITEMS.xs[i].assume_init().0 }.equals(&HeartOfTheSea) {
            index = Some(i);
            break;
        }
    });
    match index {
        None => panic!("Invalid loot"),
        Some(_) => 0,
    }
};

const CHESTPLATE_OR_SWORD_INDEX: Option<(usize, Item)> = {
    const_assert_eq!(TARGET_ITEMS.len(), 27);
    let mut res: Option<(usize, Item)> = None;
    const_for!(i in (0..TARGET_ITEMS.len()) => {
        let item = unsafe { TARGET_ITEMS.xs[i].assume_init() }.0;
        if item.equals(&LeatherChestplate) || item.equals(&IronSword) {
            res = Some((i, item));
            break;
        }
    });
    res
};

fn generate_buried_treasure_loot(
    mut rand: ChunkRand,
    indexed: bool,
) -> Option<InventoryVec<ItemStack>> {
    let mut loot = InventoryVec::new();
    loot.push(ItemStack(HeartOfTheSea, 1));
    let rolls = get_count(&mut rand, 5, 8);
    for _ in 0..rolls {
        let weight = rand.get_next_int_bound(35);
        if weight < 20 {
            loot.push(ItemStack(IronIngot, get_count(&mut rand, 1, 4) as usize));
        } else if weight < 30 {
            loot.push(ItemStack(GoldIngot, get_count(&mut rand, 1, 4) as usize));
        } else {
            loot.push(ItemStack(TNT, get_count(&mut rand, 1, 2) as usize));
        }
    }
    let rolls = get_count(&mut rand, 1, 3);
    for _ in 0..rolls {
        let weight = rand.get_next_int_bound(15);
        if weight < 5 {
            loot.push(ItemStack(Emerald, get_count(&mut rand, 4, 8) as usize));
        } else if weight < 10 {
            loot.push(ItemStack(Diamond, get_count(&mut rand, 1, 2) as usize));
        } else {
            loot.push(ItemStack(
                PrismarineCrystals,
                get_count(&mut rand, 1, 5) as usize,
            ));
        }
    }

    let should_roll = rand.get_next_bool();
    if cfg!(feature = "exit-early") && CHESTPLATE_OR_SWORD_INDEX.is_some() != should_roll {
        return None;
    }
    if should_roll {
        let weight = rand.get_next_int_bound(2);
        if weight < 1 {
            if cfg!(feature = "exit-early")
                && CHESTPLATE_OR_SWORD_INDEX.unwrap().1.equals(&IronSword)
            {
                return None;
            }
            loot.push(ItemStack(LeatherChestplate, 1))
        } else {
            if cfg!(feature = "exit-early")
                && CHESTPLATE_OR_SWORD_INDEX
                    .unwrap()
                    .1
                    .equals(&LeatherChestplate)
            {
                return None;
            }
            loot.push(ItemStack(IronSword, 1))
        }
    }

    for _ in 0..2 {
        let weight = rand.get_next_int_bound(2);
        if weight < 1 {
            loot.push(ItemStack(CookedCod, get_count(&mut rand, 2, 4) as usize));
        } else {
            loot.push(ItemStack(CookedSalmon, get_count(&mut rand, 2, 4) as usize));
        }
    }

    if indexed {
        let mut container = DEFAULT_CONTAINER;
        rand.shuffle(&mut container);
        Some(shuffle_items(&mut rand, loot, container))
    } else {
        Some(loot)
    }
}

const DEFAULT_CONTAINER: InventoryVec<usize> = InventoryVec {
    len: 27,
    xs: [
        MaybeUninit::new(0),
        MaybeUninit::new(1),
        MaybeUninit::new(2),
        MaybeUninit::new(3),
        MaybeUninit::new(4),
        MaybeUninit::new(5),
        MaybeUninit::new(6),
        MaybeUninit::new(7),
        MaybeUninit::new(8),
        MaybeUninit::new(9),
        MaybeUninit::new(10),
        MaybeUninit::new(11),
        MaybeUninit::new(12),
        MaybeUninit::new(13),
        MaybeUninit::new(14),
        MaybeUninit::new(15),
        MaybeUninit::new(16),
        MaybeUninit::new(17),
        MaybeUninit::new(18),
        MaybeUninit::new(19),
        MaybeUninit::new(20),
        MaybeUninit::new(21),
        MaybeUninit::new(22),
        MaybeUninit::new(23),
        MaybeUninit::new(24),
        MaybeUninit::new(25),
        MaybeUninit::new(26),
    ],
};

const DEFAULT_INVENTORY: InventoryVec<ItemStack> = InventoryVec {
    len: 27,
    xs: [
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
        MaybeUninit::new(ItemStack(Empty, 0)),
    ],
};

fn shuffle_items(
    rand: &mut ChunkRand,
    items: InventoryVec<ItemStack>,
    container: InventoryVec<usize>,
) -> InventoryVec<ItemStack> {
    let mut list = InventoryVec::new();
    let mut new_items = InventoryVec::new();
    let size = 27;
    for item_stack in items.iter() {
        if item_stack.1 > 1 {
            list.push(*item_stack);
        } else {
            new_items.push(*item_stack);
        }
    }

    let mut items = new_items;

    while size - items.len() - list.len() > 0 && !list.is_empty() {
        let mut item_stack2 = list.remove(get_count(rand, 0, (list.len() - 1) as i32) as usize);
        let half = item_stack2.1 / 2;
        let i = get_count(rand, 1, half as i32);
        let item_stack1 = item_stack2.split(i);
        if item_stack2.get_count() > 1 && rand.get_next_bool() {
            list.push(item_stack2);
        } else {
            items.push(item_stack2)
        }

        if item_stack1.get_count() > 1 && rand.get_next_bool() {
            list.push(item_stack1);
        } else {
            items.push(item_stack1)
        }
    }

    items
        .try_extend_from_slice(&list)
        .expect("ArrayVec should have enough capacity");
    rand.shuffle(&mut items);
    let mut result = DEFAULT_INVENTORY;
    let mut i = container.len();
    for item_stack in &items {
        if item_stack.0 != Empty {
            result[*unsafe { container.get_unchecked(i - 1) }] = *item_stack;
        }
        i -= 1;
    }
    result
}

fn get_loot(
    structure_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
    indexed: bool,
    version: MCVersion,
) -> Option<InventoryVec<ItemStack>> {
    let mut rand = ChunkRand::default();
    rand.set_decorator_seed_block_salt(structure_seed, chunk_x * 16, chunk_z * 16, 30001, version);
    let loot_rand = ChunkRand::new(rand.get_next_long());
    generate_buried_treasure_loot(loot_rand, indexed)
}