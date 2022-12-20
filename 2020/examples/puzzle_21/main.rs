use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

#[derive(Clone, Debug, Display, FromStr)]
#[display(r"{ingredients} (contains {allergens})")]
struct RecipeRaw {
    ingredients: String,
    allergens: String,
}

#[derive(Clone, Debug)]
struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl From<RecipeRaw> for Recipe {
    fn from(r: RecipeRaw) -> Recipe {
        Recipe {
            ingredients: HashSet::from_iter(
                r.ingredients
                    .split_whitespace()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>(),
            ),
            allergens: HashSet::from_iter(
                r.allergens
                    .split(", ")
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>(),
            ),
        }
    }
}

fn find_allergen_mapping(contents: &str) -> (Vec<Recipe>, HashMap<String, String>) {
    let mut recipes: Vec<Recipe> = Vec::new();
    let mut allergens: HashSet<String> = HashSet::new();
    let mut ingredients: HashSet<String> = HashSet::new();
    for line in contents.lines() {
        let recipe_raw: RecipeRaw = line.parse().unwrap();
        let recipe = Recipe::from(recipe_raw);
        ingredients.extend(recipe.ingredients.clone());
        allergens.extend(recipe.allergens.clone());
        recipes.push(recipe);
    }

    let mut ingredient_to_allergen: HashMap<String, String> = HashMap::new();
    let mut allergens_to_assign: VecDeque<String> = allergens.iter().cloned().collect();

    while let Some(allergen) = allergens_to_assign.pop_front() {
        let mut relevant_ingredients = recipes
            .iter()
            .filter(|x| x.allergens.contains(&allergen))
            .map(|x| x.ingredients.clone())
            .collect::<Vec<HashSet<String>>>();

        let (intersection, others) = relevant_ingredients.split_at_mut(1);
        let intersection = &mut intersection[0];
        intersection.retain(|e| !ingredient_to_allergen.contains_key(e));
        for other in others {
            intersection.retain(|e| other.contains(e) && !ingredient_to_allergen.contains_key(e));
        }
        if intersection.len() == 1 {
            let ingredient = intersection.drain().next().unwrap();
            ingredient_to_allergen.insert(ingredient, allergen);
        } else {
            allergens_to_assign.push_back(allergen);
        }
    }
    (recipes, ingredient_to_allergen)
}

fn part_1(contents: &str) -> u64 {
    let (recipes, ingredient_to_allergen) = find_allergen_mapping(contents);
    let mut answer = 0;
    for recipe in recipes {
        for ingredient in recipe.ingredients {
            if !ingredient_to_allergen.contains_key(&ingredient) {
                answer += 1;
            }
        }
    }
    answer
}

fn part_2(contents: &str) -> String {
    let (recipes, ingredient_to_allergen) = find_allergen_mapping(contents);

    let mut pairs = ingredient_to_allergen
        .into_iter()
        .collect::<Vec<(String, String)>>();
    // sort by allergen alphabetically
    pairs.sort_by(|x, y| x.1.cmp(&y.1));
    // comma-join ingredients
    pairs
        .into_iter()
        .map(|x| x.0)
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(include_str!("./example.txt")), 5);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(include_str!("./example.txt")), 1);
    }
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    let part_1_answer = part_1(contents);
    println!("Answer for part 1 is: {}", part_1_answer);
    let part_2_answer = part_2(contents);
    println!("Answer for part 2 is: {}", part_2_answer);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
