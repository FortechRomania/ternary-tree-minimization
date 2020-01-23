use literal_value::LiteralValue;
use product_term::ProductTerm;
use std::collections::HashSet;
use ternary_node::TernaryNode;

/**
 * Struct that contains ternary tree minimization methods.
 * It provides a method to simplify boolean functions in DNF represented by its product terms.
 * e.g. Let's take the following boolean function into consideration: f(A,B) = ~A&B | A&B
 *      This function can be simplified to g(A,B) = B
 */
pub struct TernaryTreeMinimization {}

impl<'a> TernaryTreeMinimization {
  /**
   * Applies the TT-Min algorithm on the given terms
   * @param terms product terms to simplify
   * @param variable_order the variables present in the product terms
   * @return a set containing simplified product terms covering the initial product terms
   * @throws error if the given product terms cannot be simplified (there are less than 2 variables)
   */
  pub fn apply(
    terms: &HashSet<ProductTerm>,
    variable_order: &Vec<String>,
  ) -> Result<HashSet<ProductTerm>, String> {
    let number_of_vars = variable_order.len();
    let mut resulting_terms = HashSet::new();
    let mut var_order = Vec::new();
    for i in 0..variable_order.len() {
      if let Some(variable) = variable_order.get(i) {
        var_order.push(String::clone(variable));
      }
    }
    for term in terms {
      resulting_terms.insert(term.clone());
    }
    for _ in 0..number_of_vars + 1 {
      if let Ok(extracted_terms) =
        TernaryTreeMinimization::build_and_merge(&resulting_terms, &var_order)
      {
        resulting_terms = extracted_terms;
        var_order = TernaryTreeMinimization::rotate(&var_order);
      }
    }

    Ok(resulting_terms)
  }

  /**
   * Method that represents the rotation step in the algorithm.
   * In this case, using an array representation of only the last level of the tree,
   * we only need to perform a permutation on the variable order list
   * @param old_variable_order the order that was used for the previous step of the algorithm
   * @return a new vector containing the new order that should be used for the next step
   */
  fn rotate(old_variable_order: &Vec<String>) -> Vec<String> {
    let mut new_order = Vec::new();
    for i in 0..old_variable_order.len() {
      if let Some(variable) = old_variable_order.get(i) {
        new_order.push(String::clone(variable));
      }
    }
    let first_var = new_order.remove(0);
    new_order.push(first_var);
    new_order
  }

  /**
   * Methods that performs the build step of the algorithm and then the merge, yielding covering product terms
   * @param terms the initial product terms to be simplified
   * @param variable order the variables that appear in the product terms
   * @return a new set containing product terms that cover the initial product terms
   * @throws error if it makes no sense to build and merge (the trivial level of only one variable)
   */
  fn build_and_merge(
    terms: &HashSet<ProductTerm>,
    variable_order: &Vec<String>,
  ) -> Result<HashSet<ProductTerm>, String> {
    if variable_order.len() < 2 {
      return Err("Too few variables to build tree!".to_string());
    }
    let mut var_order_copy = Vec::new();
    for i in 0..variable_order.len() {
      if let Some(variable) = variable_order.get(i) {
        var_order_copy.push(String::clone(variable));
      }
    }

    let mut root = TernaryNode::new();
    root.set_variable(String::clone(variable_order.get(0).unwrap()));
    let leaves = TernaryTreeMinimization::build(root, &mut var_order_copy, terms);
    Ok(TernaryTreeMinimization::merge(
      &leaves,
      terms,
      var_order_copy.get(0).unwrap(),
    ))
  }

  /**
   * Method that performs the build step of the algorithm, building the ternary tree level by level.
   * @param root the root node of the ternary tree
   * @param variable order the variables that appear in the product terms
   * @param terms the initial product terms to be simplified
   * @return a vector containing the leaves of the ternary tree
   */
  fn build(
    root: TernaryNode,
    variable_order: &mut Vec<String>,
    terms: &HashSet<ProductTerm>,
  ) -> Vec<TernaryNode> {
    let mut nodes = vec![root];
    while variable_order.len() > 1 {
      let removed_var = variable_order.remove(0);
      let node_variable = variable_order.get(0).unwrap();
      let mut childs = Vec::new();
      for index in 0..nodes.len() {
        if let Some(current_node) = nodes.get(index) {
          let mut product_term = if let Some(node_term) = current_node.get_term() {
            node_term.clone()
          } else {
            ProductTerm::new()
          };
          let mut false_child = TernaryNode::new();
          let mut dont_care_child = TernaryNode::new();
          let mut true_child = TernaryNode::new();

          TernaryTreeMinimization::build_node(
            &mut false_child,
            &mut product_term,
            terms,
            LiteralValue::False,
            &removed_var,
            &node_variable,
            &mut childs,
          );
          TernaryTreeMinimization::build_node(
            &mut dont_care_child,
            &mut product_term,
            terms,
            LiteralValue::DontCare,
            &removed_var,
            &node_variable,
            &mut childs,
          );
          TernaryTreeMinimization::build_node(
            &mut true_child,
            &mut product_term,
            terms,
            LiteralValue::True,
            &removed_var,
            &node_variable,
            &mut childs,
          );
        }
      }
      nodes.clear();
      nodes.append(&mut childs);
    }

    nodes
  }

  /**
   * Method that performs the merge step of the algorithm, merging the term nodes of the tree
   * @param leaves vector containing the leaves of the ternary tree
   * @param initial_terms the initial product terms to be simplified
   * @param node_variable the last variable in the variable ordering
   * @return set containing product terms that are the result of the merge step
   */
  fn merge(
    leaves: &[TernaryNode],
    initial_terms: &HashSet<ProductTerm>,
    node_variable: &String,
  ) -> HashSet<ProductTerm> {
    let mut final_terms: HashSet<ProductTerm> = HashSet::new();
    for i in 0..leaves.len() {
      if let Some(current_node) = leaves.get(i) {
        let mut built_term = current_node.get_term().unwrap().clone();
        let false_node = TernaryTreeMinimization::build_term_node(
          initial_terms,
          &mut built_term,
          node_variable,
          LiteralValue::False,
        );
        let dont_care_node = TernaryTreeMinimization::build_term_node(
          initial_terms,
          &mut built_term,
          node_variable,
          LiteralValue::DontCare,
        );
        let true_node = TernaryTreeMinimization::build_term_node(
          initial_terms,
          &mut built_term,
          node_variable,
          LiteralValue::True,
        );

        if let Some(dont_care) = dont_care_node {
          final_terms.insert(dont_care.get_term().unwrap().clone());
        }

        if let Some(true_child) = true_node {
          if let Some(false_child) = false_node {
            let merged_term = false_child
              .get_term()
              .unwrap()
              .merge(true_child.get_term().unwrap());
            if let Ok(term) = merged_term {
              final_terms.insert(term.clone());
            } else {
              final_terms.insert(true_child.get_term().unwrap().clone());
              final_terms.insert(false_child.get_term().unwrap().clone());
            }
          } else {
            final_terms.insert(true_child.get_term().unwrap().clone());
          }
        } else if let Some(false_child) = false_node {
          final_terms.insert(false_child.get_term().unwrap().clone());
        }
      }
    }

    final_terms
  }

  /**
   * Builds a ternary node with the given information if its product term is a prefix of the given terms
   * @param node the current node to build
   * @param product_term the product term built until this node was reached
   * @param literal new literal to add to this node's product term
   * @param literal_variable variable of the new added literal
   * @param node_variable variable to attach to the node
   * @param node_collection collection to add the node to
   */
  fn build_node(
    node: &mut TernaryNode,
    product_term: &mut ProductTerm,
    terms: &HashSet<ProductTerm>,
    literal: LiteralValue,
    literal_variable: &String,
    node_variable: &String,
    node_collection: &mut Vec<TernaryNode>,
  ) {
    product_term.add_literal(String::clone(&literal_variable), literal);
    if product_term.is_prefix_of_any(terms) {
      node.set_variable(String::clone(node_variable));
      node.set_term(product_term.clone());
      let node_to_add = TernaryNode::clone(&node);
      node_collection.push(node_to_add);
    }

    product_term.remove_last();
  }

  /**
   * Builds a term node if it matches any of the given product terms
   * @param terms product terms to check if this term node equals any of them
   * @param built_term the product term until this node was reached
   * @param variable variable to be attached to the node
   * @param literal new literal to be added to the product term of this node
   * @return a new term node if it matches any of the given product terms; None otherwise
   */
  fn build_term_node(
    terms: &HashSet<ProductTerm>,
    built_term: &mut ProductTerm,
    variable: &String,
    literal: LiteralValue,
  ) -> Option<TernaryNode> {
    built_term.add_literal(String::clone(variable), literal);
    if built_term.matches_any(terms) {
      let node = TernaryNode::new_with_term(built_term.clone());
      built_term.remove_last();
      Some(node)
    } else {
      built_term.remove_last();
      None
    }
  }
}

#[cfg(test)]
pub mod ternary_tree_minimization_tests {
  use super::*;

  #[test]
  fn test_minimization_01() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::DontCare),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
    ]));
    if let Ok(actual_result) =
      TernaryTreeMinimization::apply(&set, &mut vec!["A".to_string(), "B".to_string()])
    {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 2);
    } else {
      panic!();
    }
  }

  #[test]
  fn test_minimization_02() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::DontCare),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::DontCare),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::DontCare),
      ("B".to_string(), LiteralValue::False),
    ]));
    if let Ok(actual_result) =
      TernaryTreeMinimization::apply(&set, &mut vec!["A".to_string(), "B".to_string()])
    {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 2);
    } else {
      panic!();
    }
  }

  #[test]
  fn test_minimization_03() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::DontCare),
    ]));
    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec!["A".to_string(), "B".to_string(), "C".to_string()],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 2);
    } else {
      panic!();
    }
  }

  #[test]
  fn test_minimization_04() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::DontCare),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::DontCare),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));
    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec!["A".to_string(), "B".to_string(), "C".to_string()],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 4);
    } else {
      panic!();
    }
  }

  #[test]
  pub fn test_minimization_05() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::DontCare),
      ("D".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::DontCare),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::False),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::DontCare),
      ("D".to_string(), LiteralValue::False),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::DontCare),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
    ]));
    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
      ],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 4);
    } else {
      panic!();
    }
  }

  #[test]
  pub fn test_minimization_06() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::False),
      ("E".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::True),
      ("E".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::False),
      ("E".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::DontCare),
      ("D".to_string(), LiteralValue::False),
      ("E".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
      ("E".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
      ("E".to_string(), LiteralValue::False),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::DontCare),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::False),
      ("E".to_string(), LiteralValue::False),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
      ("D".to_string(), LiteralValue::True),
      ("E".to_string(), LiteralValue::False),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::DontCare),
      ("D".to_string(), LiteralValue::False),
      ("E".to_string(), LiteralValue::False),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::DontCare),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
      ("E".to_string(), LiteralValue::False),
    ]));
    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
      ],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 4);
    } else {
      panic!();
    }
  }

  #[test]
  fn test_minimization_07() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::DontCare),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));
    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec!["A".to_string(), "B".to_string(), "C".to_string()],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 3);
    } else {
      panic!();
    }
  }

  #[test]
  fn test_minimization_08() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::DontCare),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::DontCare),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::False),
    ]));
    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec!["A".to_string(), "B".to_string(), "C".to_string()],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 2);
    } else {
      panic!();
    }
  }

  #[test]
  fn test_minimization_09() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::DontCare),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
    ]));
    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
      ],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 1);
    } else {
      panic!();
    }
  }

  #[test]
  fn test_minimization_10() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::DontCare),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::DontCare),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::DontCare),
    ]));

    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
      ],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result));
      }
      assert!(actual_result.len() == 1);
    } else {
      panic!();
    }
  }

  #[test]
  fn test_minimization_11() {
    let mut set = HashSet::new();
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::True),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::False),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::True),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::DontCare),
    ]));
    set.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::DontCare),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::DontCare),
    ]));

    let mut expected_result = HashSet::new();
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::DontCare),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::DontCare),
    ]));
    expected_result.insert(ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::False),
      ("B".to_string(), LiteralValue::DontCare),
      ("C".to_string(), LiteralValue::True),
      ("D".to_string(), LiteralValue::DontCare),
    ]));

    if let Ok(actual_result) = TernaryTreeMinimization::apply(
      &set,
      &mut vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
      ],
    ) {
      println!("terms after mini: {:#?}", actual_result);
      for expected_term in &expected_result {
        assert!(expected_term.matches_any(&actual_result)); // Tree does not rotate enough to minimize the last 2 terms.
      }
      assert!(actual_result.len() == 2);
    } else {
      panic!();
    }
  }
}
