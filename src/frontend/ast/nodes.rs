use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SourceSpan{
    pub line: u32,
    pub column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

// ========================================
// AST MODULE
// ========================================

#[derive(Clone, Debug)]
pub struct AstModule {
    pub name: String,
    pub imports: Vec<AstImport>,
    pub body: Vec<AstNode>,
}

#[derive(Clone, Debug)]
pub struct AstImport {
    pub path: String,
    pub alias: Option<String>,
}

// =========================================
// AST TYPES
// =========================================

#[derive(Clone, Debug)]
pub struct AstStruct {
    pub name: String,
    pub fields: Vec<AstField>,
}

#[derive(Clone, Debug)]
pub struct AstField {
    pub name: String,
    pub ty: String,
}

#[derive(Clone, Debug)]
pub struct AstEnum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AstUnion {
    pub name: String,
    pub variants: Vec<AstUnionVariant>,
}

#[derive(Clone, Debug)]
pub struct AstUnionVariant {
    pub kind: String,
    pub fields: Vec<AstField>,
}

#[derive(Clone, Debug)]
pub struct AstTrait {
    pub name: String,
    pub methods: Vec<AstFunctionSignature>,
}

#[derive(Clone, Debug)]
pub struct AstFunctionSignature {
    pub name: String,
    pub span: SourceSpan,
    pub params: Vec<String>,
    pub return_type: String, 
}

// =========================================
// AST IMPLEMENTATIONS
// =========================================

#[derive(Clone, Debug)]
pub struct AstImpl {
    pub trait_name: String,
    pub target_type: String,
    pub methods: Vec<AstFunction>,
}

// ==========================================
// AST FUNCTIONS
// ==========================================

#[derive(Clone, Debug)]
pub struct AstFunction {
    pub name: String,
    pub span: SourceSpan,
    pub params: Vec<String>,
    pub return_type: Option<String>,
    pub body: Vec<AstStatement>,
    pub is_async: bool,
}

// ==========================================
// AST STATEMENTS
// ==========================================

#[derive(Clone, Debug)]
pub struct AstMove {
    pub source: AstExpr,
    pub target: AstExpr,
}

#[derive(Clone, Debug)]
pub struct AstClone {
    pub source: AstExpr,
    pub target: AstExpr,
}

#[derive(Clone, Debug)]
pub struct AstLet {
    pub name: String,
    pub value: AstExpr,
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug)]
pub struct AstConst {
    pub name: String,
    pub value: AstExpr,
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug)]
pub struct AstReturn {
    pub value: AstExpr,
}

#[derive(Clone, Debug)]
pub struct AstLoop {
    pub body: Vec<AstStatement>,
}

#[derive(Clone, Debug)]
pub struct AstIf {
    pub condition: AstExpr,
    pub then_branch: Vec<AstStatement>,
    pub else_branch: Vec<AstStatement>,
}

#[derive(Clone, Debug)]
pub struct AstTry {
    pub try_body: Vec<AstStatement>,
    pub catch_body: Vec<AstStatement>,
    pub finally_body: Vec<AstStatement>,
}

#[derive(Clone, Debug)]
pub struct AstMatch {
    pub scrutinee: AstExpr,
    pub arms: Vec<AstMatchArm>,
    pub otherwise: Option<Vec<AstStatement>>,
}

#[derive(Clone, Debug)]
pub struct AstMatchArm {
    pub pattern: String,
    pub body: Vec<AstStatement>,
}

#[derive(Clone, Debug)]
pub enum AstStatement {
    Move(AstMove),
    Clone(AstClone),
    Let(AstLet),
    Const(AstConst),
    Return(AstReturn),
    Loop(AstLoop),
    If(AstIf),
    Try(AstTry),
    Match(AstMatch),
    Expr(AstExpr),
}

// ===================================
// AST EXPRESSIONS
// ===================================

#[derive(Clone, Debug)]
pub enum AstLiteral {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
    List(Vec<AstLiteral>),
}

#[derive(Clone, Debug)]
pub struct AstObjectInit {
    pub type_name: String,
    fields: HashMap<String, AstExpr>,
}

#[derive(Clone, Debug)]
pub struct AstBinary {
    pub kind: String,
    pub left: Box<AstExpr>,
    pub right: Box<AstExpr>,
}

#[derive(Clone, Debug)]
pub struct AstUnary {
    pub kind: String,
    pub expr: Box<AstExpr>,
}

#[derive(Clone, Debug)]
pub struct AstCall {
    pub func: String,
    pub args: Vec<AstExpr>,
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug)]
pub struct AstVar {
    pub name: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug)]
pub struct AstPipeline {
    pub value: Box<AstExpr>,
    pub func: String,
}

#[derive(Clone, Debug)]
pub enum AstExpr {
    Literal(AstLiteral),
    Binary(Box<AstBinary>),
    Unary(Box<AstUnary>),
    Call(AstCall),
    Var(AstVar),
    Pipeline(Box<AstPipeline>),
    ObjectInit(AstObjectInit),
}

#[derive(Clone, Debug)]
pub enum AstNode {
    Struct(AstStruct),
    Enum(AstEnum),
    Union(AstUnion),
    Trait(AstTrait),
    Impl(AstImpl),
    Function(AstFunction),
    Statement(AstStatement),
}
