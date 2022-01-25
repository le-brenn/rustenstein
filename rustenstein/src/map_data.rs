/// Dump of the first plane of the first map, with wall and door placement
pub const E1_M1_TILES: [[u8; 64]; 64] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 3,
        2, 1, 1, 1, 1, 1, 2, 1, 4, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 117, 117, 117, 1, 114, 114,
        114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114,
        114, 114, 114, 114, 114, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 117, 117, 117, 91, 114, 114,
        114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114,
        114, 114, 114, 114, 114, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 117, 117, 117, 1, 114, 114,
        114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114, 114,
        114, 114, 114, 114, 114, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 114, 1, 1, 3,
        1, 1, 114, 114, 114, 1, 1, 1, 4, 2, 1, 1, 114, 2, 1, 2, 114, 114, 114, 2, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 1, 2, 1, 3, 1, 1, 1, 1, 1, 114, 114, 114,
        2, 1, 2, 2, 1, 90, 1, 1, 1, 1, 1, 1, 1, 114, 114, 114, 1, 1, 114, 114, 114, 2, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 113, 113, 113, 113, 2, 113, 113, 113, 113, 113, 113, 113, 2, 1,
        1, 2, 114, 114, 114, 1, 112, 112, 112, 112, 112, 112, 112, 112, 112, 2, 1, 1, 114, 114,
        114, 2, 2, 114, 114, 114, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 113, 113, 113, 113, 6, 113, 113, 113, 113, 113, 113, 113, 6, 1,
        1, 1, 114, 114, 114, 2, 112, 112, 112, 112, 112, 112, 112, 112, 112, 1, 1, 1, 114, 114,
        114, 1, 1, 114, 114, 114, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 113, 113, 113, 113, 1, 113, 113, 113, 113, 113, 113, 113, 1, 1,
        3, 1, 1, 3, 2, 1, 112, 112, 112, 112, 112, 112, 112, 112, 112, 1, 1, 2, 1, 3, 2, 1, 2, 114,
        114, 114, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 113, 113, 113, 113, 2, 113, 113, 113, 113, 113, 113, 113, 1,
        141, 141, 141, 141, 141, 141, 1, 112, 112, 112, 112, 112, 112, 112, 112, 112, 2, 114, 114,
        114, 114, 114, 114, 114, 114, 114, 114, 1, 1, 21, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 113, 113, 4, 113, 113, 113, 113, 113, 113, 113, 91, 141,
        141, 141, 141, 141, 141, 91, 112, 112, 112, 112, 112, 112, 112, 112, 112, 91, 114, 114,
        114, 114, 114, 114, 114, 114, 114, 114, 4, 21, 107, 21, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 113, 113, 113, 113, 113, 113, 113, 1, 141, 141,
        141, 141, 141, 141, 1, 112, 112, 112, 112, 112, 112, 112, 112, 112, 1, 114, 114, 114, 114,
        114, 114, 114, 114, 114, 114, 1, 3, 100, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 113, 113, 113, 113, 113, 113, 113, 1, 2, 3, 2, 1,
        3, 1, 1, 112, 112, 112, 112, 112, 112, 112, 112, 112, 1, 1, 2, 1, 3, 2, 2, 1, 114, 114,
        114, 1, 140, 140, 140, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 6, 113, 113, 113, 113, 113, 113, 113, 6, 1, 1, 1, 1,
        1, 1, 2, 112, 112, 112, 112, 112, 112, 112, 112, 112, 2, 1, 1, 1, 1, 1, 1, 2, 114, 114,
        114, 1, 140, 140, 140, 1, 140, 140, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 113, 113, 113, 113, 113, 113, 113, 2, 1, 1, 1, 1,
        1, 1, 1, 112, 112, 112, 112, 112, 112, 112, 112, 112, 2, 1, 1, 1, 1, 1, 1, 1, 114, 114,
        114, 2, 1, 1, 140, 1, 140, 140, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 90, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        3, 1, 2, 3, 1, 2, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 114, 114, 114, 1, 140, 140, 140, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 142, 142, 142, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 114, 114, 114, 1, 140, 140, 140, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 2, 142, 142, 142, 2, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 90, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 142, 142, 142, 142, 142, 142, 142, 142, 142, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 140, 140, 140, 1, 140,
        140, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 142, 142, 142, 142, 142, 142, 142, 142, 142, 6, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 140, 140, 140, 2, 140, 2,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 142, 142, 142, 142, 142, 142, 142, 142, 142, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 140, 140, 140, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 142, 142, 142, 1, 1, 1, 142, 142, 142, 2, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 6, 140, 140, 140, 6, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 142, 142, 142, 1, 1, 2, 142, 142, 142, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 140, 140, 140, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 142, 142, 142, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 140, 140, 106, 2, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 142, 142, 142, 3, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 100, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 142, 142, 142, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 21, 140, 21, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 142, 142, 142, 1, 1, 1, 1, 1, 1, 1, 12, 12, 12, 12, 12, 12,
        12, 12, 1, 2, 1, 6, 1, 2, 2, 6, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 21, 1, 2, 1, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 142, 142, 142, 2, 1, 1, 1, 1, 1, 1, 12, 12, 12, 12, 12, 12,
        12, 12, 1, 1, 108, 108, 108, 108, 108, 108, 108, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
        9, 8, 8, 9, 8, 8, 9, 8, 8, 8, 9, 8, 8,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 12, 10, 12, 90, 12, 11, 12, 12, 12, 1, 1, 1, 12, 12, 110, 110,
        110, 110, 12, 12, 1, 2, 108, 108, 108, 108, 108, 108, 108, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 8, 8, 109, 109, 109, 109, 8, 109, 109, 109, 9, 109, 109, 109, 5,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 12, 12, 1, 1, 1, 12, 12, 110,
        110, 110, 110, 12, 12, 1, 1, 108, 108, 108, 108, 108, 108, 108, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 8, 9, 109, 109, 109, 109, 9, 109, 109, 109, 9, 109, 109, 109, 9,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 12, 12, 12, 12, 12, 12, 12,
        10, 12, 12, 12, 12, 12, 1, 3, 108, 108, 108, 108, 108, 108, 108, 2, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 8, 8, 109, 109, 109, 109, 8, 109, 109, 109, 8, 109, 109, 109, 8,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 10, 143, 143, 143, 143, 143, 143, 143, 10, 12, 12, 12, 12, 12, 12,
        106, 12, 12, 12, 12, 12, 1, 2, 108, 108, 108, 108, 108, 108, 108, 1, 1, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 9, 109, 109, 109, 109, 8, 109, 109, 109, 8, 109, 109, 109, 5,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 12, 12, 12, 11, 12, 12, 12,
        110, 12, 12, 12, 10, 12, 12, 108, 108, 108, 108, 108, 108, 108, 108, 108, 8, 8, 9, 8, 9, 9,
        8, 9, 8, 9, 9, 8, 8, 8, 90, 8, 9, 9, 8, 90, 8, 9, 109, 109, 109, 8,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 12, 110, 110, 110, 110, 110,
        110, 110, 110, 110, 110, 110, 110, 10, 108, 108, 108, 108, 108, 108, 108, 108, 108, 8, 108,
        108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 8, 109, 109, 109, 109, 109, 109, 109,
        109, 109, 109, 109, 109, 9,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 91, 110, 110, 110, 110, 110,
        110, 110, 110, 110, 110, 110, 110, 91, 108, 108, 108, 108, 108, 108, 108, 108, 108, 91,
        108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 91, 109, 109, 109, 109, 109, 109,
        109, 109, 109, 109, 109, 109, 7,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 12, 110, 110, 110, 110, 110,
        110, 110, 110, 110, 110, 110, 110, 10, 108, 108, 108, 108, 108, 108, 108, 108, 108, 9, 108,
        108, 108, 108, 108, 108, 108, 108, 108, 108, 108, 8, 109, 109, 109, 109, 109, 109, 109,
        109, 109, 109, 109, 109, 8,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 12, 12, 12, 11, 12, 12, 12,
        110, 12, 12, 12, 10, 12, 12, 108, 108, 108, 108, 108, 108, 108, 108, 108, 8, 9, 9, 9, 8, 9,
        8, 8, 9, 9, 8, 9, 8, 8, 90, 8, 9, 8, 8, 90, 8, 9, 109, 109, 109, 9,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 10, 143, 143, 143, 143, 143, 143, 143, 10, 12, 12, 12, 12, 12, 12,
        110, 12, 12, 12, 12, 12, 2, 1, 108, 108, 108, 108, 108, 108, 108, 1, 8, 8, 8, 8, 8, 8, 8,
        9, 8, 8, 8, 8, 8, 109, 109, 109, 109, 8, 109, 109, 109, 8, 109, 109, 109, 5,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 12, 12, 12, 1, 1, 12, 12, 12,
        12, 12, 12, 1, 1, 2, 3, 108, 108, 108, 108, 108, 108, 108, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 8, 8, 109, 109, 109, 109, 8, 109, 109, 109, 8, 109, 109, 109, 9,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 143, 143, 143, 143, 143, 143, 143, 12, 12, 12, 1, 1, 12, 12, 12,
        12, 12, 12, 1, 1, 2, 1, 108, 108, 108, 108, 108, 108, 108, 4, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 8, 8, 109, 109, 109, 109, 8, 109, 109, 109, 9, 109, 109, 109, 8,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 12, 10, 12, 90, 12, 11, 12, 12, 12, 12, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 2, 2, 108, 108, 108, 108, 108, 108, 108, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8, 9,
        109, 109, 109, 109, 8, 109, 109, 109, 8, 109, 109, 109, 5,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 12, 12, 142, 142, 142, 12, 12, 12, 12, 12, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 2, 1, 108, 108, 108, 108, 108, 108, 108, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8, 8,
        9, 9, 9, 9, 8, 8, 9, 8, 8, 8, 8, 8, 8,
    ],
    [
        1, 1, 1, 1, 1, 1, 12, 12, 12, 12, 142, 106, 142, 12, 12, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 2, 1, 2, 6, 108, 108, 108, 6, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 12, 12, 142, 142, 142, 12, 12, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 2, 2, 2, 9, 9, 90, 8, 8, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 12, 12, 12, 10, 12, 12, 12, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 9, 8, 111, 111, 111, 8, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 12, 12, 12, 12, 12, 12, 12, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 9, 8, 111, 111, 111, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        9, 8, 111, 111, 111, 111, 8, 9, 9, 8, 9, 8, 9, 8, 8, 9, 8, 8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        9, 9, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 9, 8, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        9, 8, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 111, 8, 8, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        9, 8, 111, 111, 111, 111, 8, 9, 8, 9, 8, 9, 8, 111, 111, 111, 8, 8, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        9, 8, 111, 111, 111, 9, 8, 8, 8, 8, 8, 8, 8, 8, 90, 8, 8, 8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        9, 9, 111, 111, 111, 9, 8, 8, 9, 9, 9, 8, 9, 118, 118, 118, 9, 8, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 9, 9,
        9, 9, 111, 111, 111, 8, 8, 9, 9, 9, 9, 8, 118, 118, 118, 118, 118, 8, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 8, 8,
        8, 8, 8, 90, 9, 8, 9, 8, 9, 9, 9, 8, 8, 118, 118, 118, 8, 8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 8, 139,
        139, 139, 139, 139, 139, 139, 139, 139, 9, 8, 9, 8, 118, 118, 118, 118, 118, 8, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 9, 139,
        139, 139, 139, 139, 139, 139, 139, 139, 8, 9, 9, 8, 9, 118, 118, 118, 8, 8, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 8, 139,
        139, 139, 139, 139, 139, 139, 139, 139, 139, 9, 9, 8, 118, 118, 118, 118, 118, 8, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 8, 139,
        139, 139, 139, 139, 139, 139, 139, 139, 9, 9, 9, 8, 9, 118, 118, 118, 8, 8, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 8, 139,
        139, 139, 139, 139, 139, 139, 139, 139, 8, 9, 9, 8, 118, 118, 118, 118, 118, 8, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 9, 139,
        139, 139, 139, 139, 139, 139, 139, 139, 139, 9, 9, 8, 8, 118, 118, 118, 8, 8, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 9, 139,
        139, 139, 139, 139, 139, 139, 139, 139, 9, 9, 9, 8, 9, 8, 5, 8, 9, 8, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 9, 139,
        139, 139, 139, 139, 139, 139, 139, 139, 8, 9, 9, 8, 8, 8, 8, 8, 8, 8, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 8, 8,
        9, 8, 9, 5, 9, 8, 9, 8, 9, 9, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1,
    ],
];

/// Dump of the second plane of the first map, with actor and object placement
pub const E1_M1_ACTORS: [[u8; 64]; 64] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 37, 0, 0, 0, 37, 0, 0, 0,
        37, 0, 0, 0, 96, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 37, 151, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 49, 49, 49, 0, 0, 0, 0, 62, 0, 54, 54, 0, 0, 0, 0, 0, 37, 0,
        0, 180, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 49, 49, 49, 0, 52, 26, 0, 0, 0, 26, 54, 0, 0, 0, 0, 47, 110,
        53, 0, 0, 34, 0, 0, 114, 0, 0, 34, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 54, 54, 54, 0, 53, 0, 108, 0, 0, 180, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 37, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 52, 54, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 25, 0, 0, 0, 0, 0, 37, 0, 0, 37, 181,
        0, 0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 185, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 37, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        34, 0, 0, 0, 0, 0, 34, 181, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 0, 0, 0, 0, 181, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 109, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 182, 0, 146, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        34, 0, 0, 90, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 49, 50, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 180, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48, 0, 0, 0,
        0, 183, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 20,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 37, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 108, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 124, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 27, 0, 0, 0, 0, 0, 147, 37, 0, 0, 0, 37, 0, 0, 0, 37, 0,
        0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 37, 183, 0, 0, 37, 0, 0, 0, 37, 0, 0, 0,
        0, 37, 0, 0, 0, 37, 47, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0,
        0, 0, 37, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 146, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0,
        0, 49, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        34, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 182, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 52, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 212, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 181, 0, 0, 0, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47,
        0, 0, 0, 0, 96, 177, 92, 0, 0, 0, 0, 0, 0, 0, 93, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 36, 0, 0, 59, 0, 0, 0, 0, 176, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 94, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58,
        0, 110, 0, 0, 0, 36, 0, 0, 60, 0, 0, 0, 0, 0, 212, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58,
        0, 0, 0, 94, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58,
        58, 0, 0, 0, 0, 182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
];


pub enum Tile {
    Floor,
    Wall(u8),
    Door{vertical: bool, lock: u8},
}

pub enum Direction {
    North,
    East,
    South,
    West
}

pub enum Actor {
    Player(Direction),
    Enemy, // TODO differentiate enemy types
    Item, // TODO differentiate item types
    DeadGuard,
    PushWall
}

pub fn tile_at(x: u8, y: u8) -> Tile {
    let tile = E1_M1_TILES[x as usize][y as usize];
    match tile {
        90 | 92 | 94 | 96 | 98 | 100 => Tile::Door{vertical: true, lock: (tile - 90) / 2},
        91 | 93 | 95 | 97 | 99 | 101 => Tile::Door{vertical: false, lock: (tile - 91) / 2},
        106 => Tile::Floor, // this one is actually an ambush tile, review if we need to do something with it
        n if n < 107 => Tile::Wall(tile), // keep the tile number to find the proper texture
        _ => Tile::Floor
    }
}

pub fn actor_at(x: u8, y: u8) -> Option<Actor> {
    match E1_M1_ACTORS[x as usize][y as usize] {
        19 => Some(Actor::Player(Direction::North)),
        20 => Some(Actor::Player(Direction::East)),
        21 => Some(Actor::Player(Direction::South)),
        22 => Some(Actor::Player(Direction::West)),
        n if n >= 23 && n <= 72 => Some(Actor::Item),
        98 => Some(Actor::PushWall),
        124 => Some(Actor::DeadGuard),
        n if n >= 108 => Some(Actor::Enemy),
        _ => None
    }
}
