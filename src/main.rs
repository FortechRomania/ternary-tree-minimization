extern crate linked_hash_map;

mod literal_value;
mod product_term;
mod ternary_node;
mod ternary_tree_minimization;
use std::collections::HashSet;

fn main() {
  let mut term = product_term::ProductTerm::new();
  term.add_literal(String::from("A"), literal_value::LiteralValue::False);
  term.add_literal(String::from("B"), literal_value::LiteralValue::True);

  let mut other = product_term::ProductTerm::new();
  other.add_literal(String::from("A"), literal_value::LiteralValue::True);
  other.add_literal(String::from("B"), literal_value::LiteralValue::True);

  let mut set = HashSet::new();
  set.insert(term);
  set.insert(other);
  let mut vec = vec!["A".to_string(), "B".to_string()];

  if let Ok(result_set) = ternary_tree_minimization::TernaryTreeMinimization::apply(&set, &mut vec)
  {
    for term in result_set {
      println!("{}", term.to_boolean_expression());
    }
  }
}
