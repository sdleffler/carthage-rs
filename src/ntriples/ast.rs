#[derive(Debug, Clone)]
pub struct Document<'input> {
    pub triples: Vec<Triple<'input>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Triple<'input> {
    pub subject: Subject<'input>,
    pub predicate: &'input str,
    pub object: Object<'input>,
}

#[derive(Debug, Clone, Copy)]
pub enum Subject<'input> {
    IriRef(&'input str),
    BlankNodeLabel(&'input str),
}

#[derive(Debug, Clone, Copy)]
pub enum Object<'input> {
    IriRef(&'input str),
    BlankNodeLabel(&'input str),
    Literal(Literal<'input>),
}

#[derive(Debug, Clone, Copy)]
pub struct Literal<'input> {
    pub value: &'input str,
    pub datatype: Option<Datatype<'input>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Datatype<'input> {
    IriRef(&'input str),
    LangTag(&'input str),
}
