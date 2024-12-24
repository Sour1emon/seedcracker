//
//  rng.h
//  seedcracker
//
//  Created by Isaac Bess on 12/24/24.
//

#ifndef rng_h
#define rng_h

#include <stdint.h>

// https://en.wikipedia.org/wiki/Linear_congruential_generator

// Multiplier
inline const uint64_t JAVA_LCG_MULTIPLIER = 0x5deece66d;

// Addend
inline const uint64_t JAVA_LCG_ADDEND = 0xb;

// Modulus (probably should be (1 << 48))
inline const uint64_t MASK_48 = (1ULL << 48) - 1;



inline const uint64_t BURIED_TREASURE_SALT = 10387320;
inline const uint64_t BURIED_TREASURE_DECORATOR_SALT = 30001;

inline const int32_t CHUNK_X = -28;
inline const int32_t CHUNK_Z = -73;

static inline void setSeed(uint64_t *seed, uint64_t value)
{
    *seed = (value ^ JAVA_LCG_MULTIPLIER) & MASK_48;
}

static inline int next(uint64_t *seed, const int bits)
{
    *seed = (*seed * JAVA_LCG_MULTIPLIER + JAVA_LCG_ADDEND) & MASK_48;
    return (int) ((int64_t)*seed >> (48 - bits));
}

static inline int nextInt(uint64_t *seed, const int n)
{
    int bits, val;
    const int m = n - 1;
    
    if ((m & n) == 0) {
        uint64_t x = n * (uint64_t)next(seed, 31);
        return (int) ((int64_t) x >> 31);
    }
    
    do {
        bits = next(seed, 31);
        val = bits % n;
    }
    while ((int32_t)((uint32_t)bits - val + m) < 0);
    return val;
}

static inline uint64_t nextLong(uint64_t *seed)
{
    return ((uint64_t) next(seed, 32) << 32) + next(seed, 32);
}

static inline float nextFloat(uint64_t *seed)
{
    return next(seed, 24) / (float) (1 << 24);
}

// Minecraft

bool inline canGenerateTreasure(uint64_t seed) {
    seed = CHUNK_X * 341873128712ULL + CHUNK_Z * 132897987541ULL + seed + BURIED_TREASURE_SALT;
    setSeed(&seed, seed);
    return nextFloat(&seed) < 0.01;
}

uint64_t inline getPopulationSeed(uint64_t worldSeed, int32_t x, int32_t z) {
    uint64_t seed;
    setSeed(&seed, worldSeed);
    int64_t a = nextLong(&seed) | 1;
    int64_t b = nextLong(&seed) | 1;
    
    seed = (((int64_t) x * a) + ((int64_t) z * b)) ^ worldSeed;
    return seed & MASK_48;
}

uint64_t inline getDecoratorSeed(uint64_t worldSeed, int32_t x, int32_t z, const int32_t salt) {
    uint64_t seed = getPopulationSeed(worldSeed, x, z);
    setSeed(&seed, seed + (uint64_t) salt);
    return seed & MASK_48;
}

#endif /* rng_h */
