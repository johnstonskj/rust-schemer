/*!
One-line description.

More detailed description, with

# Example

*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const EMPTY_STR: &str = "";

pub const SYNTAX_DOUBLE_QUOTE_CHAR: char = '"';
pub const SYNTAX_DOUBLE_QUOTE: &str = "\"";
pub const SYNTAX_HASH_CHAR: char = '#';
pub const SYNTAX_HASH: &str = "#";
pub const SYNTAX_HYPHEN_CHAR: char = '-';
pub const SYNTAX_HYPHEN: &str = "-";
pub const SYNTAX_LEFT_PARENTHESIS_CHAR: char = '(';
pub const SYNTAX_LEFT_PARENTHESIS: &str = "(";
pub const SYNTAX_MATH_COMPLEX_CHAR: char = 'i';
pub const SYNTAX_MATH_COMPLEX: &str = "i";
pub const SYNTAX_MATH_EQUALITY_CHAR: char = '=';
pub const SYNTAX_MATH_EQUALITY: &str = "=";
pub const SYNTAX_MATH_MINUS_CHAR: char = SYNTAX_HYPHEN_CHAR;
pub const SYNTAX_MATH_MINUS: &str = SYNTAX_HYPHEN;
pub const SYNTAX_MATH_PLUS_CHAR: char = '+';
pub const SYNTAX_MATH_PLUS: &str = "+";
pub const SYNTAX_PERIOD_CHAR: char = '.';
pub const SYNTAX_PERIOD: &str = ".";
pub const SYNTAX_RIGHT_PARENTHESIS_CHAR: char = ')';
pub const SYNTAX_RIGHT_PARENTHESIS: &str = ")";
pub const SYNTAX_SPACE_CHAR: char = ' ';
pub const SYNTAX_SPACE: &str = " ";
pub const SYNTAX_UNDERSCORE_CHAR: char = '_';
pub const SYNTAX_UNDERSCORE: &str = "_";

pub const SYNTAX_ABBR_QUOTE: &str = "'";
pub const SYNTAX_ABBR_UNQUOTE_SPLICING: &str = ",@";
pub const SYNTAX_ABBR_QUASI_QUOTE: &str = "`";
pub const SYNTAX_ABBR_UNQUOTE: &str = ",";
pub const SYNTAX_BYTE_VECTOR_PREFIX: &str = "#u8";
pub const SYNTAX_CHAR_PREFIX: &str = "#\\";
pub const SYNTAX_COMMENT_START: &str = "#|";
pub const SYNTAX_COMMENT_END: &str = "|#";
pub const SYNTAX_CONS_DOT: &str = " . ";
pub const SYNTAX_DIRECTIVE_FOLD_CASE: &str = "#!fold-case";
pub const SYNTAX_DIRECTIVE_NO_FOLD_CASE: &str = "#!no-fold-case";
pub const SYNTAX_HEX_CHAR_PREFIX: &str = "#\\x";
pub const SYNTAX_NULL_LIST: &str = "()";
pub const SYNTAX_VECTOR_PREFIX: &str = SYNTAX_HASH;

pub const PSEUDO_SYNTAX_COLON_CHAR: char = ':';
pub const PSEUDO_SYNTAX_COLON: &str = ":";
pub const PSEUDO_SYNTAX_LEFT_PROCEDURE: &str = "#<";
pub const PSEUDO_SYNTAX_RANGE: &str = "..";
pub const PSEUDO_SYNTAX_RIGHT_PROCEDURE: &str = ">";

pub const VALUE_BOOLEAN_TRUE: &str = "#true";
pub const VALUE_BOOLEAN_TRUE_SHORT: &str = "#t";
pub const VALUE_BOOLEAN_FALSE: &str = "#false";
pub const VALUE_BOOLEAN_FALSE_SHORT: &str = "#f";
pub const VALUE_MATH_INFINITY_NEGATIVE: &str = "-inf.0";
pub const VALUE_MATH_INFINITY_POSITIVE: &str = "+inf.0";
pub const VALUE_MATH_NAN_NEGATIVE: &str = "-nan.0";
pub const VALUE_MATH_NAN_POSITIVE: &str = "+nan.0";
pub const VALUE_NULL_LIST: &str = "null";
pub const VALUE_SYMBOL_NIL: &str = "nil";

pub const FORM_NAME_AND: &str = "and";
pub const FORM_NAME_BEGIN: &str = "begin";
pub const FORM_NAME_CASE: &str = "case";
pub const FORM_NAME_COND: &str = "cond";
pub const FORM_NAME_DEFINE: &str = "define";
pub const FORM_NAME_DELAY: &str = "delay";
pub const FORM_NAME_DELAY_FORCE: &str = "delay-force";
pub const FORM_NAME_ELSE: &str = "else";
pub const FORM_NAME_FORCE: &str = "force";
pub const FORM_NAME_IF: &str = "if";
pub const FORM_NAME_IMPORT: &str = "import";
pub const FORM_NAME_PROMISE: &str = "promise?";
pub const FORM_NAME_LAMBDA: &str = "lambda";
pub const FORM_NAME_LAMBDA_ALT: &str = "λ";
pub const FORM_NAME_MAKE_PROMISE: &str = "make-promise";
pub const FORM_NAME_OR: &str = "or";
pub const FORM_NAME_QUASI_QUOTE: &str = "quasiquote";
pub const FORM_NAME_QUOTE: &str = "quote";
pub const FORM_NAME_SET: &str = "set!";
pub const FORM_NAME_UNQUOTE: &str = "unquote";
pub const FORM_NAME_UNQUOTE_SPLICING: &str = "unquote-splicing";
pub const FORM_NAME_UNLESS: &str = "unless";
pub const FORM_NAME_WHEN: &str = "when";
