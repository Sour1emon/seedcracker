//
//  rng.h
//  seedcracker
//
//  Created by Isaac Bess on 12/24/24.
//

#ifndef rng_h
#define rng_h

// Multiplier
constant uint64_t JAVA_LCG_MULTIPLIER = 0x5deece66d;

// Addend
constant uint64_t JAVA_LCG_ADDEND = 0xb;

// Modulus
constant int64_t MASK_48 = (uint64_t)((1ULL << 48) - 1);

static inline void setSeed(thread uint64_t *seed, uint64_t value) {
    *seed = (value ^ JAVA_LCG_MULTIPLIER) & MASK_48;
}

static inline int next(thread uint64_t *seed, const int bits)
{
    *seed = (*seed * JAVA_LCG_MULTIPLIER + JAVA_LCG_ADDEND) & MASK_48;
    return (int) ((int64_t) *seed >> (48 - bits));
}

static inline bool nextBool(thread uint64_t *seed, const int bits) {
    return next(seed, 1) == 1;
}

static inline int nextInt(thread uint64_t *seed, const int n) {
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

static inline uint64_t nextLong(thread uint64_t *seed)
{
    return ((uint64_t) next(seed, 32) << 32) + next(seed, 32);
}

static inline float nextFloat(thread uint64_t *seed)
{
    return next(seed, 24) / (float) (1 << 24);
}

static inline void skipNextN(thread uint64_t *seed, uint64_t n) {
    uint64_t m = 1;
    uint64_t a = 0;
    uint64_t im = JAVA_LCG_MULTIPLIER;
    uint64_t ia = JAVA_LCG_ADDEND;
    uint64_t k;
    
    for (k = n; k; k >>= 1) {
        if (k & 1) {
            m *= im;
            a = im * a + ia;
        }
        ia = (im + 1) * ia;
        im *= im;
    }
    
    *seed = *seed * m + a;
    *seed &= MASK_48;
}

#endif /* rng_h */
