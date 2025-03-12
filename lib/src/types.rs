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
    pub export: bool,
    pub unique: bool,
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>,
    pub content: TypeAliasDeclarationContent,
}

pub enum TypeAliasDeclarationContent {
    Object(TypeAliasDeclarationContentObject),
    Alias(TypeAliasDeclarationContentAlias),
}
