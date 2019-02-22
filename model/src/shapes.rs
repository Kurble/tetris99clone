const I: [[u8; 16]; 4] = [[
    0,0,0,0,
    1,1,1,1,
    0,0,0,0,
    0,0,0,0,
],[
    0,0,1,0,
    0,0,1,0,
    0,0,1,0,
    0,0,1,0,
],[
    0,0,0,0,
    0,0,0,0,
    1,1,1,1,
    0,0,0,0,
],[
    0,1,0,0,
    0,1,0,0,
    0,1,0,0,
    0,1,0,0,
]];

const O: [[u8; 16]; 4] = [[
    0,0,0,0,
    0,2,2,0,
    0,2,2,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,2,2,0,
    0,2,2,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,2,2,0,
    0,2,2,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,2,2,0,
    0,2,2,0,
    0,0,0,0,
]];

const T: [[u8; 16]; 4] = [[
    0,0,0,0,
    0,3,0,0,
    3,3,3,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,3,0,0,
    0,3,3,0,
    0,3,0,0,
],[
    0,0,0,0,
    3,3,3,0,
    0,3,0,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,3,0,0,
    3,3,0,0,
    0,3,0,0,
]];

const J: [[u8; 16]; 4] = [[
    0,0,0,0,
    0,4,0,0,
    0,4,0,0,
    4,4,0,0,
],[
    0,0,0,0,
    4,0,0,0,
    4,4,4,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,4,4,0,
    0,4,0,0,
    0,4,0,0,
],[
    0,0,0,0,
    4,4,4,0,
    0,0,4,0,
    0,0,0,0,
]];

const L: [[u8; 16]; 4] = [[
    0,0,0,0,
    0,5,0,0,
    0,5,0,0,
    0,5,5,0,
],[
    0,0,0,0,
    5,5,5,0,
    5,0,0,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,5,5,0,
    0,0,5,0,
    0,0,5,0,
],[
    0,0,0,0,
    0,0,5,0,
    5,5,5,0,
    0,0,0,0,
]];

const S: [[u8; 16]; 4] = [[
    0,0,0,0,
    0,6,6,0,
    6,6,0,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,6,0,0,
    0,6,6,0,
    0,0,6,0,
],[
    0,0,0,0,
    0,0,0,0,
    0,6,6,0,
    6,6,0,0,
],[
    0,0,0,0,
    6,0,0,0,
    6,6,0,0,
    0,6,0,0,
]];

const Z: [[u8; 16]; 4] = [[
    0,0,0,0,
    7,7,0,0,
    0,7,7,0,
    0,0,0,0,
],[
    0,0,0,0,
    0,0,7,0,
    0,7,7,0,
    0,7,0,0,
],[
    0,0,0,0,
    0,0,0,0,
    7,7,0,0,
    0,7,7,0,
],[
    0,0,0,0,
    0,7,0,0,
    7,7,0,0,
    7,0,0,0,
]];

pub const SHAPES: [[[u8; 16]; 4]; 7]  = [I, O, T, J, L, S, Z];