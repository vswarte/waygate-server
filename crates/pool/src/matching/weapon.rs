use std::ops::RangeInclusive;

#[allow(unused)]
pub struct WeaponLevelTableEntry {
    regular: u32,
    special: u32,
    pub regular_range: RangeInclusive<u32>,
    special_range: RangeInclusive<u32>,
}

pub fn get_level_table_entry(level: u32) -> Option<&'static WeaponLevelTableEntry> {
    WEAPON_LEVEL_TABLE.iter().find(|e| e.regular == level)
}

const WEAPON_LEVEL_TABLE: [WeaponLevelTableEntry; 26] = [
    WeaponLevelTableEntry {
        regular: 0,
        special: 0,
        regular_range: (0..=3),
        special_range: (0..=1),
    },
    WeaponLevelTableEntry {
        regular: 1,
        special: 0,
        regular_range: (0..=4),
        special_range: (0..=1),
    },
    WeaponLevelTableEntry {
        regular: 2,
        special: 1,
        regular_range: (0..=5),
        special_range: (0..=2),
    },
    WeaponLevelTableEntry {
        regular: 3,
        special: 1,
        regular_range: (0..=6),
        special_range: (0..=2),
    },
    WeaponLevelTableEntry {
        regular: 4,
        special: 1,
        regular_range: (1..=7),
        special_range: (1..=3),
    },
    WeaponLevelTableEntry {
        regular: 5,
        special: 2,
        regular_range: (2..=8),
        special_range: (1..=3),
    },
    WeaponLevelTableEntry {
        regular: 6,
        special: 2,
        regular_range: (3..=10),
        special_range: (2..=4),
    },
    WeaponLevelTableEntry {
        regular: 7,
        special: 3,
        regular_range: (4..=11),
        special_range: (2..=4),
    },
    WeaponLevelTableEntry {
        regular: 8,
        special: 3,
        regular_range: (5..=12),
        special_range: (2..=5),
    },
    WeaponLevelTableEntry {
        regular: 9,
        special: 3,
        regular_range: (6..=13),
        special_range: (3..=5),
    },
    WeaponLevelTableEntry {
        regular: 10,
        special: 4,
        regular_range: (6..=14),
        special_range: (3..=5),
    },
    WeaponLevelTableEntry {
        regular: 11,
        special: 4,
        regular_range: (7..=15),
        special_range: (3..=6),
    },
    WeaponLevelTableEntry {
        regular: 12,
        special: 5,
        regular_range: (8..=17),
        special_range: (4..=7),
    },
    WeaponLevelTableEntry {
        regular: 13,
        special: 5,
        regular_range: (9..=18),
        special_range: (4..=7),
    },
    WeaponLevelTableEntry {
        regular: 14,
        special: 5,
        regular_range: (10..=19),
        special_range: (4..=7),
    },
    WeaponLevelTableEntry {
        regular: 15,
        special: 6,
        regular_range: (11..=20),
        special_range: (5..=8),
    },
    WeaponLevelTableEntry {
        regular: 16,
        special: 6,
        regular_range: (12..=21),
        special_range: (5..=8),
    },
    WeaponLevelTableEntry {
        regular: 17,
        special: 7,
        regular_range: (12..=22),
        special_range: (5..=9),
    },
    WeaponLevelTableEntry {
        regular: 18,
        special: 7,
        regular_range: (13..=24),
        special_range: (6..=9),
    },
    WeaponLevelTableEntry {
        regular: 19,
        special: 7,
        regular_range: (14..=25),
        special_range: (6..=10),
    },
    WeaponLevelTableEntry {
        regular: 20,
        special: 8,
        regular_range: (15..=25),
        special_range: (6..=10),
    },
    WeaponLevelTableEntry {
        regular: 21,
        special: 8,
        regular_range: (16..=25),
        special_range: (7..=10),
    },
    WeaponLevelTableEntry {
        regular: 22,
        special: 9,
        regular_range: (18..=25),
        special_range: (7..=10),
    },
    WeaponLevelTableEntry {
        regular: 23,
        special: 9,
        regular_range: (18..=25),
        special_range: (8..=10),
    },
    WeaponLevelTableEntry {
        regular: 24,
        special: 9,
        regular_range: (18..=25),
        special_range: (8..=10),
    },
    WeaponLevelTableEntry {
        regular: 25,
        special: 10,
        regular_range: (19..=25),
        special_range: (8..=10),
    },
];
