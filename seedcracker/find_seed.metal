//
//  test.metal
//  seedcracker
//
//  Created by Isaac Bess on 12/23/24.
//

#include <metal_stdlib>
#include "rng.h"

using namespace metal;

constant int32_t BURIED_TREASURE_SALT = 10387320;
constant int32_t BURIED_TREASURE_DECORATOR_SALT = 30001;

constant int32_t CHUNK_X = -28;
constant int32_t CHUNK_Z = -73;

uint64_t inline getPopulationSeed(uint64_t worldSeed, int32_t x, int32_t z) {
    uint64_t seed;
    setSeed(&seed, worldSeed);
    int64_t a = nextLong(&seed) | 1;
    int64_t b = nextLong(&seed) | 1;
    
    seed = (((int64_t) x * a) + ((int64_t) z * b)) ^ worldSeed;
    return seed & MASK_48;
}

uint64_t inline getDecoratorSeed(uint64_t worldSeed, int32_t x, int32_t z, int32_t salt) {
    uint64_t seed = getPopulationSeed(worldSeed, x, z);
    setSeed(&seed, seed + (int64_t) salt);
    return seed & MASK_48;
}

//
// HELPER FUNCTIONS
//

bool inline canGenerateTreasureClean(uint64_t seed) {
    
    int32_t newSeed = (float)(int32_t)(((((((uint64_t) CHUNK_X * 341873128712 + (uint64_t) CHUNK_Z * 132897987541 + (int64_t) seed + 10387320) ^ JAVA_LCG_MULTIPLIER) & MASK_48) * JAVA_LCG_MULTIPLIER + JAVA_LCG_ADDEND) & MASK_48) >> 24);
    return newSeed < 167772.16;
}

bool canGenerateTreasure(uint64_t seed) {
    uint64_t s = CHUNK_X * 341873128712ULL + CHUNK_Z * 132897987541ULL + BURIED_TREASURE_SALT + seed;
    return nextFloat(&s) < 0.01;
}

int getCount(thread uint64_t *seed, int32_t min, int32_t max) {
    return nextInt(seed, max - min + 1) + min;
}

enum Item: char {
    HEART_OF_THE_SEA,
    IRON_INGOT,
    GOLD_INGOT,
    TNT,
    EMERALD,
    DIAMOND,
    PRISMARINE,
    LEATHER_CHESTPLATE,
    IRON_SWORD,
    COOKED_COD,
    COOKED_SALMON,
    EMPTY,
};

struct ItemStack {
    Item item;
    char count;
};

constant ItemStack TARGET_LOOT[27] = {
    { LEATHER_CHESTPLATE, 1 },
    { IRON_INGOT, 2 },
    { IRON_INGOT, 1 },
    { GOLD_INGOT, 1 },
    { GOLD_INGOT, 1 },
    { COOKED_COD, 1 },
    { EMPTY, 0 },
    { IRON_INGOT, 1 },
    { IRON_INGOT, 1 },
    { DIAMOND, 1 },
    { GOLD_INGOT, 1 },
    { COOKED_COD, 2 },
    { GOLD_INGOT, 1 },
    { DIAMOND, 1 },
    { HEART_OF_THE_SEA, 1 },
    { COOKED_SALMON, 1 },
    { IRON_INGOT, 1 },
    { EMERALD, 6 },
    { IRON_INGOT, 1 },
    { GOLD_INGOT, 1 },
    { COOKED_SALMON, 2 },
    { TNT, 1 },
    { COOKED_SALMON, 1 },
    { GOLD_INGOT, 1 },
    { IRON_INGOT, 1 },
    { GOLD_INGOT, 1 },
    { EMERALD, 1 },
};

constexpr char getItemCount(Item item) {
    char count = 0;
    for (char i = 0; i < 27; i++) {
        ItemStack itemStack = TARGET_LOOT[i];
        if (itemStack.item == item) {
            count += itemStack.count;
        }
    }
    return count;
}

#define COUNT_ITEMS(ITEM) constant char ITEM##_COUNT = getItemCount(ITEM);

COUNT_ITEMS(IRON_INGOT);
COUNT_ITEMS(GOLD_INGOT);
COUNT_ITEMS(TNT);
COUNT_ITEMS(EMERALD);
COUNT_ITEMS(DIAMOND);
COUNT_ITEMS(PRISMARINE);
COUNT_ITEMS(LEATHER_CHESTPLATE);
COUNT_ITEMS(IRON_SWORD);
COUNT_ITEMS(COOKED_COD);
COUNT_ITEMS(COOKED_SALMON);

#define CHECK_COUNT(ITEM)
//if (ITEM##_COUNT == 0) {    \
//return false;             \
//}

ItemStack inline splitItemstack(thread ItemStack *itemStack, char count) {
    char splitCount = min(count, itemStack->count);
    ItemStack newItemstack = { itemStack->item, splitCount };
    itemStack->count -= splitCount;
    return newItemstack;
}

bool isTargetLoot(ItemStack loot[27]) {
    for (int i = 0; i < 27; i++) {
        if (loot[i].item != TARGET_LOOT[i].item || loot[i].count != TARGET_LOOT[i].count) {
            return false;
        }
    }
    return true;
}

template <typename T>
class InventoryVec {
public:
    char len;
    T contents[27];
    
    void inline push(T item) {
        contents[len] = item;
        len++;
    };
    
    T inline pop() {
        T item = contents[len];
        len--;
        return item;
    };
    
    T inline remove(char index) {
        T removedValue = contents[index];
        
        for (char i = index; i < len - 1; i++) {
            contents[i] = contents[i + 1];
        }
        
        len--;
        
        return removedValue;
    }
    
    void shuffle(thread uint64_t *seed) {
        for (char i = len; i > 1; i--) {
            char index = (char) nextInt(seed, i);
            T tmp = contents[i - 1];
            contents[i - 1] = contents[index];
            contents[index] = tmp;
        }
    }
    
    void extendArray(InventoryVec other, char offset = 0) {
        if (other.len == 0) {
            return;
        }
        
        char max = min(other.len + offset, 27);
        for (char i = offset; i < max; i++) {
            contents[i] = other[i - offset];
            if (i > len) {
                i++;
            }
        }
    }
    
    void inline append(InventoryVec other) {
        extendArray(other, len);
    }
    
    T inline operator[](char index) const {
        return contents[index];
    }
};

bool shuffleChest(uint64_t seed, InventoryVec<ItemStack> loot) {
    InventoryVec<char> container = {
        27,
        {
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26
        }
    };
    
    container.shuffle(&seed);
    
    InventoryVec<ItemStack> list;
    
    InventoryVec<ItemStack> items;
    
    for (char i = 0; i < loot.len; i++) {
        ItemStack itemStack = loot[i];
        if (itemStack.count > 1) {
            list.push(itemStack);
        } else {
            items.push(itemStack);
        }
    }
    
    while ((27 - items.len - list.len) > 0 && list.len != 0) {
        ItemStack itemStack2 = list.remove((char) getCount(&seed, 0, (list.len - 1)));
        char half_ = itemStack2.count / 2;
        char i = getCount(&seed, 1, half_);
        ItemStack itemStack1 = splitItemstack(&itemStack2, i);
        if (itemStack2.count > 1 && nextBool(&seed)) {
            list.push(itemStack2);
        } else {
            items.push(itemStack2);
        }
        if (itemStack1.count > 1 && nextBool(&seed)) {
            list.push(itemStack1);
        } else {
            items.push(itemStack1);
        }
    }
    
    items.append(list);
    
    items.shuffle(&seed);
    ItemStack result[27] = {
        { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 }, { EMPTY, 0 },
    };
    char i = 27;
    for (char j = 0; j < items.len; j++) {
        i -= 1;
        ItemStack item_stack = items[j];
        if (item_stack.item != EMPTY) {
            result[container[i]] = item_stack;
        }
    }
    
    return isTargetLoot(result);
}

#define PUSH_ITEM(ITEM, COUNT) loot.push({ ITEM, COUNT });

bool isCorrectLoot(uint64_t structureSeed) {
    structureSeed = getDecoratorSeed(structureSeed, CHUNK_X * 16, CHUNK_Z * 16,
                                     BURIED_TREASURE_DECORATOR_SALT);
    // Loot seed
    uint64_t seed;
    setSeed(&seed, nextLong(&structureSeed));
    
    InventoryVec<ItemStack> loot = {
        1,
        { {HEART_OF_THE_SEA, 1} }
    };
    
    char rolls = getCount(&seed, 5, 8);
    
    char ironIngotCount = 0;
    char goldIngotCount = 0;
    char tntCount = 0;
    
    for (char i = 0; i < rolls; i++) {
        char weight = nextInt(&seed, 35);
        if (weight < 20) {
            CHECK_COUNT(IRON_INGOT);
            char value = (char) getCount(&seed, 1, 4);
            ironIngotCount += value;
            PUSH_ITEM(IRON_INGOT, value);
        } else if (weight < 30) {
            CHECK_COUNT(GOLD_INGOT);
            char value = (char) getCount(&seed, 1, 4);
            goldIngotCount += value;
            PUSH_ITEM(GOLD_INGOT, value);
        } else {
            CHECK_COUNT(TNT);
            char value = (char) getCount(&seed, 1, 2);
            tntCount += value;
            PUSH_ITEM(TNT, value);
        }
    }
    
    if (ironIngotCount != IRON_INGOT_COUNT || goldIngotCount != GOLD_INGOT_COUNT || tntCount != TNT_COUNT) {
        return false;
    }
    
    rolls = getCount(&seed, 1, 3);
    
    char emeraldCount = 0;
    char diamondCount = 0;
    char prismarineCount = 0;
    
    for (char i = 0; i < rolls; i++) {
        char weight = nextInt(&seed, 15);
        if (weight < 5) {
            CHECK_COUNT(EMERALD)
            char value = (char) getCount(&seed, 4, 8);
            emeraldCount += value;
            PUSH_ITEM(EMERALD, value);
        } else if (weight < 10) {
            CHECK_COUNT(DIAMOND)
            char value = (char) getCount(&seed, 1, 2);
            diamondCount += value;
            PUSH_ITEM(DIAMOND, value);
        } else {
            CHECK_COUNT(PRISMARINE)
            char value = (char) getCount(&seed, 1, 5);
            prismarineCount += value;
            PUSH_ITEM(PRISMARINE, value);
        }
    }
    
    if (emeraldCount != EMERALD_COUNT || prismarineCount != PRISMARINE_COUNT || diamondCount != DIAMOND_COUNT) {
        return false;
    }
    
    bool shouldRoll = nextBool(&seed);
    
    if ((LEATHER_CHESTPLATE_COUNT != 0 || IRON_SWORD_COUNT != 0) != shouldRoll) {
        return false;
    }
    
    if (shouldRoll) {
        char weight = nextInt(&seed, 2);
        if (weight < 1) {
            CHECK_COUNT(LEATHER_CHESTPLATE);
            PUSH_ITEM(LEATHER_CHESTPLATE, 1);
        } else {
            CHECK_COUNT(IRON_SWORD);
            PUSH_ITEM(IRON_SWORD, 1);
        }
    }
    
    char cookedCodCount = 0;
    char cookedSalmonCount = 0;
    
    for (char i = 0; i < 2; i++) {
        char weight = nextInt(&seed, 2);
        if (weight < 1) {
            CHECK_COUNT(COOKED_COD)
            char value = (char) getCount(&seed, 2, 4);
            cookedCodCount += value;
            PUSH_ITEM(COOKED_COD, value);
        } else {
            CHECK_COUNT(COOKED_SALMON)
            char value = (char) getCount(&seed, 2, 4);
            cookedSalmonCount += value;
            PUSH_ITEM(COOKED_SALMON, value);
        }
    }
    
    if (cookedCodCount != COOKED_COD_COUNT && cookedSalmonCount != COOKED_SALMON_COUNT) {
        return false;
    }
    
    return shuffleChest(seed, loot);
}

bool inline checkSeed(uint64_t seed) {
    if (!canGenerateTreasure(seed)) {
        return false;
    }
    return isCorrectLoot(seed);
}

// 13574107339664782187ul

constant uint64_t MAX_SEED [[function_constant(0)]];

[[kernel]]
void find_seed(
                device uint64_t *result [[buffer(0)]],
                device bool *exit_flag [[buffer(1)]],
                uint tid [[thread_position_in_grid]],
                uint threads_per_grid [[threads_per_grid]])
{
    if (tid == 1) {
        *exit_flag = false;
    }
    
    for (uint64_t lower48 = tid; lower48 < MAX_SEED - threads_per_grid; lower48 += threads_per_grid) {
//        if (*exit_flag) {
//            break;
//        }
        if (checkSeed(lower48 + 13574107339664782187ul - MAX_SEED + 1)) {
            result[0] = lower48;
            result[1]++;
            *exit_flag = true;
            break;
        }
    }
}
