#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token<'t> {
    text: &'t str,
    type_: TokenType,
    modifier: TokenModifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    /// For identifiers that declare or reference a namespace, module, or package.
    Namespace,

    /// For identifiers that declare or reference a class type.
    Class,

    /// For identifiers that declare or reference an enumeration type.
    Enum,

    /// For identifiers that declare or reference an interface type.
    Interface,

    /// For identifiers that declare or reference a struct type.
    Struct,

    /// For identifiers that declare or reference a type parameter.
    TypeParameter,

    /// For identifiers that declare or reference a type that is not covered above.
    Type,

    /// For identifiers that declare or reference a function or method parameters.
    Parameter,

    /// For identifiers that declare or reference a local or global variable.
    Variable,

    /// For identifiers that declare or reference a member property, member field, or member variable.
    Property,

    /// For identifiers that declare or reference an enumeration property, constant, or member.
    EnumMember,

    /// For identifiers that declare or reference decorators and annotations.
    Decorator,

    /// For identifiers that declare an event property.
    Event,

    /// For identifiers that declare a function.
    Function,

    /// For identifiers that declare a member function or method.
    Method,

    /// For identifiers that declare a macro.
    Macro,

    /// For identifiers that declare a label.
    Label,

    /// For tokens that represent a comment.
    Comment,

    /// For tokens that represent a string literal.
    String,

    /// For tokens that represent a language keyword.
    Keyword,

    /// For tokens that represent a number literal.
    Number,

    /// For tokens that represent a regular expression literal.
    Regexp,

    /// For tokens that represent an operator.
    Operator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenModifier {
    /// For declarations of symbols.
    pub declaration: bool,

    /// For definitions of symbols, for example, in header files.
    pub definition: bool,

    /// For readonly variables and member fields (constants).
    pub readonly: bool,

    /// For class members (static members).
    pub static_: bool,

    /// For symbols that should no longer be used.
    pub deprecated: bool,

    /// For types and member functions that are abstract.
    pub abstract_: bool,

    /// For functions that are marked async.
    pub async_: bool,

    /// For variable references where the variable is assigned to.
    pub modification: bool,

    /// For occurrences of symbols in documentation.
    pub documentation: bool,

    /// For symbols that are part of the standard library.
    pub default_library: bool,
}
