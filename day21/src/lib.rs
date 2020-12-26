#![allow(dead_code)]

use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

#[derive(Debug,Default)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    fn new() -> Food {
        Default::default()
    }

    fn parse(&mut self, line: &str) {
        let parts = line.split(" (contains ").take(2).collect::<Vec<&str>>();
        self.ingredients.extend(parts[0].split_whitespace().map(|x| x.to_string()));
        let mut allergens: &str = parts[1];
        allergens = &allergens[0..allergens.len()-1];
        self.allergens.extend(allergens.split(", ").map(|x| x.to_string()));
    }
}

#[derive(Default)]
struct Foods {
    foods: Vec<Food>,
    all_allergens: HashSet<String>,
    all_ingredients: HashSet<String>,
    ingredient_count: HashMap<String, usize>,  // key is ingredient, value is counter
    ingredients_map: HashMap<String, String>,  // key: ingredient, value: allergen
    allergens_map: HashMap<String, String>,  // key: allergen, value: ingredient
}

impl Foods {
    fn new() -> Foods { Default::default() }

    fn parse(&mut self, text: &str) {
        self.foods = text.lines()
            .map(|line| {
                let mut food = Food::new();
                food.parse(line);
                food })
            .collect();
        for food in self.foods.iter() {
            dbg!(food);
            self.all_allergens = self.all_allergens.union(&food.allergens).map(|x| x.clone()).collect();
            self.all_ingredients = self.all_ingredients.union(&food.ingredients).map(|x| x.clone()).collect();
            for ingredient in food.ingredients.iter() {
                let mut count: usize = 0;
                let value = self.ingredient_count.get(ingredient);
                match value {
                    Some(x) => count = *x,
                    _ => (),
                };
                count += 1;
                self.ingredient_count.insert(ingredient.clone(), count);
            }
        }
        dbg!(&self.all_allergens);
        dbg!(&self.all_ingredients);
    }

    // Returns true if a new mapping was added.
    fn deduce_ingredients_map(&mut self) -> bool {
        let mut did_map: bool = false;
        for allergen in self.all_allergens.iter() {
            let mut possible_ingredients = self.all_ingredients.clone();
            // Remove known ingredients:
            let known_ingredients: HashSet<String> = self.ingredients_map.keys()
                .map(|x| x.clone()).collect();
            possible_ingredients = possible_ingredients.difference(&known_ingredients).map(|x| x.clone()).collect();
            // Find the minimum set of foods that might contain this allergen:
            for food in self.foods.iter().filter(|f| f.allergens.contains(allergen)) {
                possible_ingredients = possible_ingredients.intersection(&food.ingredients)
                    .map(|x| x.clone()).collect();
            }
            dbg!(&possible_ingredients);
            // If only one ingredient could contain the allergen, add it to the map:
            if possible_ingredients.len() == 1 {
                let ingredients = possible_ingredients.iter().map(|x| x.clone()).collect::<Vec<String>>();
                self.ingredients_map.insert(ingredients[0].clone(), allergen.clone());
                self.allergens_map.insert(allergen.clone(), ingredients[0].clone());
                println!("mapped ingredient {} to allergen {}", ingredients[0], allergen);
                did_map = true;
            }
        }
        return did_map;
    }

    fn deduce_all_ingredients(&mut self) {
        while self.deduce_ingredients_map() {}
    }
}

pub fn part1() -> usize {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    println!("Loaded {} bytes", contents.len());
    let mut foods = Foods::new();
    foods.parse(&contents);
    foods.deduce_all_ingredients();
    let known_ingredients: HashSet<String> = foods.ingredients_map.keys()
        .map(|x| x.clone()).collect();
    let unknown_ingredients: HashSet<String> = foods.all_ingredients.difference(&known_ingredients)
        .map(|x| x.clone()).collect();
    dbg!(unknown_ingredients.len());
    let result: usize = unknown_ingredients.iter()
        .map(|x| foods.ingredient_count[x])
        .sum();
    dbg!(result);

    let mut sorted_allergens: Vec<String> = foods.allergens_map.keys()
        .map(|x| x.clone()).collect();
    sorted_allergens.sort();
    for allergen in sorted_allergens.iter() {
        print!("{},", foods.allergens_map[allergen]);
    }
    println!("");
    /*
    let part2: String =
        sorted_allergens.iter()
          .map(|x| foods.allergens_map[x].as_str())
          .join(",");
    */
    dbg!(result);
    return result;
}


#[test]
pub fn test_foods() {
    let contents = fs::read_to_string("testcase.txt")
        .expect("Something went wrong reading the file");
    println!("Loaded {} bytes", contents.len());
    let mut foods = Foods::new();
    foods.parse(&contents);
    foods.deduce_all_ingredients();
    assert_eq!(foods.ingredients_map["fvjkl"], "soy");
}

