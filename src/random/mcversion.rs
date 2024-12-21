use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCVersion {
    name: &'static str,
    release: i32,
    sub_version: i32,
}

impl MCVersion {
    const fn new(name: &'static str, release: i32, sub_version: i32) -> Self {
        Self {
            name,
            release,
            sub_version,
        }
    }

    pub fn from_string(name: &str) -> Option<&'static MCVersion> {
        VERSIONS_MAP.get(name).cloned()
    }

    pub fn get_release(&self) -> i32 {
        self.release
    }

    pub fn get_sub_version(&self) -> i32 {
        self.sub_version
    }

    pub fn latest() -> &'static MCVersion {
        &VERSIONS[0]
    }

    pub fn oldest() -> &'static MCVersion {
        VERSIONS.last().unwrap()
    }

    pub fn is_newer_than(&self, other: &MCVersion) -> bool {
        self.cmp(other) == Ordering::Greater
    }

    pub fn is_newer_or_equal_to(&self, other: &MCVersion) -> bool {
        self.cmp(other) != Ordering::Less
    }

    pub fn is_older_than(&self, other: &MCVersion) -> bool {
        self.cmp(other) == Ordering::Less
    }

    pub fn is_older_or_equal_to(&self, other: &MCVersion) -> bool {
        self.cmp(other) != Ordering::Greater
    }

    pub fn is_equal_to(&self, other: &MCVersion) -> bool {
        self.cmp(other) == Ordering::Equal
    }

    pub fn is_between(&self, a: &MCVersion, b: &MCVersion) -> bool {
        self.cmp(a) != Ordering::Less && self.cmp(b) != Ordering::Greater
    }

    pub fn is_between_exclusive(&self, a: &MCVersion, b: &MCVersion) -> bool {
        self.cmp(a) == Ordering::Greater && self.cmp(b) == Ordering::Less
    }

    pub fn newer(&self) -> Option<&'static MCVersion> {
        VERSIONS
            .iter()
            .position(|v| v.name == self.name)
            .and_then(|i| if i > 0 { Some(&VERSIONS[i - 1]) } else { None })
    }

    pub fn older(&self) -> Option<&'static MCVersion> {
        VERSIONS
            .iter()
            .position(|v| v.name == self.name)
            .and_then(|i| VERSIONS.get(i + 1))
    }

    pub fn is_release(&self) -> bool {
        MCVersion::is_release_static(self)
    }

    fn is_release_static(version: &MCVersion) -> bool {
        version.is_older_or_equal_to(&V1_0)
    }

    pub fn is_alpha(&self) -> bool {
        MCVersion::is_alpha_static(self)
    }

    fn is_alpha_static(version: &MCVersion) -> bool {
        version.is_between(&VA1_0_4, &VA1_2_6)
    }

    pub fn is_beta(&self) -> bool {
        MCVersion::is_beta_static(self)
    }

    fn is_beta_static(version: &MCVersion) -> bool {
        version.is_between(&VB1_0, &VB1_8_1)
    }
}

impl PartialOrd for MCVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MCVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self
            .release
            .cmp(&other.release)
            .then_with(|| other.sub_version.cmp(&self.sub_version))
    }
}

impl std::fmt::Display for MCVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Define all versions as static constants
pub static V1_19_2: MCVersion = MCVersion::new("1.19.2", 19, 2); // August 5, 2022
pub static V1_19_1: MCVersion = MCVersion::new("1.19.1", 19, 1); // July 27, 2022
pub static V1_19: MCVersion = MCVersion::new("1.19", 19, 0); // June 7, 2022

pub static V1_18_2: MCVersion = MCVersion::new("1.18.2", 18, 2); // February 28, 2022
pub static V1_18_1: MCVersion = MCVersion::new("1.18.1", 18, 1); // December 10, 2021
pub static V1_18: MCVersion = MCVersion::new("1.18", 18, 0); // November 30, 2021

pub static V1_17_1: MCVersion = MCVersion::new("1.17.1", 17, 1); // July 6, 2021
pub static V1_17: MCVersion = MCVersion::new("1.17", 17, 0); // June 8, 2021

pub static V1_16_5: MCVersion = MCVersion::new("1.16.5", 16, 5); // January 14, 2021
pub static V1_16_4: MCVersion = MCVersion::new("1.16.4", 16, 4); // October 29, 2020
pub static V1_16_3: MCVersion = MCVersion::new("1.16.3", 16, 3); // September 10, 2020
pub static V1_16_2: MCVersion = MCVersion::new("1.16.2", 16, 2); // August 11, 2020
pub static V1_16_1: MCVersion = MCVersion::new("1.16.1", 16, 1); // June 24, 2020
pub static V1_16: MCVersion = MCVersion::new("1.16", 16, 0); // June 23, 2020

pub static V1_15_2: MCVersion = MCVersion::new("1.15.2", 15, 2); // January 17, 2020
pub static V1_15_1: MCVersion = MCVersion::new("1.15.1", 15, 1); // December 16, 2019
pub static V1_15: MCVersion = MCVersion::new("1.15", 15, 0); // December 9, 2019

pub static V1_14_4: MCVersion = MCVersion::new("1.14.4", 14, 4); // July 19, 2019
pub static V1_14_3: MCVersion = MCVersion::new("1.14.3", 14, 3); // June 24, 2019
pub static V1_14_2: MCVersion = MCVersion::new("1.14.2", 14, 2); // May 27, 2019
pub static V1_14_1: MCVersion = MCVersion::new("1.14.1", 14, 1); // May 13, 2019
pub static V1_14: MCVersion = MCVersion::new("1.14", 14, 0); // April 23, 2019

pub static V1_13_2: MCVersion = MCVersion::new("1.13.2", 13, 2); // October 22, 2018
pub static V1_13_1: MCVersion = MCVersion::new("1.13.1", 13, 1); // August 22, 2018
pub static V1_13: MCVersion = MCVersion::new("1.13", 13, 0); // July 18, 2018

pub static V1_12_2: MCVersion = MCVersion::new("1.12.2", 12, 2); // September 18, 2017
pub static V1_12_1: MCVersion = MCVersion::new("1.12.1", 12, 1); // August 3, 2017
pub static V1_12: MCVersion = MCVersion::new("1.12", 12, 0); // June 2, 2017

pub static V1_11_2: MCVersion = MCVersion::new("1.11.2", 11, 2); // December 21, 2016
pub static V1_11_1: MCVersion = MCVersion::new("1.11.1", 11, 1); // December 20, 2016
pub static V1_11: MCVersion = MCVersion::new("1.11", 11, 0); // November 14, 2016

pub static V1_10_2: MCVersion = MCVersion::new("1.10.2", 10, 2); // June 23, 2016
pub static V1_10_1: MCVersion = MCVersion::new("1.10.1", 10, 1); // June 22, 2016
pub static V1_10: MCVersion = MCVersion::new("1.10", 10, 0); // June 8, 2016

pub static V1_9_4: MCVersion = MCVersion::new("1.9.4", 9, 4); // May 10, 2016
pub static V1_9_3: MCVersion = MCVersion::new("1.9.3", 9, 3); // May 10, 2016
pub static V1_9_2: MCVersion = MCVersion::new("1.9.2", 9, 2); // March 30, 2016
pub static V1_9_1: MCVersion = MCVersion::new("1.9.1", 9, 1); // March 30, 2016
pub static V1_9: MCVersion = MCVersion::new("1.9", 9, 0); // February 29, 2016

pub static V1_8_9: MCVersion = MCVersion::new("1.8.9", 8, 9); // December 3, 2015
pub static V1_8_8: MCVersion = MCVersion::new("1.8.8", 8, 8); // July 27, 2015
pub static V1_8_7: MCVersion = MCVersion::new("1.8.7", 8, 7); // June 5, 2015
pub static V1_8_6: MCVersion = MCVersion::new("1.8.6", 8, 6); // May 25, 2015
pub static V1_8_5: MCVersion = MCVersion::new("1.8.5", 8, 5); // May 22, 2015
pub static V1_8_4: MCVersion = MCVersion::new("1.8.4", 8, 4); // April 17, 2015
pub static V1_8_3: MCVersion = MCVersion::new("1.8.3", 8, 3); // February 20, 2015
pub static V1_8_2: MCVersion = MCVersion::new("1.8.2", 8, 2); // February 19, 2015
pub static V1_8_1: MCVersion = MCVersion::new("1.8.1", 8, 1); // November 24, 2014
pub static V1_8: MCVersion = MCVersion::new("1.8", 8, 0); // September 2, 2014

pub static V1_7_10: MCVersion = MCVersion::new("1.7.10", 7, 10); // May 14, 2014
pub static V1_7_9: MCVersion = MCVersion::new("1.7.9", 7, 9); // April 14, 2014
pub static V1_7_8: MCVersion = MCVersion::new("1.7.8", 7, 8); // April 9, 2014
pub static V1_7_7: MCVersion = MCVersion::new("1.7.7", 7, 7); // April 9, 2014
pub static V1_7_6: MCVersion = MCVersion::new("1.7.6", 7, 6); // April 9, 2014
pub static V1_7_5: MCVersion = MCVersion::new("1.7.5", 7, 5); // February 26, 2014
pub static V1_7_4: MCVersion = MCVersion::new("1.7.4", 7, 4); // December 9, 2013
pub static V1_7_3: MCVersion = MCVersion::new("1.7.3", 7, 3); // December 6, 2013
pub static V1_7_2: MCVersion = MCVersion::new("1.7.2", 7, 2); // October 25, 2013

pub static V1_6_4: MCVersion = MCVersion::new("1.6.4", 6, 4); // September 19, 2013
pub static V1_6_2: MCVersion = MCVersion::new("1.6.2", 6, 2); // July 5, 2013
pub static V1_6_1: MCVersion = MCVersion::new("1.6.1", 6, 1); // June 28, 2013

pub static V1_5_2: MCVersion = MCVersion::new("1.5.2", 5, 2); // April 25, 2013
pub static V1_5_1: MCVersion = MCVersion::new("1.5.1", 5, 1); // March 20, 2013

pub static V1_4_7: MCVersion = MCVersion::new("1.4.7", 4, 7); // December 27, 2012
pub static V1_4_6: MCVersion = MCVersion::new("1.4.6", 4, 6); // December 19, 2012
pub static V1_4_5: MCVersion = MCVersion::new("1.4.5", 4, 5); // December 19, 2012
pub static V1_4_4: MCVersion = MCVersion::new("1.4.4", 4, 4); // December 13, 2012
pub static V1_4_2: MCVersion = MCVersion::new("1.4.2", 4, 2); // November 24, 2012

pub static V1_3_2: MCVersion = MCVersion::new("1.3.2", 3, 2); // August 15, 2012
pub static V1_3_1: MCVersion = MCVersion::new("1.3.1", 3, 1); // July 31, 2012

pub static V1_2_5: MCVersion = MCVersion::new("1.2.5", 2, 5); // March 29, 2012
pub static V1_2_4: MCVersion = MCVersion::new("1.2.4", 2, 4); // March 21, 2012
pub static V1_2_3: MCVersion = MCVersion::new("1.2.3", 2, 3); // March 1, 2012
pub static V1_2_2: MCVersion = MCVersion::new("1.2.2", 2, 2); // February 29, 2012
pub static V1_2_1: MCVersion = MCVersion::new("1.2.1", 2, 1); // February 29, 2012

pub static V1_1: MCVersion = MCVersion::new("1.1", 1, 0); // January 11, 2012

pub static V1_0: MCVersion = MCVersion::new("1.0", 0, 0); // November 17, 2011

pub static VB1_8_1: MCVersion = MCVersion::new("b1.8.1", 8, 1); // September 18, 2011
pub static VB1_8: MCVersion = MCVersion::new("b1.8", 8, 0); // September 14, 2011

pub static VB1_7_3: MCVersion = MCVersion::new("b1.7.3", 7, 3); // July 7, 2011
pub static VB1_7_2: MCVersion = MCVersion::new("b1.7.2", 7, 2); // June 30, 2011
pub static VB1_7: MCVersion = MCVersion::new("b1.7", 7, 0); // June 29, 2011

pub static VB1_6_6: MCVersion = MCVersion::new("b1.6.6", 6, 6); // May 30, 2011
pub static VB1_6_5: MCVersion = MCVersion::new("b1.6.5", 6, 5); // May 27, 2011
pub static VB1_6_4: MCVersion = MCVersion::new("b1.6.4", 6, 4); // May 25, 2011
pub static VB1_6_3: MCVersion = MCVersion::new("b1.6.3", 6, 3); // May 25, 2011
pub static VB1_6_2: MCVersion = MCVersion::new("b1.6.2", 6, 2); // May 25, 2011
pub static VB1_6_1: MCVersion = MCVersion::new("b1.6.1", 6, 1); // May 25, 2011
pub static VB1_6: MCVersion = MCVersion::new("b1.6", 6, 0); // May 25, 2011

pub static VB1_5_01: MCVersion = MCVersion::new("b1.5_01", 501, 0); // April 19, 2011

pub static VB1_5: MCVersion = MCVersion::new("b1.5", 5, 0); // April 18, 2011

pub static VB1_4_01: MCVersion = MCVersion::new("b1.4_01", 401, 0); // April 4, 2011

pub static VB1_4: MCVersion = MCVersion::new("b1.4", 4, 0); // March 30, 2011

pub static VB1_3_01: MCVersion = MCVersion::new("b1.3_01", 301, 0); // February 22, 2011

pub static VB1_3B: MCVersion = MCVersion::new("b1.3b", 3, 0); // February 21, 2011

pub static VB1_2_02: MCVersion = MCVersion::new("b1.2_02", 202, 0); // January 20, 2011

pub static VB1_2_01: MCVersion = MCVersion::new("b1.2_01", 201, 0); // January 13, 2011

pub static VB1_2: MCVersion = MCVersion::new("b1.2", 2, 0); // January 12, 2011

pub static VB1_1_02: MCVersion = MCVersion::new("b1.1_02", 102, 0); // December 21, 2010

pub static VB1_1_01: MCVersion = MCVersion::new("b1.1_01", 101, 0); // December 21, 2010

pub static VB1_0_2: MCVersion = MCVersion::new("b1.0.2", 0, 2); // December 20, 2010

pub static VB1_0_01: MCVersion = MCVersion::new("b1.0_01", 1, 0); // December 19, 2010

pub static VB1_0: MCVersion = MCVersion::new("b1.0", 0, 0); // December 19, 2010

pub static VA1_2_6: MCVersion = MCVersion::new("a1.2.6", 2, 6); // December 2, 2010
pub static VA1_2_5: MCVersion = MCVersion::new("a1.2.5", 2, 5); // November 30, 2010
pub static VA1_2_4_01: MCVersion = MCVersion::new("a1.2.4_01", 2, 401); // November 29, 2010
pub static VA1_2_3_04: MCVersion = MCVersion::new("a1.2.3_04", 2, 304); // November 25, 2010
pub static VA1_2_3_02: MCVersion = MCVersion::new("a1.2.3_02", 2, 302); // November 24, 2010
pub static VA1_2_3_01: MCVersion = MCVersion::new("a1.2.3_01", 2, 301); // November 23, 2010
pub static VA1_2_3: MCVersion = MCVersion::new("a1.2.3", 2, 3); // November 23, 2010
pub static VA1_2_2B: MCVersion = MCVersion::new("a1.2.2b", 2, 2); // November 9, 2010
pub static VA1_2_2A: MCVersion = MCVersion::new("a1.2.2a", 2, 2); // November 9, 2010
pub static VA1_2_1_01: MCVersion = MCVersion::new("a1.2.1_01", 2, 101); // November 4, 2010
pub static VA1_2_1: MCVersion = MCVersion::new("a1.2.1", 2, 1); // November 4, 2010
pub static VA1_2_0_02: MCVersion = MCVersion::new("a1.2.0_02", 2, 2); // November 3, 2010
pub static VA1_2_0_01: MCVersion = MCVersion::new("a1.2.0_01", 2, 1); // October 30, 2010
pub static VA1_2_0: MCVersion = MCVersion::new("a1.2.0", 2, 0); // October 29, 2010

pub static VA1_1_2_01: MCVersion = MCVersion::new("a1.1.2_01", 1, 201); // September 22, 2010
pub static VA1_1_2: MCVersion = MCVersion::new("a1.1.2", 1, 2); // September 19, 2010
pub static VA1_1_0: MCVersion = MCVersion::new("a1.1.0", 1, 0); // September 12, 2010

pub static VA1_0_17_04: MCVersion = MCVersion::new("a1.0.17_04", 0, 1704); // August 22, 2010
pub static VA1_0_17_02: MCVersion = MCVersion::new("a1.0.17_02", 0, 1702); // August 19, 2010
pub static VA1_0_16: MCVersion = MCVersion::new("a1.0.16", 0, 16); // August 11, 2010
pub static VA1_0_15: MCVersion = MCVersion::new("a1.0.15", 0, 15); // August 3, 2010
pub static VA1_0_14: MCVersion = MCVersion::new("a1.0.14", 0, 14); // July 29, 2010
pub static VA1_0_11: MCVersion = MCVersion::new("a1.0.11", 0, 11); // July 22, 2010
pub static VA1_0_5_01: MCVersion = MCVersion::new("a1.0.5_01", 0, 501); // July 12, 2010
pub static VA1_0_4: MCVersion = MCVersion::new("a1.0.4", 0, 4); // July 8, 2010

lazy_static! {
    static ref VERSIONS: Vec<MCVersion> = vec![
        V1_19_2,
        V1_19_1,
        V1_19,
        V1_18_2,
        V1_18_1,
        V1_18,
        V1_17_1,
        V1_17,
        V1_16_5,
        V1_16_4,
        V1_16_3,
        V1_16_2,
        V1_16_1,
        V1_16,
        V1_15_2,
        V1_15_1,
        V1_15,
        V1_14_4,
        V1_14_3,
        V1_14_2,
        V1_14_1,
        V1_14,
        V1_13_2,
        V1_13_1,
        V1_13,
        V1_12_2,
        V1_12_1,
        V1_12,
        V1_11_2,
        V1_11_1,
        V1_11,
        V1_10_2,
        V1_10_1,
        V1_10,
        V1_9_4,
        V1_9_3,
        V1_9_2,
        V1_9_1,
        V1_9,
        V1_8_9,
        V1_8_8,
        V1_8_7,
        V1_8_6,
        V1_8_5,
        V1_8_4,
        V1_8_3,
        V1_8_2,
        V1_8_1,
        V1_8,
        V1_7_10,
        V1_7_9,
        V1_7_8,
        V1_7_7,
        V1_7_6,
        V1_7_5,
        V1_7_4,
        V1_7_3,
        V1_7_2,
        V1_6_4,
        V1_6_2,
        V1_6_1,
        V1_5_2,
        V1_5_1,
        V1_4_7,
        V1_4_6,
        V1_4_5,
        V1_4_4,
        V1_4_2,
        V1_3_2,
        V1_3_1,
        V1_2_5,
        V1_2_4,
        V1_2_3,
        V1_2_2,
        V1_2_1,
        V1_1,
        V1_0,
        VB1_8_1,
        VB1_8,
        VB1_7_3,
        VB1_7_2,
        VB1_7,
        VB1_6_6,
        VB1_6_5,
        VB1_6_4,
        VB1_6_3,
        VB1_6_2,
        VB1_6_1,
        VB1_6,
        VB1_5_01,
        VB1_5,
        VB1_4_01,
        VB1_4,
        VB1_3_01,
        VB1_3B,
        VB1_2_02,
        VB1_2_01,
        VB1_2,
        VB1_1_02,
        VB1_1_01,
        VB1_0_2,
        VB1_0_01,
        VB1_0,
        VA1_2_6,
        VA1_2_5,
        VA1_2_4_01,
        VA1_2_3_04,
        VA1_2_3_02,
        VA1_2_3_01,
        VA1_2_3,
        VA1_2_2B,
        VA1_2_2A,
        VA1_2_1_01,
        VA1_2_1,
        VA1_2_0_02,
        VA1_2_0_01,
        VA1_2_0,
        VA1_1_2_01,
        VA1_1_2,
        VA1_1_0,
        VA1_0_17_04,
        VA1_0_17_02,
        VA1_0_16,
        VA1_0_15,
        VA1_0_14,
        VA1_0_11,
        VA1_0_5_01,
        VA1_0_4
    ];
    static ref VERSIONS_MAP: HashMap<&'static str, &'static MCVersion> = {
        let mut map = HashMap::new();
        for version in VERSIONS.iter() {
            map.insert(version.name, version);
        }
        map
    };
}
