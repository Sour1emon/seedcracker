use crate::random::mth::MASK_48;

#[derive(Debug, Copy, Clone)]
pub struct JRand {
    lcg: LCG,
    pub seed: i64,
}

impl JRand {
    const DOUBLE_UNIT: f64 = 1.0 / (1_i64 << 53) as f64;

    #[inline(always)]
    pub const fn new(seed: i64) -> Self {
        Self::new_scramble(seed, true)
    }

    #[inline(always)]
    pub const fn new_scramble(seed: i64, scramble: bool) -> Self {
        let mut rand = Self {
            lcg: LCG::JAVA,
            seed,
        };
        rand.set_seed(seed, scramble);
        rand
    }

    const fn of_internal_seed(seed: i64) -> Self {
        Self::new_scramble(seed, false)
    }

    const fn of_scrambled_seed(seed: i64) -> Self {
        Self::new_scramble(seed, false)
    }

    pub const fn next_bool(seed: i64) -> bool {
        (((seed as u64) >> 47) & 1) == 1
    }

    pub const fn next_int(seed: i64) -> i32 {
        (seed as u64 >> 16) as i32
    }

    pub fn next_int_bound(mut seed: i64, bound: i32) -> i32 {
        if bound <= 0 {
            panic!("bound must be positive");
        }

        if (bound & -bound) == bound {
            return ((bound as i64 * seed) >> 31) as i32;
        }

        let mut bits;
        let mut value;

        loop {
            bits = (seed as u64 >> 17) as i32;
            value = bits % bound;
            seed = LCG::JAVA.next_seed(seed);
            if bits - value + (bound - 1) >= 0 {
                break;
            }
        }

        value
    }

    pub const fn next_float(seed: i64) -> f32 {
        (seed as u64 >> 24) as f32 / ((1 << 24) as f32)
    }

    pub const fn next_long(seed: i64) -> i64 {
        (((seed as u64 >> 16) as i64) << 32) + (LCG::JAVA.next_seed(seed) as u64 >> 16) as i64
    }

    pub fn next_double(seed: i64) -> f64 {
        (((((seed as u64 >> 22) as i64) << 27) + (LCG::JAVA.next_seed(seed) as u64 >> 16) as i64)
            as f64)
            * Self::DOUBLE_UNIT
    }

    #[inline(always)]
    pub const fn next(&mut self, bits: i32) -> i32 {
        self.seed = (self.seed * 0x5deece66d + 0xb) & MASK_48;
        (self.seed >> (48 - bits)) as i32
    }

    #[inline(always)]
    pub const fn get_next(seed: i64, bits: i32) -> i32 {
        (((seed * 0x5deece66d + 0xb) & MASK_48) >> (48 - bits)) as i32
    }

    #[inline(always)]
    pub const fn get_next_bool(&mut self) -> bool {
        self.next(1) == 1
    }

    #[inline(always)]
    pub const fn get_next_int_bound(&mut self, n: i32) -> i32 {
        let m = n - 1;

        debug_assert!(n >= 0);

        if (m & n) == 0 {
            let x = n as i64 * self.next(31) as i64;
            return (x >> 31) as i32;
        }

        let mut bits: i32;
        let mut value: i32;

        loop {
            bits = self.next(31);
            value = bits % n;
            if bits - value + m >= 0 {
                break;
            }
        }

        value
    }

    #[inline(always)]
    pub const fn get_next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1 << 24) as f32
    }

    #[inline(always)]
    pub const fn get_next_long(&mut self) -> i64 {
        ((self.next(32) as i64) << 32) + self.next(32) as i64
    }

    #[inline(always)]
    pub const fn get_next_double(&mut self) -> f64 {
        (((self.next(26) as i64) << 27) + self.next(27) as i64) as f64 * Self::DOUBLE_UNIT
    }

    #[inline(always)]
    pub fn shuffle<T>(&mut self, vec: &mut [T]) {
        let size = vec.len();
        for i in (2..=size).rev() {
            vec.swap(i - 1, self.get_next_int_bound(i as i32) as usize);
        }
    }

    #[inline(always)]
    pub const fn set_seed(&mut self, seed: i64, scramble: bool) {
        self.seed = if scramble {
            seed ^ LCG::JAVA.multiplier
        } else {
            seed
        };
    }

    #[inline(always)]
    pub fn next_seed(&mut self) -> i64 {
        let next = self.lcg.next_seed(self.seed);
        self.seed = next;
        next
    }

    pub fn advance_calls(&mut self, calls: i64) {
        self.advance(self.lcg.combine_steps(calls))
    }

    fn advance(&mut self, skip: LCG) {
        self.seed = skip.next_seed(self.seed);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LCG {
    multiplier: i64,
    addend: i64,
    modulus: i64,
    is_power_of_two: bool,
    trailing_zeros: i32,
}

const fn is_power_of_two(value: i64) -> bool {
    (-value & value) == value
}

impl LCG {
    pub const CC65_M23: Self = Self::new(65793, 4282663, 1 << 23);

    pub const VISUAL_BASIC: Self = Self::new(1140671485, 12820163, 1 << 24);

    pub const RTL_UNIFORM: Self = Self::new(2147483629, 2147483587, (1 << 31) - 1);
    pub const MINSTD_RAND0_C: Self = Self::new(16807, 0, (1 << 31) - 1);
    pub const MINSTD_RAND_C: Self = Self::new(48271, 0, (1 << 31) - 1);

    pub const CC65_M31: Self = Self::new(16843009, 826366247, 1 << 23);
    pub const RANDU: Self = Self::new(65539, 0, 1 << 31);
    pub const GLIB_C: Self = Self::new(1103515245, 12345, 1 << 31);

    pub const BORLAND_C: Self = Self::new(22695477, 1, 1 << 32);
    pub const PASCAL: Self = Self::new(134775813, 1, 1 << 32);
    pub const OPEN_VMS: Self = Self::new(69069, 1, 1 << 32);
    pub const NUMERICAL_RECIPES: Self = Self::new(1664525, 1013904223, 1 << 32);
    pub const MS_VISUAL_C: Self = Self::new(214013, 2531011, 1 << 32);

    pub const JAVA: Self = Self::new(25214903917, 11, 1 << 48);

    pub const JAVA_UNIQUIFIER_OLD: Self = Self::new(181783497276652981, 0, 0);
    pub const JAVA_UNIQUIFIER_NEW: Self = Self::new(1181783497276652981, 0, 0);
    pub const MMIX: Self = Self::new(364136223846793005, 1442695040888963407, 0);
    pub const NEWLIB_C: Self = Self::new(6364136223846793005, 1, 0);
    pub const XKCD: Self = Self::new(0, 4, 0);

    pub const fn new(multiplier: i64, addend: i64, modulus: i64) -> Self {
        let is_power_of_two = is_power_of_two(modulus);
        Self {
            multiplier,
            addend,
            modulus,
            is_power_of_two,
            trailing_zeros: if is_power_of_two {
                modulus.trailing_zeros() as i32
            } else {
                -1
            },
        }
    }

    pub fn combine(lcgs: Vec<Self>) -> Self {
        let mut lcg = lcgs[0];

        for x in lcgs {
            lcg = lcg.combine_two(x);
        }

        lcg
    }

    #[inline(always)]
    pub const fn mod_(&self, n: i64) -> i64 {
        if self.is_power_of_two {
            return n & (self.modulus - 1);
        } else if n <= 1 << 32 {
            return ((n as u64) % self.modulus as u64) as i64;
        }

        panic!("Unsupported operation")
    }

    pub const fn combine_steps(&self, steps: i64) -> Self {
        let mut multiplier: i64 = 1;
        let mut addend: i64 = 0;

        let mut intermediate_multiplier = self.multiplier;
        let mut intermediate_addend = self.addend;

        let mut k = steps;
        while k != 0 {
            if (k & 1) != 0 {
                multiplier *= intermediate_multiplier;
                addend = intermediate_multiplier * addend + intermediate_addend;
            }

            intermediate_addend *= intermediate_multiplier + 1;
            intermediate_multiplier *= intermediate_multiplier;
            k = (k as u64 >> 1) as i64;
        }

        multiplier = self.mod_(multiplier);
        addend = self.mod_(addend);

        Self::new(multiplier, addend, self.modulus)
    }

    pub fn combine_two(&self, lcg: Self) -> Self {
        if self.modulus != lcg.modulus {
            panic!("Unsupported operation")
        }

        Self::new(
            self.multiplier * lcg.multiplier,
            lcg.multiplier * self.addend + lcg.addend,
            self.modulus,
        )
    }

    #[inline(always)]
    pub const fn next_seed(&self, mut seed: i64) -> i64 {
        let mut m = 1;
        let mut a = 0;
        let mut im = Self::JAVA.multiplier;
        let mut ia = Self::JAVA.addend;
        let mut k = 1;

        while k != 0 {
            if (k & 1) != 0 {
                m *= im;
                a = im * a + ia;
            }
            ia *= im + 1;
            im *= im;
            k >>= 1;
        }

        seed = seed * m + a;
        seed & MASK_48
    }

    #[inline(always)]
    pub const fn invert(&self) -> Self {
        self.combine_steps(-1)
    }

    // pub fn distance(&self, seed1: i64, seed2: i64) -> i64 {
    //     if DiscreteLog::supports(*self) {
    //         let a_from_zero = DiscreteLog::distance_from_zero(*self, seed1);
    //         let b_from_zero = DiscreteLog::distance_from_zero(*self, seed2);
    //         mask_signed(b_from_zero - a_from_zero, self.trailing_zeros)
    //     } else {
    //         panic!("DiscreteLog is not supported by this LCG")
    //     }
    // }
}

//struct DiscreteLog;

// impl DiscreteLog {
//     fn supports(lcg: LCG) -> bool {
//         if lcg.is_power_of_two || lcg.trailing_zeros > 61 {
//             return false;
//         };
//         lcg.multiplier % 2 != 0 && lcg.addend % 2 != 0
//     }
//
//     fn distance_from_zero(lcg: LCG, seed: i64) -> i64 {
//         let exp = lcg.trailing_zeros;
//
//         let a = lcg.multiplier;
//         let b = mask(
//             seed * (lcg.multiplier - 1) * mod_inverse(lcg.addend, exp) + 1,
//             exp + 2,
//         );
//         let a_bar = Self::theta(a, exp);
//         let b_bar = Self::theta(b, exp);
//         b_bar * mask(mod_inverse(a_bar, exp), exp)
//     }
//
//     fn theta(mut number: i64, exp: i32) -> i64 {
//         if number % 4 == 3 {
//             number = (1_i64 << (exp + 2)) - number;
//         }
//
//         let mut x_hat = Integer::from(number);
//         x_hat = x_hat
//             .pow_mod(
//                 &Integer::ONE.shl(exp + 1).complete(),
//                 &Integer::ONE.shl(2 * exp + 3).complete(),
//             )
//             .unwrap();
//         x_hat -= Integer::ONE;
//         x_hat /= Integer::ONE.shl(exp + 3).complete();
//         x_hat %= Integer::ONE.shl(exp).complete();
//         x_hat.to_i64().unwrap()
//     }
// }
