//
//  seedcracker.hpp
//  seedcracker
//
//  Created by Isaac Bess on 12/25/24.
//

#ifndef seedcracker_hpp
#define seedcracker_hpp

#include <stdio.h>
#include <stdint.h>
#include <algorithm>
#include "rng.hpp"
#include "cubiomes/finders.h"

typedef unsigned char uchar;

enum Item: uchar {
    NUL = 0,
    EMPTY = 1,
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
};

struct ItemStack {
    Item item;
    uchar count;
    bool const inline isEmpty() {
        return item < 2 || count < 1;
    };
    ItemStack const static inline null() {
        return { NUL, 0 };
    }
};

const ItemStack TARGET_LOOT[27] = {
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

constexpr uchar getItemCount(Item item) {
    uchar count = 0;
    for (uchar i = 0; i < 27; i++) {
        ItemStack itemStack = TARGET_LOOT[i];
        if (itemStack.item == item) {
            count += itemStack.count;
        }
    }
    return count;
}

#define COUNT_ITEMS(ITEM) const uchar ITEM##_COUNT = getItemCount(ITEM);

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

#define CHECK_COUNT(ITEM)   \
if (ITEM##_COUNT == 0) {    \
return false;               \
}

ItemStack inline splitItemstack(ItemStack *itemStack, uchar count) {
    uchar splitCount = std::min(count, itemStack->count);
    ItemStack newItemstack = { itemStack->item, splitCount };
    itemStack->count -= splitCount;
    return newItemstack;
}

template <typename T>
class InventoryVec {
public:
    uchar len;
    T contents[27];
    
    InventoryVec const static inline empty() {
        return { 0, {{ NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }, { NUL, 0 }} };
    }
    
    void inline push(T item) {
        contents[len] = item;
        len++;
    };
    
    T inline pop() {
        T item = contents[len];
        len--;
        return item;
    };
    
    T inline remove(uchar index) {
        T removedValue = contents[index];
        
        for (uchar i = index; i < len - 1; i++) {
            contents[i] = contents[i + 1];
        }
        
        len--;
        contents[len] = ItemStack::null();
        
        return removedValue;
    }
    
    void swap(uchar a, uchar b) {
        T pa = contents[a];
        T pb = contents[b];
        contents[b] = pa;
        contents[a] = pb;
    }
    
    void shuffle(int64_t *seed) {
        for (uchar i = len; i > 1; i--) {
            swap(i - 1, nextInt(seed, i));
        }
    }
    
    void extendArray(InventoryVec &other, uchar offset = 0) {
        if (other.len == 0) {
            return;
        }
        
        uchar max = std::min(other.len + offset, 27);
        for (uchar i = offset; i < max; i++) {
            contents[i] = other[i - offset];
            if (i > len) {
                i++;
            }
        }
    }
    
    void inline append(InventoryVec other) {
        extendArray(other, len);
    }
    
    T inline operator[](uchar index) const {
        return contents[index];
    }
};

bool shuffleChest(int64_t s, InventoryVec<ItemStack> loot) {
    int64_t seed = s;
    InventoryVec<uchar> container = InventoryVec<uchar> {
        27,
        {
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26
        }
    };
    
    container.shuffle(&seed);
    
    
    InventoryVec<ItemStack> list = InventoryVec<ItemStack>::empty();
    
    InventoryVec<ItemStack> items = InventoryVec<ItemStack>::empty();
    
    for (uchar i = 0; i < loot.len; i++) {
        ItemStack itemStack = loot[i];
        if (itemStack.count > 1 && !itemStack.isEmpty()) {
            list.push(itemStack);
        } else {
            items.push(itemStack);
        }
    }
    while ((27 - items.len - list.len) > 0 && list.len != 0) {
        uchar index = (uchar) getCount(&seed, 0, (list.len - 1));
        ItemStack itemStack2 = list.remove(index);
        uchar half_ = itemStack2.count / 2;
        uchar i = (uchar) getCount(&seed, 1, (int32_t) half_);
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
    uchar i = 27;
    for (uchar j = 0; j < items.len; j++) {
        i -= 1;
        uchar index = container[i];
        if (result[index].item == TARGET_LOOT[index].item || result[index].count == TARGET_LOOT[index].count) {
            return false;
        }
    }
    
    
    return true;
}

#define PUSH_ITEM(ITEM, COUNT) loot.push({ ITEM, COUNT });

bool isCorrectLoot(int64_t structureSeed, const int32_t chunk_x, const int32_t chunk_z) {
    
    int64_t seed = getDecoratorSeed(structureSeed, chunk_x * 16, chunk_z * 16,
                                    BURIED_TREASURE_DECORATOR_SALT);
    
    int64_t newSeed = nextLong(&seed);
    
    // Loot seed
    setSeed(&seed, newSeed);
    
    InventoryVec<ItemStack> loot = {
        1,
        { {HEART_OF_THE_SEA, 1} }
    };
    
    uchar rolls = getCount(&seed, 5, 8);
    
    uchar ironIngotCount = 0;
    uchar goldIngotCount = 0;
    uchar tntCount = 0;
    
    for (uchar i = 0; i < rolls; i++) {
        uchar weight = nextInt(&seed, 35);
        if (weight < 20) {
            CHECK_COUNT(IRON_INGOT);
            uchar value = (uchar) getCount(&seed, 1, 4);
            ironIngotCount += value;
            PUSH_ITEM(IRON_INGOT, value);
        } else if (weight < 30) {
            CHECK_COUNT(GOLD_INGOT);
            uchar value = (uchar) getCount(&seed, 1, 4);
            goldIngotCount += value;
            PUSH_ITEM(GOLD_INGOT, value);
        } else {
            CHECK_COUNT(TNT);
            uchar value = (uchar) getCount(&seed, 1, 2);
            tntCount += value;
            PUSH_ITEM(TNT, value);
        }
    }
    
    if (ironIngotCount != IRON_INGOT_COUNT || goldIngotCount != GOLD_INGOT_COUNT || tntCount != TNT_COUNT) {
        return false;
    }
    
    rolls = getCount(&seed, 1, 3);
    
    uchar emeraldCount = 0;
    uchar diamondCount = 0;
    uchar prismarineCount = 0;
    
    for (uchar i = 0; i < rolls; i++) {
        uchar weight = nextInt(&seed, 15);
        if (weight < 5) {
            CHECK_COUNT(EMERALD)
            uchar value = (uchar) getCount(&seed, 4, 8);
            emeraldCount += value;
            PUSH_ITEM(EMERALD, value);
        } else if (weight < 10) {
            CHECK_COUNT(DIAMOND)
            uchar value = (uchar) getCount(&seed, 1, 2);
            diamondCount += value;
            PUSH_ITEM(DIAMOND, value);
        } else {
            CHECK_COUNT(PRISMARINE)
            uchar value = (uchar) getCount(&seed, 1, 5);
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
        uchar weight = nextInt(&seed, 2);
        if (weight < 1) {
            CHECK_COUNT(LEATHER_CHESTPLATE);
            PUSH_ITEM(LEATHER_CHESTPLATE, 1);
        } else {
            CHECK_COUNT(IRON_SWORD);
            PUSH_ITEM(IRON_SWORD, 1);
        }
    }
    
    uchar cookedCodCount = 0;
    uchar cookedSalmonCount = 0;
    
    for (uchar i = 0; i < 2; i++) {
        uchar weight = nextInt(&seed, 2);
        if (weight < 1) {
            CHECK_COUNT(COOKED_COD)
            uchar value = (uchar) getCount(&seed, 2, 4);
            cookedCodCount += value;
            PUSH_ITEM(COOKED_COD, value);
        } else {
            CHECK_COUNT(COOKED_SALMON)
            uchar value = (uchar) getCount(&seed, 2, 4);
            cookedSalmonCount += value;
            PUSH_ITEM(COOKED_SALMON, value);
        }
    }
    
    if (cookedCodCount != COOKED_COD_COUNT && cookedSalmonCount != COOKED_SALMON_COUNT) {
        return false;
    }
    
    return shuffleChest(seed, loot);
}

bool inline checkSeed(int64_t seed, Generator *g, const int32_t chunk_x, const int32_t chunk_z) {
    if (!canGenerateTreasure(seed, chunk_x, chunk_z)) {
        return false;
    }
    if (isCorrectLoot(seed, chunk_x, chunk_z)) {
        applySeed(g, DIM_OVERWORLD, seed);
        return isViableStructurePos(Treasure, g, chunk_x * 16 + 9, chunk_z * 16 + 9, 0);
    }
    return false;
}


#endif /* seedcracker_hpp */
