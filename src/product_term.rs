use linked_hash_map::LinkedHashMap;
use literal_value::LiteralValue;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/**
 * Struct that represents a product term in a boolean expression (a product of sums
 * in a boolean expression that is in disjunctive normal form).
 * A product term differenciates itself by a min term by the aspect that a product term can contain a don't care literal.
 * (Any min term is a product term, but not every product term is a min term)
 *
 * e.g. Let's take the following boolean function(that is in DNF) into consideration: f(A,B,C) = ~A&B | A&B&C
 * We can conclude the following statements from this function:
 *  - ~A&B is a product term
 *  - A&B&C is a product term and a min term
 */
pub struct ProductTerm {
  literals: LinkedHashMap<String, LiteralValue>,
}

impl ProductTerm {
  /**
   * Clones this product term
   * @return a clone of this product term
   */
  pub fn clone(&self) -> ProductTerm {
    let mut literals = LinkedHashMap::new();
    for (variable, literal) in &self.literals {
      literals.insert(String::clone(variable), *literal);
    }

    ProductTerm { literals }
  }

  /**
   * Creates a new product term with the given literals
   * @param literals a vector of tuples of the form (variable, literal)
   * @return a new product term with the given literals
   */
  pub fn new_with_literals(literals: std::vec::Vec<(String, LiteralValue)>) -> ProductTerm {
    let mut new_literals = LinkedHashMap::new();
    for (variable, literal) in literals {
      new_literals.insert(variable, literal);
    }

    ProductTerm {
      literals: new_literals,
    }
  }

  /**
   * Creates a new product term
   * @return new empty product term
   */
  pub fn new() -> ProductTerm {
    ProductTerm {
      literals: LinkedHashMap::new(),
    }
  }

  /**
   * Checks if this product term is empty (contains no literals)
   * @return true if the product term is empty; false otherwise
   */
  pub fn is_empty(&self) -> bool {
    self.literals.is_empty()
  }

  /**
   * Clones this product term literals
   * @return a linked hash map containing this product term literals
   */
  pub fn get_literals(&self) -> LinkedHashMap<String, LiteralValue> {
    self.literals.clone()
  }

  /**
   * Adds a given literal
   * @param name name of the literal
   * @param literal value of the literal
   */
  pub fn add_literal(&mut self, name: String, literal: LiteralValue) {
    self.literals.insert(name, literal);
  }

  /**
   * Removes the last literal that was added
   * @return value of the removed literal
   */
  pub fn remove_last(&mut self) -> LiteralValue {
    self.literals.pop_back().unwrap().1
  }

  /**
   * Merges this product term with another one
   * @param other a product term with which to try merge
   * @return a new product term representing the merge of these product terms
   * @throws error if the product terms cannot be merged
   */
  pub fn merge(&self, other: &ProductTerm) -> Result<ProductTerm, String> {
    if !self.can_merge(other) {
      return Err("Cannot merge product terms!".to_string());
    }

    let mut combined_literals = 0;
    let mut new_product_term = ProductTerm::new();
    for (variable, literal) in &self.literals {
      if let Some(other_literal) = other.literals.get(variable) {
        if *other_literal == LiteralValue::DontCare && *literal != LiteralValue::DontCare
          || *literal == LiteralValue::DontCare && *other_literal != LiteralValue::DontCare
        {
          return Err("Cannot merge product terms!".to_string());
        } else if *other_literal != *literal {
          new_product_term.add_literal(variable.clone(), LiteralValue::DontCare);
          combined_literals += 1;
          if combined_literals > 1 {
            return Err("Cannot merge product terms!".to_string());
          }
        } else {
          new_product_term.add_literal(variable.clone(), *literal);
        }
      }
    }

    Ok(new_product_term)
  }

  /**
   * Checks if this product term can be merged with a given one
   * @param other a product term to check if it can be merged with this
   * @return true if this product term can be merged with the other; false otherwise
   */
  fn can_merge(&self, other: &ProductTerm) -> bool {
    if self.literals.keys().len() != other.literals.keys().len() {
      return false;
    }

    for (variable, _) in &self.literals {
      if !other.literals.contains_key(variable) {
        return false;
      }
    }

    true
  }

  /**
   * Transforms this product term in its string representation
   * @return string representation of this product term
   */
  pub fn to_boolean_expression(&self) -> String {
    let mut buffer = "".to_string();
    for (variable, literal) in &self.literals {
      if *literal == LiteralValue::False {
        if buffer != "" {
          buffer.push_str(&"&");
        }
        buffer.push_str(&"~");
        buffer.push_str(variable);
      } else if *literal == LiteralValue::True {
        if buffer != "" {
          buffer.push_str(&"&");
        }
        buffer.push_str(variable);
      }
    }

    buffer
  }

  /**
   * Checks if this product term is a prefix of another
   * @param other product term to check if it contains this product term
   * @return true if this product term is a prefix of the given product term; false otherwise
   */
  pub fn is_prefix_of(&self, other: &ProductTerm) -> bool {
    for (variable, literal) in &self.literals {
      if let Some(other_literal) = other.get_literals().get(variable) {
        if other_literal != literal {
          return false;
        }
      } else {
        return false;
      }
    }

    true
  }

  /**
   * Checks if this product term is a prefix of any of the given product terms
   * @param terms set containing product terms to check if any of them contains this product term
   * @return true if this product term is a prefix of at least one of the given terms; false otherwise
   */
  pub fn is_prefix_of_any(&self, terms: &HashSet<ProductTerm>) -> bool {
    for product_term in terms {
      if self.is_prefix_of(&product_term) {
        return true;
      }
    }

    false
  }

  /**
   * Checks if this product term matches any of the given product terms
   * @param terms set containing product terms to check if any of them matches this product term
   * @return true if this product term matches at least one of the given product terms; false otherwise
   */
  pub fn matches_any(&self, terms: &HashSet<ProductTerm>) -> bool {
    for product_term in terms {
      if self.matches(&product_term) {
        return true;
      }
    }

    false
  }

  /**
   * Checks if this product term matches another given product term
   * @param term the other product term to compare to
   * @return true if this product term matches the given one; false otherwise
   */
  fn matches(&self, term: &ProductTerm) -> bool {
    self.is_prefix_of(term) && self.literals.len() == term.get_literals().len()
  }
}

impl PartialEq<ProductTerm> for ProductTerm {
  /**
   * Checks if this product term is equal with another
   * @param other other product term to check for equality
   * @return true if this product term is equal with the given one; false otherwise
   */
  fn eq(&self, other: &ProductTerm) -> bool {
    if self.literals.len() != other.literals.len() {
      return false;
    }

    for (variable, literal) in &self.literals {
      if !other.literals.contains_key(variable) {
        return false;
      }

      if let Some(other_literal) = other.literals.get(variable) {
        if other_literal != literal {
          return false;
        }
      }
    }

    true
  }
}

impl std::fmt::Debug for ProductTerm {
  /**
   * Method that formats this product term's string representation for debugging purposes
   */
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.to_boolean_expression())
  }
}

impl Hash for ProductTerm {
  /**
   * Hash function for this product term
   */
  fn hash<H: Hasher>(&self, state: &mut H) {
    for (variable, literal) in &self.literals {
      variable.hash(state);
      literal.hash(state);
    }
  }
}

impl Eq for ProductTerm {}

impl<'a> std::fmt::Display for ProductTerm {
  /**
   * Method that prints this product term's string representation
   */
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({})", self.to_boolean_expression())
  }
}

/**
 * Module for tests regarding the Product Term struct and its methods
 */
#[cfg(test)]
mod product_term_tests {
  use super::*;

  #[test]
  fn test_merge_01() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::True);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::True);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut expected_result = ProductTerm::new();
    expected_result.add_literal(String::from("B"), LiteralValue::True);
    expected_result.add_literal(String::from("A"), LiteralValue::DontCare);
    expected_result.add_literal(String::from("C"), LiteralValue::DontCare);

    assert_eq!(term.merge(&other).unwrap(), expected_result);
  }

  #[test]
  fn test_merge_02() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);
    term.add_literal(String::from("B"), LiteralValue::False);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::True);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut expected_result = ProductTerm::new();
    expected_result.add_literal(String::from("B"), LiteralValue::DontCare);
    expected_result.add_literal(String::from("A"), LiteralValue::True);
    expected_result.add_literal(String::from("C"), LiteralValue::DontCare);

    assert_eq!(term.merge(&other).unwrap(), expected_result);
  }

  #[test]
  fn test_merge_03_fail() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);
    term.add_literal(String::from("B"), LiteralValue::False);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::True);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    match term.merge(&other) {
      Ok(_) => assert!(false),
      Err(_) => assert!(true),
    }
  }

  #[test]
  fn test_merge_04_fail() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::False);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::True);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    match term.merge(&other) {
      Ok(_) => assert!(false),
      Err(_) => assert!(true),
    }
  }

  #[test]
  fn test_merge_05() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::DontCare);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::DontCare);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut expected_result = ProductTerm::new();
    expected_result.add_literal(String::from("B"), LiteralValue::DontCare);
    expected_result.add_literal(String::from("A"), LiteralValue::DontCare);
    expected_result.add_literal(String::from("C"), LiteralValue::DontCare);

    assert_eq!(term.merge(&other).unwrap(), expected_result);
  }

  #[test]
  fn test_merge_06_fail() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::DontCare);
    term.add_literal(String::from("B"), LiteralValue::False);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::DontCare);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    match term.merge(&other) {
      Ok(_) => assert!(false),
      Err(_) => assert!(true),
    }
  }

  #[test]
  fn test_merge_07_fail() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::DontCare);
    term.add_literal(String::from("B"), LiteralValue::False);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::DontCare);

    match term.merge(&other) {
      Ok(_) => assert!(false),
      Err(_) => assert!(true),
    }
  }

  #[test]
  fn test_merge_08() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::True);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::False);
    other.add_literal(String::from("B"), LiteralValue::False);

    let mut expected_result = ProductTerm::new();
    expected_result.add_literal(String::from("A"), LiteralValue::False);
    expected_result.add_literal(String::from("B"), LiteralValue::DontCare);

    assert_eq!(term.merge(&other).unwrap(), expected_result);
  }

  #[test]
  fn test_merge_09_fail() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::False);
    other.add_literal(String::from("B"), LiteralValue::False);

    match term.merge(&other) {
      Ok(_) => assert!(false),
      Err(_) => assert!(true),
    }
  }

  #[test]
  fn test_merge_10_fail() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::False);
    term.add_literal(String::from("C"), LiteralValue::False);
    term.add_literal(String::from("D"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::False);
    other.add_literal(String::from("B"), LiteralValue::True);
    other.add_literal(String::from("C"), LiteralValue::False);
    other.add_literal(String::from("D"), LiteralValue::False);

    match term.merge(&other) {
      Ok(_) => assert!(false),
      Err(_) => assert!(true),
    }
  }

  #[test]
  fn test_merge_11_fail() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::False);
    term.add_literal(String::from("C"), LiteralValue::False);
    term.add_literal(String::from("D"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::False);
    other.add_literal(String::from("B"), LiteralValue::True);
    other.add_literal(String::from("C"), LiteralValue::DontCare);
    other.add_literal(String::from("D"), LiteralValue::False);

    match term.merge(&other) {
      Ok(_) => assert!(false),
      Err(_) => assert!(true),
    }
  }

  #[test]
  fn test_equals_01() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::True);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::False);
    other.add_literal(String::from("B"), LiteralValue::True);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    assert_eq!(term, other);
  }

  #[test]
  fn test_equals_02() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::True);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::True);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    assert_ne!(term, other);
  }

  #[test]
  fn test_equals_03() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::DontCare);
    term.add_literal(String::from("B"), LiteralValue::True);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::False);
    other.add_literal(String::from("C"), LiteralValue::DontCare);

    assert_ne!(term, other);
  }

  #[test]
  fn test_equals_04() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::DontCare);
    term.add_literal(String::from("B"), LiteralValue::True);
    term.add_literal(String::from("C"), LiteralValue::DontCare);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);
    other.add_literal(String::from("B"), LiteralValue::False);

    assert_ne!(term, other);
  }

  #[test]
  fn test_equals_05() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);

    let mut other = ProductTerm::new();
    other.add_literal(String::from("A"), LiteralValue::True);

    assert_eq!(term, other);
  }

  #[test]
  fn test_equals_06() {
    let term = ProductTerm::new();
    let other = ProductTerm::new();

    assert_eq!(term, other);
  }

  #[test]
  fn test_is_prefix_of_01() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::True);
    product_term.add_literal(String::from("B"), LiteralValue::DontCare);

    assert!(term.is_prefix_of(&product_term));
  }

  #[test]
  fn test_is_prefix_of_02() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);
    term.add_literal(String::from("B"), LiteralValue::DontCare);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::True);
    product_term.add_literal(String::from("B"), LiteralValue::DontCare);

    assert!(term.is_prefix_of(&product_term));
  }

  #[test]
  fn test_is_prefix_of_03() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::True);

    assert!(term.is_prefix_of(&product_term));
  }

  #[test]
  fn test_is_prefix_of_04() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::False);

    assert!(!term.is_prefix_of(&product_term));
  }

  #[test]
  fn test_is_prefix_of_05() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::True);
    product_term.add_literal(String::from("B"), LiteralValue::False);

    assert!(!term.is_prefix_of(&product_term));
  }

  #[test]
  fn test_is_prefix_of_06() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);
    term.add_literal(String::from("B"), LiteralValue::True);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::True);
    product_term.add_literal(String::from("B"), LiteralValue::False);

    assert!(!term.is_prefix_of(&product_term));
  }

  #[test]
  fn test_is_prefix_of_any_01() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::True);
    product_term.add_literal(String::from("B"), LiteralValue::False);

    let mut other_term = ProductTerm::new();
    other_term.add_literal(String::from("A"), LiteralValue::False);
    other_term.add_literal(String::from("B"), LiteralValue::True);

    let mut set = HashSet::new();
    set.insert(product_term);
    set.insert(other_term);

    assert!(term.is_prefix_of_any(&set));
  }

  #[test]
  fn test_is_prefix_of_any_02() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::True);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::False);
    product_term.add_literal(String::from("B"), LiteralValue::True);

    let mut other_term = ProductTerm::new();
    other_term.add_literal(String::from("A"), LiteralValue::True);
    other_term.add_literal(String::from("B"), LiteralValue::True);

    let mut set = HashSet::new();
    set.insert(product_term);
    set.insert(other_term);

    assert!(term.is_prefix_of_any(&set));
  }

  #[test]
  fn test_is_prefix_of_any_03() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("B"), LiteralValue::DontCare);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::False);
    product_term.add_literal(String::from("B"), LiteralValue::True);

    let mut other_term = ProductTerm::new();
    other_term.add_literal(String::from("A"), LiteralValue::False);
    other_term.add_literal(String::from("B"), LiteralValue::False);

    let third_term = ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
    ]);
    let mut set = HashSet::new();
    set.insert(product_term);
    set.insert(other_term);
    set.insert(third_term);

    assert!(!term.is_prefix_of_any(&set));
  }

  #[test]
  fn test_is_prefix_of_any_04() {
    let mut term = ProductTerm::new();
    term.add_literal(String::from("A"), LiteralValue::False);
    term.add_literal(String::from("B"), LiteralValue::DontCare);

    let mut product_term = ProductTerm::new();
    product_term.add_literal(String::from("A"), LiteralValue::False);
    product_term.add_literal(String::from("B"), LiteralValue::True);

    let mut other_term = ProductTerm::new();
    other_term.add_literal(String::from("A"), LiteralValue::False);
    other_term.add_literal(String::from("B"), LiteralValue::False);

    let third_term = ProductTerm::new_with_literals(vec![
      ("A".to_string(), LiteralValue::True),
      ("B".to_string(), LiteralValue::False),
    ]);
    let mut set = HashSet::new();
    set.insert(product_term);
    set.insert(other_term);
    set.insert(third_term);

    assert!(!term.is_prefix_of_any(&set));
  }
}
