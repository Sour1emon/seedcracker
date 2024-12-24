use seedcracker::random::mth::MASK_48;
use seedcracker::{can_spawn_buried_treasure, check_seed};

#[test]
fn test_check_seed() {
    assert!(check_seed((-4872636734044769429_i64) as u64, -28, -73));
    assert!(!check_seed((-4872636734044769428_i64) as u64, -28, -73));
    assert!(!check_seed(7823457984237234584_i64 as u64, -28, -73));
}

#[test]
fn test_valid_structure_pos() {
    const CHUNK_X: i32 = -28;
    const CHUNK_Z: i32 = -73;

    let lower48 = ((-4872636734044769429) & MASK_48) as u64;
    assert!(can_spawn_buried_treasure(lower48, CHUNK_X, CHUNK_Z));
    for i in 0..0x10000 {
        assert!(can_spawn_buried_treasure(
            lower48 | (i << 48),
            CHUNK_X,
            CHUNK_Z
        ));
    }

    let lower48 = ((-754238579824375972) & MASK_48) as u64;
    assert!(!can_spawn_buried_treasure(lower48, CHUNK_X, CHUNK_Z));
    for i in 0..0x10000 {
        assert!(!can_spawn_buried_treasure(
            lower48 | (i << 48),
            CHUNK_X,
            CHUNK_Z
        ));
    }
}