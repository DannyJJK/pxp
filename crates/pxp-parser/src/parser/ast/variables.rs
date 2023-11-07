use std::fmt::Display;

use pxp_bytestring::ByteString;
use pxp_span::Span;
use crate::node::Node;
use crate::parser::ast::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Variable {
    SimpleVariable(SimpleVariable),
    VariableVariable(VariableVariable),
    BracedVariableVariable(BracedVariableVariable),
}

impl Node for Variable {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            Variable::SimpleVariable(variable) => variable.children(),
            Variable::VariableVariable(variable) => variable.children(),
            Variable::BracedVariableVariable(variable) => variable.children(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SimpleVariable {
    pub span: Span,
    pub name: ByteString,
}

impl Node for SimpleVariable {
    //
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct VariableVariable {
    pub span: Span,
    pub variable: Box<Variable>,
}

impl Node for VariableVariable {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.variable.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BracedVariableVariable {
    pub start: Span,
    pub variable: Box<Expression>,
    pub end: Span,
}

impl Node for BracedVariableVariable {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.variable.as_mut()]
    }
}

impl Display for SimpleVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
