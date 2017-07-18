use syntax::symbol::Symbol;
use dart;
use node::Node;

pub enum Item {
    ComponentDef(Symbol, Vec<FieldDef>, Instance),
    Dart(Vec<Node<dart::ast::Item>>),
}

pub struct Instance {
    pub name: Symbol,
    pub fields: Vec<Field>,
}

pub struct FieldDef {
    pub name: Symbol,
    pub ty: Option<Type>,
    pub default: Option<Expr>,
}

pub struct Field {
    pub name: Symbol,
    pub value: Expr,
}

pub enum Type {
    Dart(Node<dart::ast::Type>),
}

pub enum Expr {
    Instance(Instance),
    Array(Vec<Expr>),
    Dart(Node<dart::ast::Expr>),
}
