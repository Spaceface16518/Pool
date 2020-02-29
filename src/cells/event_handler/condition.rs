use super::Target;
use crate::cells::event_handler::SizeComparator;
use Node::*;

#[derive(Clone, Debug)]
pub struct ConditionExpression {
    head: Node,
}

impl ConditionExpression {
    pub fn evaluate(&self, conditions: &[Condition]) -> bool {
        fn eval(node: &Node, conditions: &[Condition]) -> bool {
            match node {
                Cond(condition) => conditions.contains(condition),
                Not(node) => !eval(node, conditions),
                And(left, right) => eval(left, conditions) && eval(right, conditions),
                Or(left, right) => eval(left, conditions) || eval(right, conditions),
                XOr(left, right) => eval(left, conditions) ^ eval(right, conditions),
            }
        }
        eval(&self.head, conditions)
    }
}

#[derive(Clone, Debug, Hash)]
enum Node {
    Cond(Condition),
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    XOr(Box<Node>, Box<Node>),
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Condition {
    Near(Target),
    Hungry,
    Closest(SizeComparator),
}
