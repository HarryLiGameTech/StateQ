use super::*;
use std::collections::HashSet;

type TestGraph<'a> = DecompositionGraph<&'a str, &'a str>;

macro_rules! graph {
    (@materials $graph:ident $($id:ident),*) => {
        $($graph.add_item(stringify!($id), true);)*
    };
    (@non_materials $graph:ident $($id:ident),*) => {
        $($graph.add_item(stringify!($id), false);)*
    };
    (@recipes $graph:ident $($name:ident = $($materials:ident),* => $target:ident : $cost:literal);*) => {
        $($graph.add_recipe(
            &stringify!($target), HashSet::from([$(stringify!($materials)),*]),
            $cost, Box::new(|_| vec![stringify!($name)]),
        );)*
    };
    {
        materials = $($mat_id:ident),*;
        items = $($non_mat_id:ident),*;
        $($name:ident = [$($materials:ident),*] => $target:ident : $cost:literal);*;
    } => {
        {
            let mut graph: TestGraph = DecompositionGraph::new();
            graph!(@materials graph $($mat_id),*);
            graph!(@non_materials graph $($non_mat_id),*);
            graph!(@recipes graph $($name = $($materials),* => $target : $cost);*);
            graph
        }
    }
}

fn assert_decomposable(graph: &mut TestGraph, item: &str) {
    if !graph.is_decomposable(&item) {
        panic!("[assert_decomposable] Assertion failed: item `{}` is not decomposable", item);
    }
}

fn assert_not_decomposable(graph: &mut TestGraph, item: &str) {
    if graph.is_decomposable(&item) {
        panic!("[assert_not_decomposable] Assertion failed: item `{}` is decomposable", item);
    }
}

fn assert_decomposition<'a>(graph: &mut TestGraph<'a>, item: &'a str, decomposition: &'a str) {
    assert_eq!(graph.execute_decomposition(&item, &"").unwrap()[0], decomposition);
}

macro_rules! assert_decomposable {
    ($graph:ident => $($id:ident),*) => {
        $(assert_decomposable(&mut $graph, stringify!($id));)*
    };
}

macro_rules! assert_not_decomposable {
    ($graph:ident => $($id:ident),*) => {
        $(assert_not_decomposable(&mut $graph, stringify!($id));)*
    };
}

macro_rules! assert_decompositions {
    ($graph:ident => $($id:ident: $decomposition:ident),*) => {
        $(assert_decomposition(&mut $graph, stringify!($id), stringify!($decomposition));)*
    };
}

#[test]
fn test_decomposition_simple_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C;
        a = [A, B] => C : 10;
    };
    assert_decomposable!(graph => C);
    assert_not_decomposable!(graph => A, B);
    assert_decompositions!(graph => C: a);
}

#[test]
fn test_decomposition_lowest_cost() {
    let mut graph = graph! {
        materials = A, B, C;
        items = D;
        a = [A, B] => D : 2;
        b = [B, C] => D : 1;
        c = [C, A] => D : 0;
    };
    assert_decomposable!(graph => D);
    assert_not_decomposable!(graph => A, B, C);
    assert_decompositions!(graph => D: c);
}

#[test]
fn test_decomposition_simple_one_to_one_case() {
    let mut graph = graph! {
        materials = A;
        items = B;
        a = [A] => B : 2;
    };
    assert_decomposable!(graph => B);
    assert_not_decomposable!(graph => A);
    assert_decompositions!(graph => B: a);
}

#[test]
fn test_decomposition_one_to_one_with_not_decomposable_item_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D;
        a = [A] => C : 1;
        b = [B] => C : 2;
    };
    assert_decomposable!(graph => C);
    assert_not_decomposable!(graph => A, B, D);
    assert_decompositions!(graph => C: a);
}

#[test]
fn test_decomposition_invalid_one_to_one_case() {
    let mut graph = graph! {
        materials = A;
        items = B;
        a = [B] => B : 12;
    };
    assert_not_decomposable!(graph => A, B);
}

#[test]
fn test_decomposition_one_to_one_with_invalid_decomposition_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D;
        a = [A] => C : 1;
        b = [D] => C : 2;
    };
    assert_not_decomposable!(graph => A, B, D);
    assert_decomposable!(graph => C);
    assert_decompositions!(graph => C: a);
}

#[test]
fn test_decomposition_simple_loop_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C;
        a = [A, B] => C : 5;
        b = [B, C] => A : 3;
    };
    assert_decomposable!(graph => C);
    assert_not_decomposable!(graph => A, B);
    assert_decompositions!(graph => C: a);
}

#[test]
fn test_decomposition_complicate_loop_case() {
    let mut graph = graph! {
        materials = A, B, C, D;
        items = E, F;
        a = [A, B] => E : 8;
        b = [B, C] => F : 10;
        c = [E, F] => B : 9;
        d = [D, F] => B : 7;
    };
    assert_decomposable!(graph => E, F);
    assert_not_decomposable!(graph => A, B, C, D);
    assert_decompositions!(graph => E: a);
    assert_decompositions!(graph => F: b);
}

#[test]
fn test_decomposition_loop_with_not_decomposable_item_case() {
    let mut graph = graph! {
        materials = A, B, C;
        items = D, E;
        a = [A, B] => E : 6;
        b = [B, C] => E : 8;
        c = [B, E] => A : 9;
    };
    assert_decomposable!(graph => E);
    assert_not_decomposable!(graph => A, B, C, D);
    assert_decompositions!(graph => E: a);
}

#[test]
fn test_decomposition_loop_with_invalid_decomposition_case() {
    let mut graph = graph! {
        materials = A, B, C;
        items = D, E;
        a = [A, B] => E : 6;
        b = [B, C] => E : 5;
        c = [B, D] => A : 3;
    };
    assert_not_decomposable!(graph => A, B, C, D);
    assert_decomposable!(graph => E);
    assert_decompositions!(graph => E: b);
}

#[test]
fn test_decomposition_loop_with_strange_decomposition_case() {
    let mut graph = graph! {
        materials = A, B, C;
        items = D, E;
        a = [A, B] => E : 6;
        b = [B, E] => D : 6;
        c = [B, E] => A : 3;
    };
    assert_decomposable!(graph => D, E);
    assert_not_decomposable!(graph => A, B, C);
    assert_decompositions!(graph => D: b);
    assert_decompositions!(graph => E: a);
}

#[test]
fn test_decomposition_material_to_item_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D;
        a = [A, B] => C : 5;
        b = [B, C] => D : 3;
        c = [A, C] => D : 12;
    };
    assert_decomposable!(graph => C, D);
    assert_not_decomposable!(graph => A, B);
    assert_decompositions!(graph => C: a);
    assert_decompositions!(graph => D: b);
}

#[test]
fn test_decomposition_material_to_item_with_not_decomposable_item_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D;
        a = [A, B] => C : 3;
    };
    assert_decomposable!(graph => C);
    assert_not_decomposable!(graph => A, B, D);
    assert_decompositions!(graph => C: a);
}

#[test]
fn test_decomposition_material_to_item_with_invalid_decomposition_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D;
        a = [A, B] => C : 3;
        b = [B, D] => C : 10;
    };
    assert_not_decomposable!(graph => A, B, D);
    assert_decomposable!(graph => C);
    assert_decompositions!(graph => C: a);
}

#[test]
fn test_decomposition_material_to_item_strange_case() {
    let mut graph = graph! {
        materials = A, B, C;
        items = D, E;
        a = [A, B] => D : 5;
        b = [A, B] => E : 3;
        c = [A, D] => E : 12;
    };
    assert_decomposable!(graph => D, E);
    assert_not_decomposable!(graph => A, B, C);
    assert_decompositions!(graph => D: a);
    assert_decompositions!(graph => E: b);
}

#[test]
fn test_decomposition_contain_item_to_item_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D, E;
        a = [A, B] => C : 5;
        b = [B, C] => D : 18;
        c = [C, D] => E : 10;
    };
    assert_decomposable!(graph => C, D, E);
    assert_not_decomposable!(graph => A, B);
    assert_decompositions!(graph => C: a);
    assert_decompositions!(graph => D: b);
    assert_decompositions!(graph => E: c);
}

#[test]
fn test_decomposition_contain_item_to_item_with_not_decomposable_item_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D, E, F;
        a = [A, B] => D : 3;
        b = [A, B] => E : 4;
        c = [D, E] => C : 5;
    };
    assert_decomposable!(graph => C, D, E);
    assert_not_decomposable!(graph => A, B, F);
    assert_decompositions!(graph => C: c);
    assert_decompositions!(graph => D: a);
    assert_decompositions!(graph => E: b);
}

#[test]
fn test_decomposition_contain_item_to_item_invalid_decomposition_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D, E;
        a = [A, B] => C : 3;
        b = [B, D] => E : 18;
        c = [C, E] => D : 10;
    };
    assert_not_decomposable!(graph => A, B, D, E);
    assert_decomposable!(graph => C);
    assert_decompositions!(graph => C: a);
}

#[test]
fn test_decomposition_contain_item_to_item_strange_case() {
    let mut graph = graph! {
        materials = A, B;
        items = C, D, E;
        a = [A, B] => C : 15;
        b = [A, B] => D : 18;
        c = [D, E] => C : 12;
        d = [A, C] => E : 11;
    };
    assert_decomposable!(graph => C, D, E);
    assert_not_decomposable!(graph => A, B);
    assert_decompositions!(graph => D: b);
    assert_decompositions!(graph => C: a);
    assert_decompositions!(graph => E: d);
}
