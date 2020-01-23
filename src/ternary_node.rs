use product_term::ProductTerm;
use std::hash::{Hash, Hasher};
use std::option::Option;

/**
 * Struct that represents a TernaryNode.
 * It contains a product term and the node variable.
 */
pub struct TernaryNode {
    variable: Option<String>,
    built_term: Option<ProductTerm>,
}

impl TernaryNode {
    /**
     * Clones the given node
     * @param node node to be cloned
     * @return clone of the given node
     */
    pub fn clone(node: &TernaryNode) -> TernaryNode {
        TernaryNode {
            variable: if node.variable.is_some() {
                Some(String::clone(node.variable.as_ref().unwrap()))
            } else {
                None
            },
            built_term: if node.built_term.is_some() {
                Some(node.built_term.as_ref().unwrap().clone())
            } else {
                None
            },
        }
    }

    /**
     * Creates a new node
     * @return a new empty node
     */
    pub fn new() -> TernaryNode {
        TernaryNode {
            variable: None,
            built_term: None,
        }
    }

    /**
     * Creates a new node with the given product term attached
     * @param term product term to attach to the new node
     * @return a new node with the given product term
     */
    pub fn new_with_term(term: ProductTerm) -> TernaryNode {
        TernaryNode {
            built_term: Some(term),
            ..TernaryNode::new()
        }
    }

    /**
     * Creates a new node with the given information
     * @param variable the variable contained by this node
     * @param built_term the product term to attach to this node
     * @return a new node with the given information
     */
    pub fn new_with_all(variable: Option<String>, built_term: Option<ProductTerm>) -> TernaryNode {
        TernaryNode {
            variable,
            built_term,
        }
    }

    pub fn set_variable(&mut self, variable: String) {
        self.variable = Some(variable);
    }

    pub fn get_variable(&self) -> Option<&String> {
        self.variable.as_ref()
    }

    pub fn set_term(&mut self, term: ProductTerm) {
        self.built_term = Some(term);
    }

    pub fn get_term(&self) -> Option<&ProductTerm> {
        self.built_term.as_ref()
    }
}

impl<'a> PartialEq<TernaryNode> for TernaryNode {
    /**
     * Method to test equality between this ternary node and another
     * @param other the other ternary node to compare to
     * @return true if this node is equal to the given node; false otherwise
     */
    fn eq(&self, other: &TernaryNode) -> bool {
        if let Some(this_variable) = &self.variable {
            if let Some(other_variable) = &other.variable {
                if this_variable != other_variable {
                    return false;
                }
            } else {
                return false;
            }
        } else if let Some(_) = &other.variable {
            return false;
        }

        if let Some(this_term) = &self.built_term {
            if let Some(other_term) = &other.built_term {
                return this_term == other_term;
            }
        }

        self.built_term.is_none() && other.built_term.is_none()
    }
}

impl Eq for TernaryNode {}

impl Hash for TernaryNode {
    /**
     * Hash method for the ternary node
     */
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.built_term.as_ref().hash(state);
    }
}

impl std::fmt::Debug for TernaryNode {
    /**
     * Utility method for debugging purposes that prints this node's string representation
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dummy_term = ProductTerm::new();

        write!(
            f,
            "(var={}, term={})",
            if self.variable.is_some() {
                self.variable.as_ref().unwrap()
            } else {
                ""
            },
            if self.built_term.is_some() {
                self.built_term.as_ref().unwrap()
            } else {
                &dummy_term
            }
        )
    }
}
