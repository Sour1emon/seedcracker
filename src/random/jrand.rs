#[derive(Debug, Copy, Clone)]
pub struct JRand {
    lcg: LCG,
    pub seed: i64,
    next_next_gaussian: f64,
    have_next_next_gaussian: bool,
}

impl JRand {
    const DOUBLE_UNIT: f64 = 1.0 / (1_i64 << 53) as f64;

    pub const fn new(seed: i64) -> Self {
        Self::new_scramble(seed, true)
    }

    pub const fn new_scramble(seed: i64, scramble: bool) -> Self {
        let mut rand = Self {
            lcg: LCG::JAVA,
            seed,
            next_next_gaussian: 0.0,
            have_next_next_gaussian: false,
        };
        rand.set_seed(seed, scramble);
        rand
    }

    fn of_internal_seed(seed: i64) -> Self {
        Self::new_scramble(seed, false)
    }

    fn of_scrambled_seed(seed: i64) -> Self {
        Self::new_scramble(seed, false)
    }

    pub fn next_bool(seed: i64) -> bool {
        (((seed as u64) >> 47) & 1) == 1
    }

    pub fn next_int(seed: i64) -> i32 {
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

    pub fn next_float(seed: i64) -> f32 {
        (seed as u64 >> 24) as f32 / ((1 << 24) as f32)
    }

    pub fn next_long(seed: i64) -> i64 {
        (((seed as u64 >> 16) as i64) << 32) + (LCG::JAVA.next_seed(seed) as u64 >> 16) as i64
    }

    pub fn next_double(seed: i64) -> f64 {
        ((((seed as u64 >> 22) as i64) << 27) + (LCG::JAVA.next_seed(seed) as u64 >> 16) as i64)
            as f64
            + Self::DOUBLE_UNIT
    }

    pub fn next(&mut self, bits: i32) -> i32 {
        self.next_bits(bits) as i32
    }

    pub fn get_next_bool(&mut self) -> bool {
        self.next(1) == 1
    }

    pub fn get_next_int(&mut self) -> i32 {
        self.next(32)
    }

    pub fn get_next_int_bound(&mut self, bound: i32) -> i32 {
        let m = bound - 1;

        debug_assert!(bound >= 0);

        if (bound & -bound) == bound {
            return ((bound as i64 * self.next(31) as i64) >> 31) as i32;
        }

        let mut bits: i32;
        let mut value: i32;

        loop {
            bits = self.next(31);
            value = bits % bound;
            if bits - value + m >= 0 {
                break;
            }
        }

        value
    }

    pub fn get_next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1 << 24) as f32
    }

    pub fn get_next_long(&mut self) -> i64 {
        ((self.next(32) as i64) << 32) + self.next(32) as i64
    }

    pub fn get_next_double(&mut self) -> f64 {
        (((self.next(26) as i64) << 27) + self.next(27) as i64) as f64 * Self::DOUBLE_UNIT
    }

    pub fn next_gaussian(&mut self) -> f64 {
        if self.have_next_next_gaussian {
            self.have_next_next_gaussian = false;
            self.next_next_gaussian
        } else {
            let mut v1: f64;
            let mut v2: f64;
            let mut s: f64;

            loop {
                v1 = 2.0 * self.get_next_double() - 1.0;
                v2 = 2.0 * self.get_next_double() - 1.0;
                s = v1 * v1 + v2 * v2;
                if !(s >= 1.0 || s == 0.0) {
                    break;
                }
            }

            let multiplier = (-2.0 * s.ln() / s).sqrt();
            self.next_next_gaussian = v2 * multiplier;
            self.have_next_next_gaussian = true;
            v1 * multiplier
        }
    }

    pub fn shuffle_clone<T: Clone>(&mut self, vec: &mut [T]) {
        let size = vec.len();
        let mut list = vec![];
        list.extend_from_slice(vec);
        for i in (1..size).rev() {
            list.swap(i - 1, self.get_next_int_bound(i as i32) as usize)
        }
        for (i, t) in list.iter().enumerate() {
            vec[i] = t.clone();
        }
    }

    pub fn shuffle<T: Copy>(&mut self, vec: &mut [T]) {
        let size = vec.len();
        for i in (2..=size).rev() {
            vec.swap(i - 1, self.get_next_int_bound(i as i32) as usize);
        }
    }

    pub fn shuffle_test<T>(&mut self, list: &mut [T]) {
        let size = list.len();
        for i in (2..=size).rev() {
            list.swap(i - 1, self.get_next_int_bound(i as i32) as usize);
        }
    }

    pub const fn set_seed(&mut self, seed: i64, scramble: bool) {
        self.seed = if scramble {
            seed ^ LCG::JAVA.multiplier
        } else {
            seed
        };
    }

    pub fn next_seed(&mut self) -> i64 {
        let next = self.lcg.next_seed(self.seed);
        self.seed = next;
        next
    }

    pub fn next_bits(&mut self, bits: i32) -> i64 {
        self.seed = self.next_seed();

        if self.lcg.is_power_of_two {
            (self.seed as u64 >> (self.lcg.trailing_zeros - bits)) as i64
        } else {
            self.seed / (1 << bits)
        }
    }

    pub fn advance_calls(&mut self, calls: i64) {
        self.advance(self.lcg.combine_steps(calls))
    }

    pub fn advance(&mut self, skip: LCG) {
        self.seed = skip.next_seed(self.seed);
    }
}

#[derive(Debug, Copy, Clone)]
struct LCG {
    multiplier: i64,
    addend: i64,
    modulus: i64,
    is_power_of_two: bool,
    trailing_zeros: i32,
}

const fn is_power_of_two(value: i64) -> bool {
    (value & -value) == value
}

impl LCG {
    pub const CC65_M23: LCG = LCG::new(65793, 4282663, 1 << 23);

    pub const VISUAL_BASIC: LCG = LCG::new(1140671485, 12820163, 1 << 24);

    pub const RTL_UNIFORM: LCG = LCG::new(2147483629, 2147483587, (1 << 31) - 1);
    pub const MINSTD_RAND0_C: LCG = LCG::new(16807, 0, (1 << 31) - 1);
    pub const MINSTD_RAND_C: LCG = LCG::new(48271, 0, (1 << 31) - 1);

    pub const CC65_M31: LCG = LCG::new(16843009, 826366247, 1 << 23);
    pub const RANDU: LCG = LCG::new(65539, 0, 1 << 31);
    pub const GLIB_C: LCG = LCG::new(1103515245, 12345, 1 << 31);

    pub const BORLAND_C: LCG = LCG::new(22695477, 1, 1 << 32);
    pub const PASCAL: LCG = LCG::new(134775813, 1, 1 << 32);
    pub const OPEN_VMS: LCG = LCG::new(69069, 1, 1 << 32);
    pub const NUMERICAL_RECIPES: LCG = LCG::new(1664525, 1013904223, 1 << 32);
    pub const MS_VISUAL_C: LCG = LCG::new(214013, 2531011, 1 << 32);

    pub const JAVA: LCG = LCG::new(25214903917, 11, 1 << 48);

    pub const JAVA_UNIQUIFIER_OLD: LCG = LCG::new(181783497276652981, 0, 0);
    pub const JAVA_UNIQUIFIER_NEW: LCG = LCG::new(1181783497276652981, 0, 0);
    pub const MMIX: LCG = LCG::new(364136223846793005, 1442695040888963407, 0);
    pub const NEWLIB_C: LCG = LCG::new(6364136223846793005, 1, 0);
    pub const XKCD: LCG = LCG::new(0, 4, 0);

    pub const fn new(multiplier: i64, addend: i64, modulus: i64) -> LCG {
        let is_power_of_two = is_power_of_two(modulus);
        LCG {
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

    pub fn combine(lcgs: Vec<LCG>) -> LCG {
        let mut lcg = lcgs[0];

        for x in lcgs {
            lcg = lcg.combine_two(x);
        }

        lcg
    }

    pub fn mod_(&self, n: i64) -> i64 {
        if self.is_power_of_two {
            return n & (self.modulus - 1);
        } else if n <= 1 << 32 {
            return ((n as u64) % self.modulus as u64) as i64;
        }

        panic!("Unsupported operation")
    }

    pub fn combine_steps(&self, steps: i64) -> LCG {
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

        LCG::new(multiplier, addend, self.modulus)
    }

    pub fn combine_two(&self, lcg: LCG) -> LCG {
        if self.modulus != lcg.modulus {
            panic!("Unsupported operation")
        }

        LCG::new(
            self.multiplier * lcg.multiplier,
            lcg.multiplier * self.addend + lcg.addend,
            self.modulus,
        )
    }

    pub fn next_seed(&self, seed: i64) -> i64 {
        self.mod_(seed * self.multiplier + self.addend)
    }

    pub fn invert(&self) -> LCG {
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

mod tests {
    use crate::random::jrand::JRand;

    #[test]
    fn test_next_int() {
        let mut rand = JRand::new(4506419895);
        assert_eq!(rand.get_next_int(), -4);
    }

    #[test]
    fn test_next_int_bound() {
        let mut rand = JRand::new(4506419895);
        assert_eq!(rand.get_next_int_bound(100), 10)
    }

    #[test]
    fn test_next_bool() {
        let mut rand = JRand::new(4506419895);
        assert!(rand.get_next_bool());
    }

    #[test]
    fn test_next_seed() {
        let mut rand = JRand::new(4506419895);
        assert_eq!(rand.next_seed(), 281474976500957)
    }

    #[test]
    fn test_next_float() {
        let mut rand = JRand::new(4506419895);
        assert!((rand.get_next_float() - 0.99999994).abs() < f32::EPSILON)
    }

    #[test]
    fn test_next_double() {
        let mut rand = JRand::new(4506419895);
        assert!((rand.get_next_double() - 0.9999999883008573).abs() < f64::EPSILON);
    }

    #[test]
    fn test_shuffle() {
        let mut rand = JRand::new(4506419895);
        let mut vec = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        ];
        rand.shuffle(&mut vec);
        assert_eq!(
            vec,
            [7, 11, 5, 15, 10, 17, 8, 18, 4, 14, 12, 2, 1, 3, 13, 6, 16, 9]
        )
    }

    #[test]
    fn test_next_long() {
        let mut rand = JRand::new(4506419895);
        assert_eq!(rand.get_next_long(), -16256950163)
    }

    #[test]
    fn test_next_gaussian() {
        let mut rand = JRand::new(4506419895);
        assert!((rand.next_gaussian() - (-0.1851115908371161)).abs() < f64::EPSILON);
    }
}
