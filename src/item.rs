#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Item {
    HeartOfTheSea,
    IronIngot,
    GoldIngot,
    TNT,
    Emerald,
    Diamond,
    PrismarineCrystals,
    LeatherChestplate,
    IronSword,
    CookedCod,
    CookedSalmon,
    Empty,
}

impl Item {
    pub const fn equals(&self, other: &Self) -> bool {
        *self as isize == *other as isize
    }
}