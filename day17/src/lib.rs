// I rolled my own BitVector class after having difficulty with the BitVec crate.  In particular, I
// kept using BitVec::splice to attempt to set a single bit, and BitVec::splice kept performing
// unexpected shifts on my bit vector.  Instead of diving deeper, I threw away BitVec and rewrote
// it in about 10 minutes.
//
// Getting back to BitVec now, I think I see the problem.  I get the impression that BitVec::splice
// was never meant to be a public interface, and contains ominous warnings such as "It is
// unspecified how many bits are removed from the vector if the Splice value is leaked."  While I
// thought BitVec::splice was performing the splice operation, it now seems that it was returning
// an iterator that had to be drained for splice to complete.  Muy confusing!
//
// It turns out that what I really wanted was to get a mutable BitSlice object that covered my
// BitVec vector, and that the BitSlice object would provide the set method I was hoping for:
//
//    fn set(&mut self, index: usize, value: bool) {
//       self.bits.as_mut_bitslice().set(index, value);
//    }
//      
// Ah well.
#[derive(Clone)]
struct BitVector {
    data: Vec<u64>,
}

impl BitVector {
    fn new(bits: usize) -> BitVector {
        let words: usize = (bits + 63) / 64;
        let mut bv = BitVector {
            data: Vec::new(),
        };
        bv.data.resize(words, 0);
        return bv;
    }

    fn get(&self, index: usize) -> bool {
        let word = index / 64;
        let mask: u64 = 1 << (index % 64);
        return self.data[word] & mask != 0;
    }

    fn count_ones(&self, start: usize, end: usize) -> usize {
        return (start..end)
            .map(|i| if self.get(i) { 1 } else { 0 })
            .sum();
    }

    fn set(&mut self, index: usize, value: bool) {
        let word = index / 64;
        let mask: u64 = 1 << (index % 64);
        let mut data: u64 = self.data[word];
        data &= !mask;
        data |= if value { mask } else { 0 };
        self.data[word]  = data;
    }
        
}

// A voxel map of the pocket universe.
pub struct Voxels {
    n: usize,
    nsquared: usize,
    bits: BitVector,
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
            min_x: 0, max_x: 0,
            min_y: 0, max_y: 0,
            max_z: 0,
            // Since world is mirrored in z-axis, we need N/2 planes for z.
            bits: BitVector::new(n*n*n/2),
        }
    }

    pub fn initialize(&mut self, text: &str) {
        let lines = text.lines();
        let offset = (lines.clone().count()/2) as isize;
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
        let end_of_z0 = self.nsquared / 64;
        let z0_sum: usize = (0..end_of_z0)
            .map(|i| self.bits.data[i].count_ones() as usize)
            .sum();
        let rest_sum: usize = (end_of_z0..self.bits.data.len())
            .map(|i| self.bits.data[i].count_ones() as usize)
            .sum();
        return z0_sum + 2 * rest_sum;
    }

    pub fn print(&self) {
        let x_len = (self.max_x - self.min_x + 1) as usize;
        for z in 0..(self.max_z+1) {
            println!("z = {}", z);
            for y in self.min_y..(self.max_y+1) {
                let index = self.coord_to_index(self.min_x, y, z);
                let s = (index..(index+x_len))
                    .map(|index| self.bits.get(index))
                    .map(|x| if x { '#' } else { '.' })
                    .collect::<String>();
                println!("{}", s);
            }
        }
    }

    fn coord_to_index(&self, x: isize, y: isize, z: isize) -> usize {
        let x = (x + (self.n as isize)/2) as usize;
        let y = (y + (self.n as isize)/2) as usize;
        let absz = z.abs() as usize;  // because z is a mirrored axis of symmetry.
        return x + self.n*y + self.nsquared*absz;
    }

    fn getbit(&self, x: isize, y: isize, z: isize) -> bool {
        if  (x < self.min_x) || (x > self.max_x) ||
            (y < self.min_y) || (y > self.max_y) ||
            (z > self.max_z) {
                return false;
        }
        return  self.bits.get(self.coord_to_index(x, y, z));
    }

    pub fn count_bit(&self, x: isize, y: isize, z: isize) -> usize {
        match self.getbit(x, y, z) {
            true => 1,
            false => 0
        }
    }


    pub fn count_adjacent(&self, x: isize, y: isize, z: isize) -> usize {
        let mut count = 0usize;
        // count of bits in cube:
        for adj_z in (z-1)..(z+2) {
          for adj_y in (y-1)..(y+2) {
              let index = self.coord_to_index(x-1, adj_y, adj_z);
              count += self.bits.count_ones(index, index+3);
          }
        }
        count -= self.count_bit(x, y, z);  // subtract the middle bit.
        return count;
    }

    // modifies self.bits
    fn setbit(&mut self, x: isize, y: isize, z: isize, value: bool) {
        if x < self.min_x { self.min_x = x }
        if y < self.min_y { self.min_y = y }
        if x > self.max_x { self.max_x = x }
        if y > self.max_y { self.max_y = y }
        if z > self.max_z { self.max_z = z }
        // TODO: implement resize if min/max dim > n.
        let index = self.coord_to_index(x,y,z);
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
            min_x: self.min_x, max_x: self.max_x,
            min_y: self.min_y, max_y: self.max_y,
            max_z: self.max_z,
            // Since world is mirrored in z-axis, we need N/2 planes for z.
            bits: BitVector::new(n*n*n/2),
        };
        v.bits = self.bits.clone();
        for z in 0..(self.max_z + 2) {
            for y in (self.min_y - 1)..(self.max_y + 2) {
                for x in (self.min_x - 1)..(self.max_x + 2) {
                    // this failed when using BitVec::splice:
                    assert_eq!(self.getbit(x,y,z), v.getbit(x,y,z));
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
                }  // x
            }  // y
        }  // z
        return v;
    }
}  // impl Voxels 

// Like Voxels, but in 4 dimensions.
//
// While the result is going to have z and w axis symmetry, I found myself having to think too hard
// about how symmetry works in 4 dimensions so I dropped that optimization.
struct HyperVoxels {
    n: usize,
    nsquared: usize,
    bits: BitVector,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
    min_w: isize,
    max_w: isize,
}

impl HyperVoxels {
    fn new(n: usize) -> HyperVoxels {
        HyperVoxels {
            n,
            nsquared: n * n,
            min_x: 0, max_x: 0,
            min_y: 0, max_y: 0,
            min_z: 0, max_z: 0,
            min_w: 0, max_w: 0,
            bits: BitVector::new(n*n*n*n),
        }
    }

    fn initialize(&mut self, text: &str) {
        let lines = text.lines();
        let offset = (lines.clone().count()/2) as isize;
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
        return self.bits.data.iter()
            .map(|&x| x.count_ones() as usize)
            .sum();
    }

    fn coord_to_index(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let x = (x + (self.n as isize)/2) as usize;
        let y = (y + (self.n as isize)/2) as usize;
        let z = (z + (self.n as isize)/2) as usize;
        let w = (w + (self.n as isize)/2) as usize;
        return x + self.n*y + self.nsquared*z + self.n*self.nsquared * w;
    }

    fn getbit(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        if  (x < self.min_x) || (x > self.max_x) ||
            (y < self.min_y) || (y > self.max_y) ||
            (z < self.min_z) || (z > self.max_z) ||
            (w < self.min_w) || (w > self.max_w) {
                return false;
        }
        return  self.bits.get(self.coord_to_index(x, y, z, w));
    }

    fn count_bit(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        match self.getbit(x, y, z, w) {
            true => 1,
            false => 0
        }
    }


    fn count_adjacent(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut count = 0usize;
        for adj_w in (w-1)..(w+2) {
            for adj_z in (z-1)..(z+2) {
              for adj_y in (y-1)..(y+2) {
                  let index = self.coord_to_index(x-1, adj_y, adj_z, adj_w);
                  count += self.bits.count_ones(index, index+3);
              }
            }
        }
        count -= self.count_bit(x, y, z, w);  // subtract the middle bit.
        return count;
    }

    // modifies self.bits
    fn setbit(&mut self, x: isize, y: isize, z: isize, w: isize, value: bool) {
        if x < self.min_x { self.min_x = x }
        if y < self.min_y { self.min_y = y }
        if x > self.max_x { self.max_x = x }
        if y > self.max_y { self.max_y = y }
        if z < self.min_z { self.min_z = z }
        if z > self.max_z { self.max_z = z }
        if w < self.min_w { self.min_w = w }
        if w > self.max_w { self.max_w = w }
        // TODO: implement resize if min/max dim > n.
        let index = self.coord_to_index(x,y,z,w);
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
            min_x: self.min_x, max_x: self.max_x,
            min_y: self.min_y, max_y: self.max_y,
            min_z: self.min_z, max_z: self.max_z,
            min_w: self.min_w, max_w: self.max_w,
            // Since world is mirrored in z-axis, we need N/2 planes for z.
            bits: BitVector::new(n*n*n*n),
        };
        v.bits = self.bits.clone();
        for w in (self.min_w-1)..(self.max_w + 2) {
            for z in (self.min_z-1)..(self.max_z + 2) {
                for y in (self.min_y - 1)..(self.max_y + 2) {
                    for x in (self.min_x - 1)..(self.max_x + 2) {
                        let count = self.count_adjacent(x, y, z, w);
                        let active = self.getbit(x, y, z, w);
                        if active {
                            if (count < 2) || (count > 3) {
                                v.setbit(x, y, z, w, false);
                            }
                        } else {
                            // inactive
                            if count == 3 {
                                v.setbit(x, y, z, w, true);
                            }
                        }
                    }  // x
                }  // y
            }  // z
        }  // w
        return v;
    }
}  // impl HyperVoxels 

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
