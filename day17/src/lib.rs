#[allow(dead_code)]
#[macro_use]
extern crate lazy_static;

use ahash::AHashSet;
use fxhash::FxHashSet;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

pub mod bitvector;

#[derive(Clone)]
#[cfg(feature = "hash-default")]
struct BitSet {
    data: HashSet<usize>,
}

#[derive(Clone)]
#[cfg(feature = "hash-xx")]
struct BitSet {
    data: HashSet<usize, BuildHasherDefault<XxHash64>>,
}

#[derive(Clone)]
#[cfg(feature = "hash-fx")]
struct BitSet {
    data: FxHashSet<usize>,
}

#[derive(Clone)]
#[cfg(feature = "hash-a")]
struct BitSet {
    data: AHashSet<usize>,
}

impl BitSet {
    fn new(_: usize) -> BitSet {
        BitSet {
            data: Default::default(),
        }
    }

    fn get(&self, index: usize) -> bool {
        return self.data.contains(&index);
    }

    fn count_ones(&self, start: usize, end: usize) -> usize {
        return (start..end)
            .map(|x| if self.data.contains(&x) { 1 } else { 0 })
            .sum();
    }

    fn count_all_ones(&self) -> usize {
        return self.data.len();
    }

    fn set(&mut self, index: usize, value: bool) {
        if value {
            self.data.insert(index);
        } else {
            self.data.remove(&index);
        }
    }
}

// A voxel map of the pocket universe.
pub struct Voxels {
    n: usize,
    nsquared: usize,
    bits: BitSet,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    max_z: isize,
}

impl Voxels {
    pub fn new(n: usize) -> Voxels {
        Voxels {
            n,
            nsquared: n * n,
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            max_z: 0,
            // Since world is mirrored in z-axis, we need N/2 planes for z.
            bits: BitSet::new(n * n * n / 2),
        }
    }

    pub fn initialize(&mut self, text: &str) {
        let lines = text.lines();
        let offset = (lines.clone().count() / 2) as isize;
        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let off_x: isize = x as isize - offset;
                let off_y: isize = y as isize - offset;
                match ch {
                    '#' => self.setbit(off_x, off_y, 0, true),
                    _ => (),
                }
            }
        }
    }

    pub fn count_all_ones(&self) -> usize {
        let sum = self.bits.count_all_ones();
        let mirrored: usize = self
            .bits
            .data
            .iter()
            .map(|index| self.index_to_z_coord(*index as usize))
            .filter(|z| *z != 0)
            .count();
        return sum + mirrored;
    }

    pub fn print(&self) {
        let x_len = (self.max_x - self.min_x + 1) as usize;
        for z in 0..(self.max_z + 1) {
            println!("z = {}", z);
            for y in self.min_y..(self.max_y + 1) {
                let index = self.coord_to_index(self.min_x, y, z);
                let s = (index..(index + x_len))
                    .map(|index| self.bits.get(index))
                    .map(|x| if x { '#' } else { '.' })
                    .collect::<String>();
                println!("{}", s);
            }
        }
    }

    fn coord_to_index(&self, x: isize, y: isize, z: isize) -> usize {
        let x = (x + (self.n as isize) / 2) as usize;
        let y = (y + (self.n as isize) / 2) as usize;
        let absz = z.abs() as usize; // because z is a mirrored axis of symmetry.
        return x + self.n * y + self.nsquared * absz;
    }

    fn index_to_z_coord(&self, index: usize) -> isize {
        let absz: usize = index / self.nsquared;
        let z: isize = (absz as isize) - ((self.n / 2) as isize);
        return z;
    }

    fn getbit(&self, x: isize, y: isize, z: isize) -> bool {
        if (x < self.min_x)
            || (x > self.max_x)
            || (y < self.min_y)
            || (y > self.max_y)
            || (z > self.max_z)
        {
            return false;
        }
        return self.bits.get(self.coord_to_index(x, y, z));
    }

    pub fn count_bit(&self, x: isize, y: isize, z: isize) -> usize {
        match self.getbit(x, y, z) {
            true => 1,
            false => 0,
        }
    }

    pub fn count_adjacent(&self, x: isize, y: isize, z: isize) -> usize {
        let mut count = 0usize;
        // count of bits in cube:
        for adj_z in (z - 1)..(z + 2) {
            for adj_y in (y - 1)..(y + 2) {
                let index = self.coord_to_index(x - 1, adj_y, adj_z);
                count += self.bits.count_ones(index, index + 3);
            }
        }
        count -= self.count_bit(x, y, z); // subtract the middle bit.
        return count;
    }

    // modifies self.bits
    fn setbit(&mut self, x: isize, y: isize, z: isize, value: bool) {
        if x < self.min_x {
            self.min_x = x
        }
        if y < self.min_y {
            self.min_y = y
        }
        if x > self.max_x {
            self.max_x = x
        }
        if y > self.max_y {
            self.max_y = y
        }
        if z > self.max_z {
            self.max_z = z
        }
        // TODO: implement resize if min/max dim > n.
        let index = self.coord_to_index(x, y, z);
        self.bits.set(index, value);
        assert_eq!(value, self.getbit(x, y, z));
    }

    // Apply rules:
    //
    // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube
    // remains active. Otherwise, the cube becomes inactive.
    //
    // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes
    // active. Otherwise, the cube remains inactive.
    //
    // Summary:
    //   active   && count != 2 or 3 -> inactive
    //   inactive && count == 3      -> active
    //
    // For example:
    //
    //        01110    .....
    // .#.    01121    .....
    // ..# -> 13532 -> .#.#.
    // ###    11322    ..##.
    //        12321    ..#..
    //
    pub fn update(&mut self) -> Voxels {
        let n = self.n;
        let mut v = Voxels {
            n,
            nsquared: n * n,
            min_x: self.min_x,
            max_x: self.max_x,
            min_y: self.min_y,
            max_y: self.max_y,
            max_z: self.max_z,
            // Since world is mirrored in z-axis, we need N/2 planes for z.
            bits: BitSet::new(n * n * n / 2),
        };
        v.bits = self.bits.clone();
        for z in 0..(self.max_z + 2) {
            for y in (self.min_y - 1)..(self.max_y + 2) {
                for x in (self.min_x - 1)..(self.max_x + 2) {
                    // this failed when using BitVec::splice:
                    assert_eq!(self.getbit(x, y, z), v.getbit(x, y, z));
                    let count = self.count_adjacent(x, y, z);
                    let active = self.getbit(x, y, z);
                    if active {
                        if (count < 2) || (count > 3) {
                            v.setbit(x, y, z, false);
                        }
                    } else {
                        // inactive
                        if count == 3 {
                            v.setbit(x, y, z, true);
                        }
                    }
                } // x
            } // y
        } // z
        return v;
    }
} // impl Voxels

// Like Voxels, but in 4 dimensions.
//
// While the result is going to have z and w axis symmetry, I found myself having to think too hard
// about how symmetry works in 4 dimensions so I dropped that optimization.
pub struct HyperVoxels {
    n: usize,
    nsquared: usize,
    bits: BitSet,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
    min_w: isize,
    max_w: isize,
    offsets: Vec<usize>,
}

impl HyperVoxels {
    fn new(n: usize) -> HyperVoxels {
        let mut this = HyperVoxels {
            n,
            nsquared: n * n,
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
            bits: BitSet::new(n * n * n * n),
            offsets: Vec::new(),
        };
        for w in -1..=1 {
            for z in -1..=1 {
                for y in -1..=1 {
                    for x in -1..=1 {
                        if (w == 0) && (z == 0) && (y == 0) && (x == 0) {
                            continue;
                        }
                        this.offsets.push(this.coord_to_index(x, y, z, w));
                    }
                }
            }
        }
        this
    }

    fn initialize(&mut self, text: &str) {
        let lines = text.lines();
        let offset = (lines.clone().count() / 2) as isize;
        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let off_x: isize = x as isize - offset;
                let off_y: isize = y as isize - offset;
                match ch {
                    '#' => self.setbit(off_x, off_y, 0, 0, true),
                    _ => (),
                }
            }
        }
    }

    fn count_all_ones(&self) -> usize {
        return self.bits.count_all_ones();
    }

    fn coord_to_index(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let x = (x + (self.n as isize) / 2) as usize;
        let y = (y + (self.n as isize) / 2) as usize;
        let z = (z + (self.n as isize) / 2) as usize;
        let w = (w + (self.n as isize) / 2) as usize;
        return x + self.n * y + self.nsquared * z + self.n * self.nsquared * w;
    }

    fn getbit(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        if (x < self.min_x)
            || (x > self.max_x)
            || (y < self.min_y)
            || (y > self.max_y)
            || (z < self.min_z)
            || (z > self.max_z)
            || (w < self.min_w)
            || (w > self.max_w)
        {
            return false;
        }
        return self.bits.get(self.coord_to_index(x, y, z, w));
    }

    fn count_bit(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        match self.getbit(x, y, z, w) {
            true => 1,
            false => 0,
        }
    }

    fn count_adjacent(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut count = 0usize;
        for adj_w in (w - 1)..(w + 2) {
            for adj_z in (z - 1)..(z + 2) {
                for adj_y in (y - 1)..(y + 2) {
                    let index = self.coord_to_index(x - 1, adj_y, adj_z, adj_w);
                    count += self.bits.count_ones(index, index + 3);
                }
            }
        }
        count -= self.count_bit(x, y, z, w); // subtract the middle bit.
        return count;
    }

    // modifies self.bits
    fn setbit(&mut self, x: isize, y: isize, z: isize, w: isize, value: bool) {
        if x < self.min_x {
            self.min_x = x
        }
        if y < self.min_y {
            self.min_y = y
        }
        if x > self.max_x {
            self.max_x = x
        }
        if y > self.max_y {
            self.max_y = y
        }
        if z < self.min_z {
            self.min_z = z
        }
        if z > self.max_z {
            self.max_z = z
        }
        if w < self.min_w {
            self.min_w = w
        }
        if w > self.max_w {
            self.max_w = w
        }
        // TODO: implement resize if min/max dim > n.
        let index = self.coord_to_index(x, y, z, w);
        self.bits.set(index, value);
    }

    // Apply rules:
    //
    // These are the same rules as Voxel, except now we have 80 neighbors instead of 26.
    //
    // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube
    // remains active. Otherwise, the cube becomes inactive.
    //
    // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes
    // active. Otherwise, the cube remains inactive.
    //
    // Summary:
    //   active   && count != 2 or 3 -> inactive
    //   inactive && count == 3      -> active
    //
    // For example:
    //
    //        01110    .....
    // .#.    01121    .....
    // ..# -> 13532 -> .#.#.
    // ###    11322    ..##.
    //        12321    ..#..
    //
    fn update(&mut self) -> HyperVoxels {
        let n = self.n;
        let mut v = HyperVoxels {
            n,
            nsquared: n * n,
            min_x: self.min_x,
            max_x: self.max_x,
            min_y: self.min_y,
            max_y: self.max_y,
            min_z: self.min_z,
            max_z: self.max_z,
            min_w: self.min_w,
            max_w: self.max_w,
            // Since world is mirrored in z-axis, we need N/2 planes for z.
            bits: BitSet::new(n * n * n * n),
            offsets: self.offsets.clone(),
        };
        v.bits = self.bits.clone();

        for active_index in self.bits.data.iter() {
            for offset in self.offsets.iter() {
                // check each neighbor
                let check_index = active_index + offset;
                // count neighbors of "check_index" voxel:
                let count = self
                    .offsets
                    .iter()
                    .map(|x| check_index + x)
                    .filter(|x| self.bits.data.contains(x))
                    .count();
                let active = self.bits.data.contains(&check_index);
                if active {
                    if (count < 2) || (count > 3) {
                        v.bits.data.remove(&check_index);
                    }
                } else {
                    // inactive
                    if count == 3 {
                        v.bits.data.insert(check_index);
                    }
                }
            } // offset
        } // active_index
        return v;
    }
} // impl HyperVoxels

pub fn part1(contents: &str) -> usize {
    let mut voxels = Voxels::new(128);
    voxels.initialize(contents);
    voxels.print();
    for _ in 0..6 {
        voxels = voxels.update();
    }
    return voxels.count_all_ones();
}

pub fn part2(contents: &str) -> usize {
    let mut voxels = HyperVoxels::new(64);
    voxels.initialize(contents);
    for _ in 0..6 {
        voxels = voxels.update();
    }
    return voxels.count_all_ones();
}

pub fn bv_part1(contents: &str) -> usize {
    let mut voxels = bitvector::VoxelsBV::new(128);
    voxels.initialize(contents);
    voxels.print();
    for _ in 0..6 {
        voxels = voxels.update();
    }
    return voxels.count_all_ones();
}

pub fn bv_part2(contents: &str) -> usize {
    let mut voxels = bitvector::HyperVoxelsBV::new(64);
    voxels.initialize(contents);
    for _ in 0..6 {
        voxels = voxels.update();
    }
    return voxels.count_all_ones();
}
