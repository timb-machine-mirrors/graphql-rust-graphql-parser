//! Query Language Abstract Syntax Tree (AST)
//!
//! The types and fields here resemble official [graphql grammar] whenever it
//! makes sense for rust.
//!
//! [graphql grammar]: http://facebook.github.io/graphql/October2016/#sec-Appendix-Grammar-Summary
//!
pub use crate::common::{Directive, Number, Text, Type, Value};
use crate::position::Pos;

/// Root of query data
#[derive(Debug, Clone, PartialEq)]
pub struct Document<'a, T: Text<'a>> {
    pub definitions: Vec<Definition<'a, T>>,
}

impl<'a> Document<'a, String> {
    pub fn into_static(self) -> Document<'static, String> {
        // To support both reference and owned values in the AST,
        // all string data is represented with the ::common::Str<'a, T: Text<'a>>
        // wrapper type.
        // This type must carry the lifetime of the query string,
        // and is stored in a PhantomData value on the Str type.
        // When using owned String types, the actual lifetime of
        // the Ast nodes is 'static, since no references are kept,
        // but the nodes will still carry the input lifetime.
        // To continue working with Document<String> in a owned fasion
        // the lifetime needs to be transmuted to 'static.
        //
        // This is safe because no references are present.
        // Just the PhantomData lifetime reference is transmuted away.
        unsafe { std::mem::transmute::<_, Document<'static, String>>(self) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition<'a, T: Text<'a>> {
    Operation(OperationDefinition<'a, T>),
    Fragment(FragmentDefinition<'a, T>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentDefinition<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: T::Value,
    pub type_condition: TypeCondition<'a, T>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationDefinition<'a, T: Text<'a>> {
    SelectionSet(SelectionSet<'a, T>),
    Query(Query<'a, T>),
    Mutation(Mutation<'a, T>),
    Subscription(Subscription<'a, T>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: Option<T::Value>,
    pub variable_definitions: Vec<VariableDefinition<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mutation<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: Option<T::Value>,
    pub variable_definitions: Vec<VariableDefinition<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subscription<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: Option<T::Value>,
    pub variable_definitions: Vec<VariableDefinition<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionSet<'a, T: Text<'a>> {
    pub span: (Pos, Pos),
    pub items: Vec<Selection<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDefinition<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: T::Value,
    pub var_type: Type<'a, T>,
    pub default_value: Option<Value<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selection<'a, T: Text<'a>> {
    Field(Field<'a, T>),
    FragmentSpread(FragmentSpread<'a, T>),
    InlineFragment(InlineFragment<'a, T>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a, T: Text<'a>> {
    pub position: Pos,
    pub alias: Option<T::Value>,
    pub name: T::Value,
    pub arguments: Vec<(T::Value, Value<'a, T>)>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentSpread<'a, T: Text<'a>> {
    pub position: Pos,
    pub fragment_name: T::Value,
    pub directives: Vec<Directive<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeCondition<'a, T: Text<'a>> {
    On(T::Value),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFragment<'a, T: Text<'a>> {
    pub position: Pos,
    pub type_condition: Option<TypeCondition<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
    pub selection_set: SelectionSet<'a, T>,
}
