#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Default)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    fn new() -> Food<'a> {
        Default::default()
    }

    fn parse(&mut self, line: &'a str) {
        let parts = line.split(" (contains ").take(2).collect::<Vec<&str>>();
        self.ingredients.extend(parts[0].split_whitespace());
        let mut allergens: &str = parts[1];
        allergens = &allergens[0..allergens.len() - 1];
        self.allergens.extend(allergens.split(", "));
    }
}

#[derive(Default)]
struct Foods<'a> {
    foods: Vec<Food<'a>>,
    all_allergens: HashSet<&'a str>,
    all_ingredients: HashSet<&'a str>,
    ingredient_count: HashMap<&'a str, usize>, // key is ingredient, value is counter
    ingredients_map: HashMap<&'a str, &'a str>, // key: ingredient, value: allergen
    allergens_map: HashMap<&'a str, &'a str>,  // key: allergen, value: ingredient
}

impl<'a> Foods<'a> {
    fn new() -> Foods<'a> {
        Default::default()
    }

    fn parse(&mut self, text: &'a str) {
        self.foods = text
            .lines()
            .map(|line| {
                let mut food = Food::new();
                food.parse(line);
                food
            })
            .collect();
        for food in self.foods.iter() {
            dbg!(food);
            self.all_allergens = self
                .all_allergens
                .union(&food.allergens)
                .map(|x| x.clone())
                .collect();
            self.all_ingredients = self
                .all_ingredients
                .union(&food.ingredients)
                .map(|x| x.clone())
                .collect();
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
            let known_ingredients: HashSet<&'a str> =
                self.ingredients_map.keys().map(|x| *x).collect();
            possible_ingredients = possible_ingredients
                .difference(&known_ingredients)
                .map(|x| *x)
                .collect();
            // Find the minimum set of foods that might contain this allergen:
            for food in self.foods.iter().filter(|f| f.allergens.contains(allergen)) {
                possible_ingredients = possible_ingredients
                    .intersection(&food.ingredients)
                    .map(|x| *x)
                    .collect();
            }
            dbg!(&possible_ingredients);
            // If only one ingredient could contain the allergen, add it to the map:
            if possible_ingredients.len() == 1 {
                let ingredients = possible_ingredients
                    .iter()
                    .map(|x| *x)
                    .collect::<Vec<&'a str>>();
                self.ingredients_map.insert(ingredients[0], allergen);
                self.allergens_map.insert(allergen, ingredients[0]);
                println!(
                    "mapped ingredient {} to allergen {}",
                    ingredients[0], allergen
                );
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
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Loaded {} bytes", contents.len());
    let mut foods = Foods::new();
    foods.parse(&contents);
    foods.deduce_all_ingredients();
    let known_ingredients: HashSet<&str> = foods.ingredients_map.keys().map(|x| *x).collect();
    let unknown_ingredients: HashSet<&str> = foods
        .all_ingredients
        .difference(&known_ingredients)
        .map(|x| *x)
        .collect();
    dbg!(unknown_ingredients.len());
    let result: usize = unknown_ingredients
        .iter()
        .map(|x| foods.ingredient_count[x])
        .sum();
    dbg!(result);

    let mut sorted_allergens: Vec<&str> = foods.allergens_map.keys().map(|x| *x).collect();
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
    let contents =
        fs::read_to_string("testcase.txt").expect("Something went wrong reading the file");
    println!("Loaded {} bytes", contents.len());
    let mut foods = Foods::new();
    foods.parse(&contents);
    foods.deduce_all_ingredients();
    assert_eq!(foods.ingredients_map["fvjkl"], "soy");
}
