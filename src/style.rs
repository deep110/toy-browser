/// This module takes DOM Tree and CSS Style sheet
/// and calculates style tree i.e actual values of css 
/// properties
/// 
/// It returns a one to one mapping tree with DOM tree

use super::dom;
use super::parser::css;
use std::collections::hash_map::HashMap;

// Map from CSS property names to values.
type PropertyMap = HashMap<String, css::Value>;

type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

// A node with associated style data.
pub struct StyledNode<'a> {
    node: &'a dom::Node, // pointer to a DOM node
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

impl<'a> std::fmt::Display for StyledNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node.node_type)?;
        write!(f, "AP {:#?}\n", self.specified_values)?;
        for child in self.children.iter() {
            write!(f, "{}", child)?;
        }
        Ok(())
    }
}

// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
pub fn style_tree<'a>(root: &'a dom::Node, stylesheet: &'a css::Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            dom::NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            dom::NodeType::Text(_) => HashMap::new()
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}

// Apply styles to a single element, returning the specified values.
fn specified_values(elem: &dom::ElementData, stylesheet: &css::Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // TODO: also consider inline style tag

    // Go through the rules from lowest to highest specificity.
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    return values;
}

/// Find all CSS rules that match the given element.
/// 
/// We can speed this up by storing the rules in multiple hash tables based
/// on tag name, id, class, etc.
fn matching_rules<'a>(elem: &dom::ElementData, stylesheet: &'a css::Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

/// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
///
/// Iterate through all selectors of a rule and returns the matched one
fn match_rule<'a>(elem: &dom::ElementData, rule: &'a css::Rule) -> Option<MatchedRule<'a>> {
    // Find the first (highest-specificity) matching selector.
    rule.selectors.iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

// If a selector matches an element
fn matches(elem: &dom::ElementData, selector: &css::Selector) -> bool {
    // match all selectors, simple and compound
    match *selector {
        css::Selector::Simple(ref simple_selector) => matches_simple_selector(
            elem, simple_selector)
    }
}

fn matches_simple_selector(elem: &dom::ElementData, selector: &css::SimpleSelector) -> bool {
    // if selector has tag it should match
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // if selector has id it should match
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // if selector has class it should match
    let elem_classes = elem.classes();
    if selector.class.iter().any(|class| !elem_classes.contains(&**class)) {
        return false;
    }

    // return true since everything matches
    return true;
}
