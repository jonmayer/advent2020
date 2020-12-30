#[cfg(test)]
mod tests {

    use day20::*;
    use std::fs;

    #[test]
    pub fn test_testcase() {
        let contents =
            fs::read_to_string("testcase.txt").expect("Something went wrong reading the file");
        let mut ts = day20::TileSet::new();
        ts.parse(&contents);
        assert_eq!(ts.n, 3);
        ts.connect();
        ts.layout(1);
        // Layout should be:
        // 1951    2311    3079
        // 2729    1427    2473
        // 2971    1489    1171
        assert_eq!(ts.get_tile_id_from_coord(0, 0), 1951);
        assert_eq!(ts.get_tile_id_from_coord(1, 0), 2729);
        assert_eq!(ts.get_tile_id_from_coord(2, 0), 2971);

        assert_eq!(ts.get_tile_id_from_coord(0, 1), 2311);
        assert_eq!(ts.get_tile_id_from_coord(1, 1), 1427);
        assert_eq!(ts.get_tile_id_from_coord(2, 1), 1489);

        assert_eq!(ts.get_tile_id_from_coord(0, 2), 3079);
        assert_eq!(ts.get_tile_id_from_coord(1, 2), 2473);
        assert_eq!(ts.get_tile_id_from_coord(2, 2), 1171);
        let mut img = ts.draw_image();
        img = img.transpose();
        let monsters = img.find_most_monsters(&SEAMONSTER);
        assert_eq!(2, monsters);
        img.dump();
        let monsters = img.find_most_monsters(&SEAMONSTER);
        assert_eq!(2, monsters);
        dbg!(monsters);
    }
} // mod tests
