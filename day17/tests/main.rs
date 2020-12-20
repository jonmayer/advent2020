#[cfg(test)]
mod tests {

use day17::Voxels;

#[test]
pub fn test() {
    let mut voxels = Voxels::new(128);
    voxels.initialize(".#.\n..#\n###");
    voxels.print();
    dbg!(voxels.count_adjacent(0, 0, 0));
    for y in -1..2 {
        println!("{}{}{}", voxels.count_bit(-1, y, 0), voxels.count_bit(0, y, 0), voxels.count_bit(1, y, 0));
    }
    for y in -2..3 {
        println!("{}{}{}{}{}",
            voxels.count_adjacent(-2, y, 0),
            voxels.count_adjacent(-1, y, 0), voxels.count_adjacent(0, y, 0), voxels.count_adjacent(1, y, 0),
            voxels.count_adjacent(2, y, 0));
    }
    for i in 0..6 {
        println!("=== step {} ===", i);
        voxels = voxels.update();
        voxels.print();
        dbg!(voxels.count_all_ones());
    }
    assert_eq!(112, voxels.count_all_ones());
}

}  // mod tests
