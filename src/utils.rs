#![allow(dead_code)]

use na::{Point2, Scalar};
use num_traits::bounds::LowerBounded;
use num_traits::Signed;
use rustc_hash::FxHashSet;
use std::hash::Hash;

pub fn manhattan_distance<N: Scalar + Signed>(left: &Point2<N>, right: &Point2<N>) -> N {
    (left.x.clone() - right.x.clone()).abs() + (left.y.clone() - right.y.clone()).abs()
}

/// Performs a depth first search on the input graph.
/// Returns the first leaf node with the highest score found.
///
/// This function assumes a graph without circles.
///
/// T: Node type
/// FN: Successor
/// IN: IntoIterator over successors
/// SF: Score Function
/// BSF: Best possible score function
/// SC: Score
pub fn dfs<
    N: Clone + Eq + Hash,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    SF: FnMut(&N) -> SC,
    BSF: FnMut(&N) -> SC,
    SC: Ord + LowerBounded,
    F: FnMut(&N) -> bool,
>(
    start: N,
    mut successors: FN,
    mut score: SF,
    mut best_possible_score: BSF,
    mut is_final: F,
) -> SC {
    let mut visited = FxHashSet::default();
    let mut stack = Vec::new();
    stack.push(start);

    let mut best_score = SC::min_value();
    // let mut best_path = None;

    while let Some(node) = stack.pop() {
        if best_possible_score(&node) <= best_score {
            continue;
        }

        if !visited.insert(node.clone()) {
            continue;
        }

        if is_final(&node) {
            // This is a leaf node
            let score_value = score(&node);
            if score_value > best_score {
                best_score = score_value;

                // best_path = Some(path);
            }
            continue;
        }

        stack.extend(successors(&node));
    }

    best_score
}
