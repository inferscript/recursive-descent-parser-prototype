pub type ISProgram = Vec<ISTopLevel>;
pub type ISXProgram = Vec<ISXTopLevel>;
pub type ISDProgram = Vec<ISDTopLevel>;

pub struct Position {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

pub enum ISTopLevel {
    Comment(Comment),
    ImportStatement(ImportStatement),
    ImportTypeStatement(ImportTypeStatement),
    TypeAliasDeclaration(TypeAliasDeclaration),
    VariableDeclarationStatement(ISVariableDeclarationStatement),
    ExpressionStatement(ISExpressionStatement),
    BlockStatement(ISBlockStatement),
    ExportStatement(ExportStatement),
    ExportTypeStatement(ExportTypeStatement),
}

pub enum ISXTopLevel {
    Comment(Comment),
    ImportStatement(ImportStatement),
    ImportTypeStatement(ImportTypeStatement),
    TypeAliasDeclaration(TypeAliasDeclaration),
    VariableDeclarationStatement(ISXVariableDeclarationStatement),
    ExpressionStatement(ISXExpressionStatement),
    BlockStatement(ISXBlockStatement),
    ExportStatement(ExportStatement),
    ExportTypeStatement(ExportTypeStatement),
}

pub enum ISDTopLevel {
    Comment(Comment),
    ImportTypeStatement(ImportTypeStatement),
    TypeAliasDeclaration(TypeAliasDeclaration),
    DeclareVariableDeclaration(DeclareVariableDeclaration),
    DeclareFunctionDeclaration(DeclareFunctionDeclaration),
    ExportTypeStatement(ExportTypeStatement),
}

pub struct Comment {
    pub position: Position,
    pub content: String,
}

pub struct Identifier {
    pub position: Position,
    pub name: String,
}

pub struct StringLiteral {
    pub position: Position,
    pub raw: String,
}

pub struct ImportStatement {
    pub position: Position,
    pub items: Vec<ImportItem>,
    pub from: StringLiteral,
}

pub struct ImportItem {
    pub position: Position,
    pub target: Identifier,
    pub name: Option<Identifier>,
}

pub struct ImportTypeStatement {
    pub position: Position,
    pub items: Vec<ImportItem>,
    pub from: StringLiteral,
}

pub struct TypeAliasDeclaration {
    pub position: Position,
    pub keyword_export: Option<Position>,
    pub keyword_unique: Option<Position>,
    pub keyword_type: Position,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub content: TypeAliasDeclarationContent,
}

pub enum TypeAliasDeclarationContent {
    Object(TypeAliasDeclarationContentObject),
    Alias(TypeAliasDeclarationContentAlias),
}

pub struct TypeAliasDeclarationContentObject {
    extends: Option<TypeAliasDeclarationContentObjectExtends>,
    content: TypeAliasDeclarationContentObjectContent,
}

pub enum TypeAliasDeclarationContentObjectContent {
    Semicolon(Position),
    Some(ObjectType),
}

pub struct TypeAliasDeclarationContentObjectExtends {
    pub keyword_extends: Position,
    pub items: Vec<TypeAliasDeclarationContentObjectExtendsItem>,
}

pub struct TypeAliasDeclarationContentObjectExtendsItem {
    pub target: Type,
    pub comma: Position,
}

pub struct TypeAliasDeclarationContentAlias {
    pub eq: Position,
    pub content: Type,
    pub semicolon: Position,
}

pub enum Type {
    Identifier(Identifier),
    Parenthesized(ParenthesizedType),
    Indexed(IndexedType),
    Member(MemberType),
    Generic(GenericType),
    Object(ObjectType),
    Array(ArrayType),
    Tuple(TupleType),
    Typeof(TypeofType),
    Keyof(KeyofType),
    Literal(Literal),
    Negated(NegatedType),
    Extends(ExtendsType),
    Conditional(ConditionalType),
    TemplateLiteral(TemplateLiteralType),
    Intersection(IntersectionType),
    Union(UnionType),
    Function(FunctionType),
    Readonly(ReadonlyType),
    Infer(InferType),
    Const(Position),
    Intrinsic(Position),
}

pub struct ParenthesizedType {
    pub position: Position,
    pub left: Position,
    pub content: Box<Type>,
    pub right: Position,
}

pub struct IndexedType {
    pub position: Position,
    pub parent: Box<Type>,
    pub left: Position,
    pub subscript: Box<Type>,
    pub right: Position,
}

pub struct MemberType {
    pub position: Position,
    pub parent: Box<Type>,
    pub dot: Position,
    pub subscript: Identifier,
}

pub struct GenericType {
    pub position: Position,
    pub parent: Box<Type>,
    pub parameters: GenericParameters,
}

pub struct GenericParameters {
    pub position: Position,
    pub left: Position,
    pub items: Vec<GenericParametersItem>,
    pub right: Position,
}

pub struct GenericParametersItem {
    pub keyword_const: Option<Position>,
    pub name: Identifier,
    pub parameters: Option<GenericParameters>,
    pub extends: Option<GenericParametersItemExtends>,
    pub constraints: Option<GenericParametersItemConstraints>,
    pub fallback: Option<GenericParametersItemFallback>,
    pub comma: Option<Position>,
}

pub struct GenericParametersItemExtends {
    pub keyword_extends: Position,
    pub content: Type,
}

pub struct GenericParametersItemConstraints {
    pub keyword_constraints: Position,
    pub content: Type,
}

pub struct GenericParametersItemFallback {
    pub eq: Position,
    pub content: Type,
}

pub struct ObjectType {
    pub position: Position,
    pub left: Position,
    pub fields: Vec<ObjectTypeField>,
    pub right: Position,
}

pub enum ObjectTypeField {
    CallSignature(ObjectTypeFieldCallSignature),
    Field(ObjectTypeFieldField),
    MappedField(ObjectTypeFieldMappedField),
}

pub struct ObjectTypeFieldCallSignature {
    pub position: Position,
    pub keyword_abstract: Option<Position>,
    pub keyword_new: Option<Position>,
    pub generic_parameters: Option<GenericParameters>,
    pub parameters: Parameters,
    pub colon: Position,
    pub return_type: ReturnType,
}

pub struct Parameters {
    pub position: Position,
    pub left: Position,
    pub content: Vec<ParametersItem>,
    pub right: Position,
}

pub struct ParametersItem {
    pub content: Parameter,
    pub comma: Option<Position>,
}

pub enum Parameter {
    Named(ParameterNamed),
    Unnamed(ParameterUnnamed),
}

pub struct ParameterNamed {
    pub position: Position,
    pub name: Identifier,
    pub question_mark: Option<Position>,
    pub colon: Option<Position>,
    pub content: Type,
}

pub struct ParameterUnnamed {
    pub position: Position,
    pub content: Type,
    pub question_mark: Option<Position>,
}

pub enum ReturnType {
    Type(Type),
    TypePredicateIs(TypePredicateIs),
    TypePredicateAsserts(TypePredicateAsserts),
}

pub struct TypePredicateIs {
    pub position: Position,
    pub variable: Identifier,
    pub keyword_is: Position,
    pub predicate: Type,
}

pub struct TypePredicateAsserts {
    pub position: Position,
    pub value: Option<Identifier>,
    pub keyword_asserts: Position,
    pub variable: Identifier,
    pub keyword_is: Position,
    pub predicate: Type,
}
