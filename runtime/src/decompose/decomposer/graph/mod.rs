use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::raise_error;

#[cfg(test)]
mod tests;

type RecipeRef<T> = Arc<Mutex<Recipe<T>>>;
type ItemRef<T> = Arc<Mutex<Item<T>>>;

pub type Delegate<T> = dyn Fn(&T) -> Vec<T> + Send + 'static;

struct FeasibleRecipe<T> {
    pub recipe: RecipeRef<T>,
    pub total_cost: i32,
}

impl<T> FeasibleRecipe<T> {
    pub fn new(recipe: RecipeRef<T>, total_cost: i32) -> Self {
        Self { recipe, total_cost }
    }
}

pub struct Item<T> {
    candidate_recipes: Vec<RecipeRef<T>>,
    is_material: bool,
    is_decomposable: bool,
    is_decomposing: bool,
    recipe: Option<FeasibleRecipe<T>>,
}

impl<T> Item<T> {
    pub fn new(is_material: bool) -> Self {
        Self {
            candidate_recipes: vec![],
            is_material,
            is_decomposable: true,
            is_decomposing: false,
            recipe: None
        }
    }
}

pub struct Recipe<T> {
    cost: i32,
    materials: Vec<ItemRef<T>>,
    delegate: Box<Delegate<T>>,
    type_phantom: PhantomData<T>,
}

impl<T> Recipe<T> {
    pub fn new(materials: Vec<ItemRef<T>>, cost: i32, delegate: Box<Delegate<T>>) -> Self {
        Self { cost, materials, delegate, type_phantom: PhantomData }
    }
}

pub struct DecompositionGraph<I: Eq + Hash + Ord + Display, T> {
    items: HashMap<I, ItemRef<T>>,
}

impl<I: Eq + Hash + Ord + Display, T> DecompositionGraph<I, T> {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    pub fn add_item(&mut self, id: I, is_material: bool) {
        self.items.insert(id, ItemRef::new(Item::new(is_material).into()));
    }

    pub fn add_recipe(
        &mut self, target_id: &I, materials_id: HashSet<I>, cost: i32, delegate: Box<Delegate<T>>
    ) {
        let materials: Vec<ItemRef<T>> = materials_id.iter().map(|id| {
            self.items.get(id).cloned().unwrap_or_else(|| raise_error!("Invalid item id: {}", id))
        }).collect();
        let recipe = RecipeRef::new(Recipe::new(materials, cost, delegate).into());
        self.items[target_id].lock().unwrap_or_else(|_| {
            raise_error!("Invalid target id: {}", target_id)
        }).candidate_recipes.push(recipe);
    }

    pub fn is_available(&self, id: &I) -> bool {
        self.items.get(id).and_then(|item| {
            if item.lock().unwrap().is_material { None } else { Some(()) }
        }).is_some()
    }

    pub fn is_decomposable(&self, id: &I) -> bool {
        let mut item = self.items[id].lock().unwrap();
        item.search_recipe();
        item.recipe.is_some()
    }

    pub fn execute_decomposition(&mut self, id: &I, value: &T) -> Option<Vec<T>> {
        let mut item = self.items[id].lock().unwrap();
        item.search_recipe();
        item.recipe.as_ref().map(|recipe| {
            let delegate = &*recipe.recipe.lock().unwrap().delegate;
            delegate(value)
        })
    }
}

impl<T> Item<T> {
    pub fn search_recipe(&mut self) -> Option<i32> {
        // base cases
        if self.is_material {
            self.is_decomposable = false;
            return Some(0);
        } else if !self.is_decomposable {
            return None;
        } else if self.is_decomposing {
            // if the search path forms a circle,
            // then we consider the item not decomposable
            self.is_decomposable = false;
            return None;
        } else if let Some(recipe) = &self.recipe {
            return Some(recipe.total_cost);
        }

        // set decomposing flag
        self.is_decomposing = true;

        // sort candidate recipes
        self.candidate_recipes.sort_by_key(|recipe| recipe.lock().unwrap().cost);

        // search recipe
        for recipe in &mut self.candidate_recipes {
            let mut recipe_mut = recipe.lock().unwrap();
            let cost = recipe_mut.materials.iter_mut().map(|material| {
                match material.try_lock() {
                    Ok(mut item) => item.search_recipe(),
                    Err(_) => None,
                }
            }).fold(Some(0), |acc, decompose| {
                acc.and_then(|acc| decompose.map(|val| val + acc))
            });
            if let Some(cost) = cost {
                self.recipe = Some(FeasibleRecipe::new(RecipeRef::clone(recipe), cost));
                self.is_decomposing = false;
                return Some(cost);
            }
        }

        // remove decomposing flag
        self.is_decomposing = false;

        // the item is not decomposable
        self.is_decomposable = false;
        None /* No valid recipe found */
    }
}
