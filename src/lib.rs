#![feature(const_trait_impl, derive_const)]
#![allow(dead_code, clippy::inline_always)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::mem::MaybeUninit;

use arrayvec::copy::ArrayVecCopy;
use const_for::const_for;
use crate::item::Item;
use crate::item::Item::*;
use crate::random::chunkrand::ChunkRand;
use crate::random::mcversion::{MCVersion, V1_16_5};

#[global_allocator]
static ALLOCATOR: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

pub mod item;
pub mod random;

pub type InventoryVec<T> = ArrayVecCopy<T, 27>;

pub const MC_VERSION: MCVersion = V1_16_5;

pub fn check_seed(seed: u64, chunk_x: i32, chunk_z: i32) -> bool {
    get_loot(
        i64::from_be_bytes(seed.to_be_bytes()),
        chunk_x,
        chunk_z,
        true,
    )
    .is_some_and(|x| x == TARGET_ITEMS)
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

macro_rules! item_count {
    ($item:expr) => {
        {
            let mut res = 0;
            const_for!(i in (0..TARGET_ITEMS.len()) => {
                let item_stack = unsafe { TARGET_ITEMS.xs[i].assume_init() };
                if item_stack.0.equals(&$item) {
                    res += item_stack.1;
                }
            });
            res
        }
    };
}

const IRON_INGOT_COUNT: usize = item_count!(IronIngot);
const GOLD_INGOT_COUNT: usize = item_count!(GoldIngot);
const TNT_COUNT: usize = item_count!(TNT);
const EMERALD_COUNT: usize = item_count!(Emerald);
const DIAMOND_COUNT: usize = item_count!(Diamond);
const PRISMARINE_COUNT: usize = item_count!(PrismarineCrystals);
const CHESTPLATE_COUNT: usize = item_count!(LeatherChestplate);
const SWORD_COUNT: usize = item_count!(IronSword);
const COOKED_COD_COUNT: usize = item_count!(CookedCod);
const COOKED_SALMON_COUNT: usize = item_count!(CookedSalmon);

const EXIT_EARLY: bool = cfg!(feature = "exit-early");

macro_rules! exit_function {
    () => {
        return None;
    };
}

fn generate_buried_treasure_loot(
    mut rand: ChunkRand,
    indexed: bool,
) -> Option<InventoryVec<ItemStack>> {
    let mut loot = InventoryVec::new();
    loot.push(ItemStack(HeartOfTheSea, 1));
    let rolls = get_count(&mut rand, 5, 8);
    let mut iron_ingot_count = 0;
    let mut gold_ingot_count = 0;
    let mut tnt_count = 0;
    for _ in 0..rolls {
        let weight = rand.get_next_int_bound(35);
        if weight < 20 {
            if EXIT_EARLY && IRON_INGOT_COUNT == 0 {
                exit_function!();
            }
            let value = get_count(&mut rand, 1, 4) as usize;
            iron_ingot_count += value;
            loot.push(ItemStack(IronIngot, value));
        } else if weight < 30 {
            if EXIT_EARLY && GOLD_INGOT_COUNT == 0 {
                exit_function!();
            }
            let value = get_count(&mut rand, 1, 4) as usize;
            gold_ingot_count += value;
            loot.push(ItemStack(GoldIngot, value));
        } else {
            if EXIT_EARLY && TNT_COUNT == 0 {
                exit_function!();
            }
            let value = get_count(&mut rand, 1, 2) as usize;
            tnt_count += value;
            loot.push(ItemStack(TNT, value));
        }
    }
    if EXIT_EARLY
        && (iron_ingot_count != IRON_INGOT_COUNT
            || gold_ingot_count != GOLD_INGOT_COUNT
            || tnt_count != TNT_COUNT)
    {
        exit_function!();
    }
    let rolls = get_count(&mut rand, 1, 3);
    let mut emerald_count = 0;
    let mut diamond_count = 0;
    let mut prismarine_count = 0;
    for _ in 0..rolls {
        let weight = rand.get_next_int_bound(15);
        if weight < 5 {
            if EXIT_EARLY && EMERALD_COUNT == 0 {
                exit_function!();
            }
            let value = get_count(&mut rand, 4, 8) as usize;
            emerald_count += value;
            loot.push(ItemStack(Emerald, value));
        } else if weight < 10 {
            if EXIT_EARLY && DIAMOND_COUNT == 0 {
                exit_function!();
            }
            let value = get_count(&mut rand, 1, 2) as usize;
            diamond_count += value;
            loot.push(ItemStack(Diamond, value));
        } else {
            if EXIT_EARLY && PRISMARINE_COUNT == 0 {
                exit_function!();
            }
            let value = get_count(&mut rand, 1, 5) as usize;
            prismarine_count += value;
            loot.push(ItemStack(PrismarineCrystals, value));
        }
    }

    if EXIT_EARLY
        && (emerald_count != EMERALD_COUNT
            || diamond_count != DIAMOND_COUNT
            || prismarine_count != PRISMARINE_COUNT)
    {
        exit_function!();
    }

    let should_roll = rand.get_next_bool();
    if EXIT_EARLY && (CHESTPLATE_COUNT != 0 || SWORD_COUNT != 0) != should_roll {
        exit_function!();
    }
    if should_roll {
        let weight = rand.get_next_int_bound(2);
        if weight < 1 {
            if EXIT_EARLY && CHESTPLATE_COUNT == 0 {
                exit_function!();
            }
            loot.push(ItemStack(LeatherChestplate, 1))
        } else {
            if EXIT_EARLY && SWORD_COUNT == 0 {
                exit_function!();
            }
            loot.push(ItemStack(IronSword, 1))
        }
    }

    let mut cooked_cod_count = 0;
    let mut cooked_salmon_count = 0;

    for _ in 0..2 {
        let weight = rand.get_next_int_bound(2);
        if weight < 1 {
            if EXIT_EARLY && COOKED_COD_COUNT == 0 {
                exit_function!();
            }
            let value = get_count(&mut rand, 2, 4) as usize;
            cooked_cod_count += value;
            loot.push(ItemStack(CookedCod, value));
        } else {
            if EXIT_EARLY && COOKED_SALMON_COUNT == 0 {
                exit_function!();
            }
            let value = get_count(&mut rand, 2, 4) as usize;
            cooked_salmon_count += value;
            loot.push(ItemStack(CookedSalmon, value));
        }
    }

    if EXIT_EARLY
        && (cooked_cod_count != COOKED_COD_COUNT || cooked_salmon_count != COOKED_SALMON_COUNT)
    {
        exit_function!();
    }

    if indexed {
        let mut container = DEFAULT_CONTAINER;
        rand.shuffle(&mut container);
        shuffle_items(&mut rand, loot, container)
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
) -> Option<InventoryVec<ItemStack>> {
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
        i -= 1;
        if item_stack.0 != Empty {
            result[*unsafe { container.get_unchecked(i) }] = *item_stack;
        }
    }
    Some(result)
}

fn get_loot(
    structure_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
    indexed: bool,
) -> Option<InventoryVec<ItemStack>> {
    let mut rand = ChunkRand::default();
    rand.set_decorator_seed_block_salt(structure_seed, chunk_x * 16, chunk_z * 16, 30001);
    let loot_rand = ChunkRand::new(rand.get_next_long());
    generate_buried_treasure_loot(loot_rand, indexed)
}
