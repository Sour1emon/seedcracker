//
//  rng.h
//  seedcracker
//
//  Created by Isaac Bess on 12/24/24.
//

#ifndef rng_h
#define rng_h

#include <stdint.h>
#include <limits>

typedef unsigned char uchar;

// https://en.wikipedia.org/wiki/Linear_congruential_generator

// Multiplier
inline const int64_t JAVA_LCG_MULTIPLIER = 0x5deece66d;

// Addend
inline const int64_t JAVA_LCG_ADDEND = 0xb;

// Modulus (probably should be (1 << 48))
inline const int64_t MASK_48 = (1ULL << 48) - 1;



inline const int64_t BURIED_TREASURE_SALT = 10387320;
inline const int64_t BURIED_TREASURE_DECORATOR_SALT = 30001;

inline const int32_t CHUNK_X = -28;
inline const int32_t CHUNK_Z = -73;

static void setSeed(int64_t *seed, int64_t value)
{
    *seed = (value ^ JAVA_LCG_MULTIPLIER) & MASK_48;
}

static int next(int64_t *seed, const int bits)
{
    *seed = (*seed * JAVA_LCG_MULTIPLIER + JAVA_LCG_ADDEND) & MASK_48;
    return (int32_t) (*seed >> (48 - bits));
}

static bool nextBool(int64_t *seed) {
    return (bool) next(seed, 1);
}

static int32_t nextInt(int64_t *seed, int32_t n)
{
    
    if (n < 0) {
        // Find better way
        return std::numeric_limits<int32_t>::min();
    }
    
    int32_t m = n - 1;
    
    if ((m & n) == 0){
        int64_t x = (int64_t) n * (int64_t) next(seed, 31);
        return (int32_t) (x >> 31);
    }
    
    int32_t bits;
    int32_t value;
    
    do {
        bits = next(seed, 31);
        value = bits % n;
        if (bits - value + m >= 0) {
            break;
        }
    } while (1);
    
    return value;
}

static int64_t nextLong(int64_t *seed)
{
    return ((int64_t) next(seed, 32) << 32) + next(seed, 32);
}

static float nextFloat(int64_t *seed)
{
    return next(seed, 24) / (float) (1 << 24);
}

uint64_t getPopulationSeed(int64_t worldSeed, int32_t x, int32_t z) {
    int64_t seed;
    setSeed(&seed, worldSeed);
    int64_t a = nextLong(&seed) | 1;
    int64_t b = nextLong(&seed) | 1;
    
    seed = (((int64_t) x * a) + ((int64_t) z * b)) ^ worldSeed;
    return seed & MASK_48;
}

uint64_t getDecoratorSeed(int64_t worldSeed, int32_t x, int32_t z, int32_t salt) {
    int64_t seed = getPopulationSeed(worldSeed, x, z);
    setSeed(&seed, seed + (uint64_t) salt);
    return seed & MASK_48;
}

//
// HELPER FUNCTIONS
//

bool canGenerateTreasure(int64_t seed, const int32_t chunk_x, const int32_t chunk_z) {
    int64_t s = ((int64_t) chunk_x * 341873128712 + (int64_t) chunk_z * 132897987541 + seed + BURIED_TREASURE_SALT);
    setSeed(&s, s);
    return nextFloat(&s) < 0.01;
}

int getCount(int64_t *seed, int32_t min, int32_t max) {
    if (min >= max) {
        return min;
    } else {
        return nextInt(seed, max - min + 1) + min;
    }
}

#endif /* rng_h */
