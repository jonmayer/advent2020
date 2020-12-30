#[cfg(test)]
mod tests {

    use day17::Voxels;

    use day17::bitvector::BitVector;

    #[test]
    pub fn test_bitvector_iterator() {
        let mut bv = BitVector::new(1024);
        bv.set(100, true);
        bv.set(800, true);
        bv.set(803, true);
        bv.set(801, true);
        bv.set(102, true);
        bv.set(102, false);
        bv.set(3, true);
        bv.set(31, true);
        bv.set(40, true);
        let mut it = bv.iter();
        assert_eq!(it.next(), Option::Some(3));
        assert_eq!(it.next(), Option::Some(31));
        assert_eq!(it.next(), Option::Some(40));
        assert_eq!(it.next(), Option::Some(100));
        assert_eq!(it.next(), Option::Some(800));
        assert_eq!(it.next(), Option::Some(801));
        assert_eq!(it.next(), Option::Some(803));
        assert_eq!(it.next(), Option::None);
    }

    #[test]
    pub fn test() {
        let mut voxels = Voxels::new(128);
        voxels.initialize(".#.\n..#\n###");
        voxels.print();
        dbg!(voxels.count_adjacent(0, 0, 0));
        for y in -1..2 {
            println!(
                "{}{}{}",
                voxels.count_bit(-1, y, 0),
                voxels.count_bit(0, y, 0),
                voxels.count_bit(1, y, 0)
            );
        }
        for y in -2..3 {
            println!(
                "{}{}{}{}{}",
                voxels.count_adjacent(-2, y, 0),
                voxels.count_adjacent(-1, y, 0),
                voxels.count_adjacent(0, y, 0),
                voxels.count_adjacent(1, y, 0),
                voxels.count_adjacent(2, y, 0)
            );
        }
        for i in 0..6 {
            println!("=== step {} ===", i);
            voxels = voxels.update();
            voxels.print();
            dbg!(voxels.count_all_ones());
        }
        assert_eq!(112, voxels.count_all_ones());
    }
} // mod tests
