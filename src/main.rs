use crate::items::Item;
use crate::items::Item::*;
use crate::random::chunkrand::ChunkRand;
use crate::random::mcversion::MCVersion;
use crate::random::mth::get_mask;
use std::ffi::c_int;
use std::time::Instant;

mod items;
mod random;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

// const STRUCT_TYPE: c_int = StructureType_Treasure as c_int;
// const MC: c_int = MCVersion_MC_1_16_5 as c_int;

fn main() {
    let chunk_x: c_int = -28;
    let chunk_z: c_int = -73;

    // let mut g: MaybeUninit<Generator> = MaybeUninit::zeroed();
    // unsafe { setupGenerator(g.as_mut_ptr(), MC, 0) };
    let time = Instant::now();
    for lower48 in 0..get_mask(48) as u64 {
        if lower48 % 2_u64.pow(24) == 0 {
            println!(
                "Progress: {:.2}%",
                (lower48 as f64 / get_mask(48) as f64) * 100.0
            );
        }
        if check_seed(lower48, chunk_x, chunk_z) {
            println!("Found seed `{lower48}` in {:.3?}", time.elapsed());
            break;
        }
    }
}

fn check_seed(seed: u64, chunk_x: i32, chunk_z: i32) -> bool {
    // let mut p: MaybeUninit<Pos> = MaybeUninit::zeroed();
    // if unsafe {
    //     getStructurePos(
    //         STRUCT_TYPE,
    //         MC,
    //         seed,
    //         chunk_x,
    //         chunk_z,
    //         p.as_mut_ptr(),
    //     ) != 0
    // } {
    //     return false;
    // }
    //
    // let p = unsafe { p.assume_init() };
    //
    // if p.x != chunk_x * 16 + if chunk_x.is_negative() { -9 } else { 9 } || p.z != chunk_z * 16 + if chunk_z.is_negative() { -9 } else { 9 } {
    //     return false;
    // }

    // for upper16 in 0..0x10000 {
    //     let seed = seed | (upper16 << 48);
    //     unsafe { applySeed(g.as_mut_ptr(), Dimension_DIM_OVERWORLD, seed as u64) }
    //     if unsafe { isViableStructurePos(STRUCT_TYPE, g.as_mut_ptr(), p.x, p.z, 0) != 0 } {
    //         println!("Seed {seed} has a Buried Treasure at ({}, {})", p.x, p.z);
    //         exit(0);
    //     }
    // }

    if get_loot(
        i64::from_be_bytes(seed.to_be_bytes()),
        chunk_x,
        chunk_z,
        true,
        *MCVersion::latest(),
    ) == TARGET_ITEMS
    {
        return true;
    };

    false
}

const TARGET_ITEMS: [ItemStack; 27] = [
    ItemStack(LeatherChestplate, 1),
    ItemStack(IronIngot, 2),
    ItemStack(IronIngot, 1),
    ItemStack(GoldIngot, 1),
    ItemStack(GoldIngot, 1),
    ItemStack(CookedCod, 1),
    ItemStack(Empty, 0),
    ItemStack(IronIngot, 1),
    ItemStack(IronIngot, 1),
    ItemStack(Diamond, 1),
    ItemStack(GoldIngot, 1),
    ItemStack(CookedCod, 2),
    ItemStack(GoldIngot, 1),
    ItemStack(Diamond, 1),
    ItemStack(HeartOfTheSea, 1),
    ItemStack(CookedSalmon, 1),
    ItemStack(IronIngot, 1),
    ItemStack(Emerald, 6),
    ItemStack(IronIngot, 1),
    ItemStack(GoldIngot, 1),
    ItemStack(CookedSalmon, 2),
    ItemStack(TNT, 1),
    ItemStack(CookedSalmon, 1),
    ItemStack(GoldIngot, 1),
    ItemStack(IronIngot, 1),
    ItemStack(GoldIngot, 1),
    ItemStack(Emerald, 1),
];

// const TARGET_ITEMS: [ItemStack; 27] = [
//     ItemStack(PrismarineCrystals, 1),
//     ItemStack(CookedCod, 3),
//     ItemStack(IronIngot, 1),
//     ItemStack(CookedCod, 1),
//     ItemStack(PrismarineCrystals, 1),
//     ItemStack(CookedCod, 1),
//     ItemStack(PrismarineCrystals, 1),
//     ItemStack(IronIngot, 1),
//     ItemStack(IronIngot, 1),
//     ItemStack(Empty, 0),
//     ItemStack(GoldIngot, 1),
//     ItemStack(GoldIngot, 1),
//     ItemStack(Empty, 0),
//     ItemStack(CookedCod, 1),
//     ItemStack(Empty, 0),
//     ItemStack(Empty, 0),
//     ItemStack(GoldIngot, 1),
//     ItemStack(Empty, 0),
//     ItemStack(CookedCod, 1),
//     ItemStack(HeartOfTheSea, 1),
//     ItemStack(IronSword, 1),
//     ItemStack(Empty, 0),
//     ItemStack(IronIngot, 1),
//     ItemStack(Empty, 0),
//     ItemStack(GoldIngot, 2),
//     ItemStack(GoldIngot, 1),
//     ItemStack(IronIngot, 1),
// ];

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
}

fn fake_bonus_roll() {}

fn get_float(rand: &mut ChunkRand, min: f32, max: f32) -> f32 {
    if min >= max {
        min
    } else {
        rand.get_next_float() * (max - min) + min
    }
}

fn generate_buried_treasure_loot(mut rand: ChunkRand, indexed: bool) -> Vec<ItemStack> {
    let mut loot = vec![];
    get_float(&mut rand, 0.0, 0.0);
    loot.push(ItemStack(HeartOfTheSea, 1));
    let rolls = get_count(&mut rand, 5, 8);
    get_float(&mut rand, 0.0, 0.0);
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
    get_float(&mut rand, 0.0, 0.0);
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
    let rolls = get_count(&mut rand, 0, 1);
    get_float(&mut rand, 0.0, 0.0);
    for _ in 0..rolls {
        let weight = rand.get_next_int_bound(2);
        if weight < 1 {
            loot.push(ItemStack(LeatherChestplate, 1))
        } else {
            loot.push(ItemStack(IronSword, 1))
        }
    }
    get_float(&mut rand, 0.0, 0.0);
    for _ in 0..2 {
        let weight = rand.get_next_int_bound(2);
        if weight < 1 {
            loot.push(ItemStack(CookedCod, get_count(&mut rand, 2, 4) as usize));
        } else {
            loot.push(ItemStack(CookedSalmon, get_count(&mut rand, 2, 4) as usize));
        }
    }

    if indexed {
        shuffle_items(&mut rand, loot)
    } else {
        loot
    }
}

fn shuffle_items(rand: &mut ChunkRand, items: Vec<ItemStack>) -> Vec<ItemStack> {
    let mut container = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26,
    ];
    rand.shuffle_test(&mut container);
    let mut list: Vec<ItemStack> = Vec::with_capacity(27);
    let mut new_items: Vec<ItemStack> = Vec::with_capacity(27);
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

    items.append(&mut list);
    rand.shuffle(&mut items);
    let mut result: Vec<ItemStack> = vec![ItemStack(Empty, 0); 27];
    for item_stack in &items {
        if container.is_empty() {
            return items;
        }

        if item_stack.1 == 0 || item_stack.0 == Empty {
            //result.insert(container.remove(container.len() - 1), ItemStack(Empty, 0));
        } else {
            result[container.remove(container.len() - 1)] = *item_stack;
        }
    }
    result
}

fn get_loot(
    structure_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
    indexed: bool,
    version: MCVersion,
) -> Vec<ItemStack> {
    let mut rand = ChunkRand::default();
    rand.set_decorator_seed_block_salt(structure_seed, chunk_x * 16, chunk_z * 16, 30001, version);
    let loot_rand = ChunkRand::new(rand.get_next_long());
    generate_buried_treasure_loot(loot_rand, indexed)
}
