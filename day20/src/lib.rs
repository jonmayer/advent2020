#![allow(dead_code)]
use num_integer::Roots;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use transpose;

// Ordinal directions are enumerated in clockwise order from N.
// Should this be an enum?
const N: usize = 0;
const E: usize = 1;
const S: usize = 2;
const W: usize = 3;
const EDGES: [usize; 4] = [N, E, S, W];

// maps a direction enum to a human-readable character.
fn dir_to_name(dir: usize) -> char {
    match dir & 0b11 {
        N => 'N',
        E => 'E',
        S => 'S',
        W => 'W',
        _ => panic!("Invalid direction value: {}", dir),
    }
}

// rotate direction 180 degrees.
fn opposite_dir(dir: usize) -> usize {
    return (dir + 2) % 4;
}

// bit reverse a 10 bit edge fingerprint.
fn flip_edge(edge: u16) -> u16 {
    return (0..=9)
        .map(|x| {
            if (edge & (1 << x)) != 0 {
                1 << (9 - x)
            } else {
                0
            }
        })
        .sum();
}

// parse a datafile bitmap line into a packed u8 integer.
fn line_to_bitmap(line: &str) -> u8 {
    let mut bitmap = 0u8;
    let chars: Vec<char> = line.chars().collect();
    for i in 0..8 {
        if chars[8 - i] == '#' {
            bitmap |= 1 << i;
        }
    }
    return bitmap;
}

// EdgeId designates what a tile edge connects to.
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct EdgeId {
    tile_id: u16,  // the other tile's tile_id
    dir: usize,    // the edge of the other tile we connect to
    flipped: bool, // whether we must flip the other tile over.
}

impl EdgeId {
    fn new(tile: &Tile, dir: usize, flipped: bool) -> EdgeId {
        EdgeId {
            tile_id: tile.tile_id, // tile that we connect to
            dir,                   // the edge of the tile being connected to.
            flipped,
        }
    }
}

impl fmt::Display for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Edge:{}:{}:{}",
            self.tile_id,
            dir_to_name(self.dir),
            if self.flipped { "T" } else { "f" }
        )
    }
}

impl fmt::Debug for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Edge:{}:{}:{}",
            self.tile_id,
            dir_to_name(self.dir),
            if self.flipped { "T" } else { "f" }
        )
    }
}

// Tile data:
#[derive(Copy, Clone, Debug, Default)]
struct Tile {
    tile_id: u16,    // what tile am i
    edges: [u16; 4], // fingerprint of each edge (NESW)
    bitmap: [u8; 8], // the 8x8 bitmap contents
    // The following are set by the connect() method:
    connections: [Option<EdgeId>; 4],
    // The following are set by the layout() method:
    placed: bool,
    flipped: bool,
    rotated: usize, // 90 degree steps, clockwise.
}

impl Tile {
    fn new() -> Tile {
        return Default::default();
    }

    fn parse(&mut self, text: &str) {
        let lines: Vec<&str> = text.lines().map(|x| x.trim()).collect();
        self.tile_id = lines[0][5..9].parse::<u16>().unwrap();
        // Tiles are 10x10:
        self.edges[N] = (0..=9)
            .map(|x| lines[1].chars().nth(x).unwrap()) // surely there is a better way?
            .map(|c| if c == '#' { 1 } else { 0 })
            .enumerate()
            .map(|(i, v)| v << i)
            .sum();
        self.edges[E] = (0..=9)
            .map(|x| lines[1 + x].chars().nth(9).unwrap()) // surely there is a better way?
            .map(|c| if c == '#' { 1 } else { 0 })
            .enumerate()
            .map(|(i, v)| v << i)
            .sum();
        self.edges[S] = (0..=9)
            .map(|x| lines[10].chars().nth(9 - x).unwrap()) // surely there is a better way?
            .map(|c| if c == '#' { 1 } else { 0 })
            .enumerate()
            .map(|(i, v)| v << i)
            .sum();
        self.edges[W] = (0..=9)
            .map(|x| lines[10 - x].chars().nth(0).unwrap()) // surely there is a better way?
            .map(|c| if c == '#' { 1 } else { 0 })
            .enumerate()
            .map(|(i, v)| v << i)
            .sum();
        for index in 0..8 {
            self.bitmap[index] = line_to_bitmap(lines[2 + index]);
        }
    }

    fn is_corner(&self) -> bool {
        let count = self.connections.iter().filter(|x| x.is_none()).count();
        return count == 2;
    }

    fn rotate(&mut self) {
        // When the tile is placed, it is rotated one more step clockwise.
        self.rotated = (self.rotated + 1) % 4;
    }

    fn flip(&mut self) {
        // Flip the tile over.
        self.flipped = !self.flipped;
    }

    // Maps a TileSet (layout) direction to a Tile (internal) direction by
    // applying the flip and rotate transforms.
    fn transform_dir(&self, dir: usize) -> usize {
        let mut dir = dir;
        // rotation is clockwise.  So, rotation is 1 and we ask for the
        // Layout north edge, we should get the west internal edge.
        dir = (dir + 4 - self.rotated) % 4;
        // always rotate and then flip.  In the reverse order, then the
        // rotation direction reverses.
        if self.flipped {
            dir = match dir {
                E => W,
                W => E,
                N => N,
                S => S,
                _ => panic!("bad dir: {}", dir),
            }
        }
        return dir;
    }

    fn get_xformed_edge(&self, dir: usize) -> u16 {
        let xform_dir = self.transform_dir(dir);
        let mut edge = self.edges[xform_dir];
        if self.flipped {
            edge = flip_edge(edge);
        }
        return edge;
    }

    fn get_xformed_connection(&self, dir: usize) -> Option<EdgeId> {
        let xform_dir = self.transform_dir(dir);
        return self.connections[xform_dir];
    }

    // gets bit from the bitmap.
    //   positive x is right.
    //   positive y is down.
    fn get_bit(&self, x: usize, y: usize) -> i8 {
        //    N                          N
        //   123                        321
        // W 456 E -> flipped over -> E 654 W
        //   789                        987
        //    S                          S
        //
        //    N                          N
        //   abc                        gda
        // W def E ->  rotate 1    -> E heb W
        //   ghi                        ifc
        //    S                          S
        //
        // When mapping from external coordinates to internal coordinates: if our
        // tile has rotate=1, we calculate ix,iy by rotating x,y counter-clockwise.
        // External coordinate (0, 0) maps to internal coordinate (0, 7).
        let mut ix: usize;
        let mut iy: usize;
        match self.rotated {
            0 => {
                ix = x;
                iy = y;
            }
            1 => {
                ix = y;
                iy = 7 - x;
            }
            2 => {
                ix = 7 - x;
                iy = 7 - y;
            }
            3 => {
                ix = 7 - y;
                iy = x;
            }
            _ => panic!("wat {}", self.rotated),
        }
        if self.flipped {
            ix = 7 - ix;
        }
        return if (self.bitmap[iy] & (1 << (7 - ix))) == 0 {
            0
        } else {
            1
        };
    }

    fn dump(&self) {
        println!(
            "tile:{} connections:N:{:?},E:{:?},S:{:?},W:{:?} flipped:{} rotated:{}",
            self.tile_id,
            self.connections[0],
            self.connections[1],
            self.connections[2],
            self.connections[3],
            self.flipped,
            self.rotated
        );
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tile_id.cmp(&other.tile_id)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Tile {}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.tile_id == other.tile_id
    }
}

// TileSet has it's own "north".  Each tile has a "fipped" and
// "rotated" attribute that says how the tile must be manipulated
// to line up with the internal direction.
#[derive(Clone, Debug, Default)]
pub struct TileSet {
    tilemap: HashMap<u16, Tile>,
    grid: Vec<u16>,
    pub n: u16, // width and height
}

impl TileSet {
    pub fn new() -> TileSet {
        return Default::default();
    }

    pub fn parse(&mut self, text: &str) {
        let mut count: u16 = 0;
        for tile_text in text.split("\n\n") {
            let mut tile = Tile::new();
            tile.parse(tile_text);
            self.tilemap.insert(tile.tile_id, tile);
            count += 1;
        }
        self.n = count.sqrt() as u16;
        assert_eq!(self.n * self.n, count); // fits square grid.
        dbg!(count, self.n);
        self.grid = vec![0; count as usize];
    }

    fn find_matching_edges(&self, edge_id: EdgeId) -> Vec<EdgeId> {
        // Really, find matching half-edge.  That is: the n:m edge of
        // this tile must match the m:n edge of another tile.
        let mut edges_matched: Vec<EdgeId> = Vec::new();
        let edge_value = self.tilemap[&edge_id.tile_id].edges[edge_id.dir];
        let edge_flipped = flip_edge(edge_value);

        for tile in self.tilemap.values() {
            // Don't match against ourselves:
            if tile.tile_id == edge_id.tile_id {
                continue;
            }
            for dir in EDGES.iter() {
                // Filter out already-connected edges
                if !tile.connections[*dir].is_none() {
                    continue;
                }
                // Does this edge match?
                if edge_value == tile.edges[*dir] {
                    // if the edge fingerprints are the same, then the
                    // next tile must be flipped over (so our n:m half edge
                    // key corresponds to the m:n of the other half edge).
                    edges_matched.push(EdgeId::new(tile, *dir, true));
                }
                // Does this edge match if flipped?
                if edge_flipped == tile.edges[*dir] {
                    // Since our n:m fingerprint is the same as the other
                    // tile's m:n fingerprint, we don't need to flip the
                    // other tile over.
                    edges_matched.push(EdgeId::new(tile, *dir, false));
                }
            }
        }
        return edges_matched;
    }

    fn connect_edges(&mut self, e1: EdgeId, e2: EdgeId) {
        assert_eq!(e1.flipped, e2.flipped); // flipped flags must match.
        self.tilemap.get_mut(&e1.tile_id).unwrap().connections[e1.dir] = Option::Some(e2);
        self.tilemap.get_mut(&e2.tile_id).unwrap().connections[e2.dir] = Option::Some(e1);
    }

    pub fn connect(&mut self) {
        let mut v: Vec<&Tile> = self.tilemap.values().collect();
        v.sort(); // or, v.sort_by_key(|x| x.tile_id);
        let connections: Vec<(EdgeId, Vec<EdgeId>)> = v
            .iter()
            .flat_map(|tile| EDGES.iter().map(move |dir| EdgeId::new(tile, *dir, false)))
            .map(|e| (e, self.find_matching_edges(e)))
            .collect();

        let unique_connections: Vec<(EdgeId, EdgeId)> = connections
            .iter()
            .filter(|(_, m)| m.len() == 1)
            .map(|(e, m)| (*e, m[0]))
            .collect();
        let expected_num_of_connections = (self.n * (self.n - 1) * 4) as usize;
        assert_eq!(unique_connections.len(), expected_num_of_connections);
        for mut edges in unique_connections {
            // if the connection from edges.0 to edges.1 requires a flipped connection,
            // then the connection from edges.1 to edges.0 must be the same.
            edges.0.flipped = edges.1.flipped;
            self.connect_edges(edges.0, edges.1);
        }
    }

    fn get_corners(&self) -> Vec<u16> {
        let mut corners: Vec<u16> = self
            .tilemap
            .values()
            .filter(|tile| tile.is_corner())
            .map(|tile| (*tile).tile_id)
            .collect();
        corners.sort();
        dbg!(&corners);
        return corners;
    }

    fn set_tile_id_at_coord(&mut self, x: usize, y: usize, tile_id: u16) {
        let index = x + y * self.n as usize;
        self.grid[index] = tile_id;
    }

    // get tile_id from coordinates
    pub fn get_tile_id_from_coord(&self, x: usize, y: usize) -> u16 {
        let index = x + y * self.n as usize;
        return self.grid[index];
    }

    fn place_tile(&mut self, x: usize, y: usize) {
        // the "adjacent" tile is the previously placed tile:
        let adj_y = if x == 0 { y - 1 } else { y };
        let adj_x = if x == 0 { x } else { x - 1 };
        // direction from the current tile to the previous tile:
        let dir_to_adj = if x == 0 { N } else { W };
        // direction from the previous tile to the current tile:
        let dir_from_adj = (dir_to_adj + 2) % 4;

        let adj_tile_id = self.get_tile_id_from_coord(adj_x, adj_y);
        let adj_tile = self.tilemap[&adj_tile_id];
        println!(
            "PLACING from tile {},{} ({}) to tile {},{} via {}",
            adj_x,
            adj_y,
            adj_tile_id,
            x,
            y,
            dir_to_name(dir_from_adj)
        );
        print!("  adj_tile: ");
        adj_tile.dump();
        println!(
            "  adj_tile {} maps to {}: con={:?}",
            dir_to_name(dir_from_adj),
            dir_to_name(adj_tile.transform_dir(dir_from_adj)),
            adj_tile.connections[adj_tile.transform_dir(dir_from_adj)],
        );

        // get the connection from the previous tile to the current tile:
        let conn = adj_tile.connections[adj_tile.transform_dir(dir_from_adj)].unwrap();
        // Place the tile indicated by that connection here:
        let tile_id = conn.tile_id;
        self.set_tile_id_at_coord(x, y, tile_id);

        // Correctly orient the current tile:
        let tile = &mut self.tilemap.get_mut(&tile_id).unwrap();
        assert!(!tile.placed, "Tile {} already placed.", tile_id);
        tile.placed = true;
        // Flip the this tile if either:
        //   the connection is not flipped, but the prev tile is.
        //   the connection is flipped, but the prev tile is not.
        if conn.flipped ^ adj_tile.flipped {
            tile.flip();
        }
        // Rotate the tile until edge the previous tile connected to is
        // facing the previous tile.
        while tile.transform_dir(dir_to_adj) != conn.dir {
            tile.rotate();
        }
        print!("  placed: ");
        tile.dump();
    }

    pub fn layout(&mut self, i: usize) {
        // Pick a corner and call it the top-left:
        let topleft = self.get_corners()[i];
        self.set_tile_id_at_coord(0, 0, topleft);
        // Rotate that corner until the 2 unconnected edges are top and left:
        let topleft_tile = &mut self.tilemap.get_mut(&topleft).unwrap();
        while !(topleft_tile.connections[topleft_tile.transform_dir(W)].is_none()
            && topleft_tile.connections[topleft_tile.transform_dir(N)].is_none())
        {
            (*topleft_tile).rotate();
        }

        // Layout all tiles relative to top-left.
        for y in 0..self.n as usize {
            println!("");
            for x in 0..self.n as usize {
                if (x == 0) && (y == 0) {
                    continue;
                }
                self.place_tile(x, y);
            }
        }

        // Check that all edges match.
        for y in 0..(self.n - 1) as usize {
            for x in 0..(self.n - 1) as usize {
                let tile = self.tilemap[&self.get_tile_id_from_coord(x, y)];
                let tile_e = self.tilemap[&self.get_tile_id_from_coord(x + 1, y)];
                let tile_s = self.tilemap[&self.get_tile_id_from_coord(x, y + 1)];
                assert_eq!(
                    tile.get_xformed_edge(E),
                    flip_edge(tile_e.get_xformed_edge(W))
                );
                assert_eq!(
                    tile.get_xformed_edge(S),
                    flip_edge(tile_s.get_xformed_edge(N))
                );
            }
        }
    }

    pub fn draw_image(&self) -> Image {
        let mut img = Image::new(self.n as usize * 8);
        for ty in 0..self.n as usize {
            for tx in 0..self.n as usize {
                let tile_id = self.get_tile_id_from_coord(tx, ty);
                let tile = &self.tilemap[&tile_id];
                for row in 0..8 {
                    for col in 0..8 {
                        let value = tile.get_bit(col, row);
                        img.draw_pixel(tx * 8 + col, ty * 8 + row, value);
                    }
                }
            }
        }
        return img;
    }
}

type Point = (isize, isize);

#[derive(Clone, Debug)]
pub struct Image {
    pixels: Vec<i8>,
    n: usize,
    sea_monsters: usize,
}

fn pixel_value_to_char(v: i8) -> char {
    match v {
        0 => '.', // off
        1 => '#', // on
        2 => 'X', // sea monster?
        _ => '?',
    }
}

// Sea Monster:
//   01234567890123456789
// 0      .    .    .  #
// 1 #    ##   .##  . ###
// 2  #  #. #  #  # .#
pub static SEAMONSTER: [Point; 15] = [
    (0, 1),
    (1, 2),
    (4, 2),
    (5, 1),
    (6, 1),
    (7, 2), // tail, 1st hump
    (10, 2),
    (11, 1),
    (12, 1),
    (13, 2), // 2nd hump
    (16, 2),
    (17, 1),
    (18, 1),
    (18, 0),
    (19, 1), // head
];

fn transform_shape(shape: &[Point], rot: isize, flip: bool) -> Vec<Point> {
    let mut xshape: Vec<Point> = shape.to_vec();
    for _ in 0..rot {
        xshape = xshape.iter().map(|(x, y)| (1 * y, -1 * x)).collect();
    }
    if flip {
        xshape = xshape.iter().map(|(x, y)| (1 * x, -1 * y)).collect();
    }
    let min_x = xshape.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = xshape.iter().map(|(_, y)| *y).min().unwrap();
    xshape = xshape
        .iter()
        .map(|(x, y)| (*x - min_x, *y - min_y))
        .collect();
    return xshape;
}

impl Image {
    pub fn new(n: usize) -> Image {
        Image {
            pixels: vec![0i8; n * n],
            n,
            sea_monsters: 0,
        }
    }

    fn coord(&self, x: isize, y: isize) -> usize {
        return self.n * y as usize + x as usize;
    }

    fn draw_pixel(&mut self, x: usize, y: usize, value: i8) {
        let index = self.coord(x as isize, y as isize);
        self.pixels[index] = value;
    }

    // Produce a transposed image:
    pub fn transpose(&self) -> Image {
        let mut img = Image::new(self.n);
        transpose::transpose(&self.pixels, &mut img.pixels, self.n, self.n);
        return img;
    }

    pub fn dump(&self) {
        for y in 0..self.n {
            for x in (0..self.n).step_by(8) {
                let index = x + self.n * y;
                let chrs: String = self.pixels[index..(index + 8)]
                    .iter()
                    .map(|x| pixel_value_to_char(*x) as char)
                    .collect();
                print!(" {}", chrs);
            }
            println!("");
            if (y % 8) == 7 {
                println!("");
            }
        }
    }

    pub fn count_shapes(&self, shape: &[Point]) -> usize {
        let mut count: usize = 0usize;
        let max_x: isize = shape.iter().map(|(x, _)| *x).max().unwrap();
        let max_y: isize = shape.iter().map(|(_, y)| *y).max().unwrap();
        for y in 0..(self.n as isize - max_y) {
            for x in 0..(self.n as isize - max_x) {
                count += shape
                    .iter()
                    .map(|(dx, dy)| self.pixels[self.coord(x + *dx, y + *dy)] as usize)
                    .product::<usize>();
            }
        }
        return count;
    }

    pub fn find_most_monsters(&self, shape: &[Point]) -> usize {
        let mut most = 0;
        for flip in &[false, true] {
            for rot in 0..=3 {
                let m = transform_shape(shape, rot, *flip);
                let count = self.count_shapes(m.as_slice());
                most = max(most, count);
            }
        }
        return most;
    }
}

pub fn part1(text: &str) -> u64 {
    let mut ts = TileSet::new();
    ts.parse(text);
    ts.connect();
    let part1 = ts.get_corners().iter().map(|x| *x as u64).product();
    return part1;
}

pub fn part2(text: &str) -> u64 {
    let mut ts = TileSet::new();
    ts.parse(text);
    ts.connect();
    ts.layout(0);
    let img = ts.draw_image();
    img.dump();

    let monsters = img.find_most_monsters(&SEAMONSTER);
    dbg!(monsters);

    let count_ones = img.pixels.iter().filter(|&x| *x == 1).count();
    let count_monster_pixels = monsters * SEAMONSTER.len();
    let count_noise_pixels = count_ones - count_monster_pixels;

    return count_noise_pixels as u64;
}
