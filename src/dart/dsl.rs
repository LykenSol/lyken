use syntax::symbol::Symbol;

pub enum Item {
    ComponentDef(Symbol, Vec<ComponentPart>),
    VerbatimDart(String),
}

pub enum ComponentPart {
    Instance(Instance),
    Field(Symbol, Option<Type>, Option<Expr>),
}

pub struct Instance {
    pub name: Symbol,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: Symbol,
    pub value: Expr,
}

pub struct Type {
    pub dart: String,
}

pub enum Expr {
    Instance(Instance),
    VerbatimDart(String),
}