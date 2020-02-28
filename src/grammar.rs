pub struct PestParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    grammar_rules,
    grammar_rule,
    assignment_operator,
    opening_brace,
    closing_brace,
    opening_paren,
    closing_paren,
    opening_brack,
    closing_brack,
    modifier,
    silent_modifier,
    atomic_modifier,
    compound_atomic_modifier,
    non_atomic_modifier,
    expression,
    term,
    node,
    terminal,
    prefix_operator,
    infix_operator,
    postfix_operator,
    positive_predicate_operator,
    negative_predicate_operator,
    sequence_operator,
    choice_operator,
    optional_operator,
    repeat_operator,
    repeat_once_operator,
    repeat_exact,
    repeat_min,
    repeat_max,
    repeat_min_max,
    number,
    integer,
    comma,
    _push,
    peek_slice,
    identifier,
    alpha,
    alpha_num,
    string,
    insensitive_string,
    range,
    character,
    inner_str,
    inner_chr,
    escape,
    code,
    unicode,
    hex_digit,
    quote,
    single_quote,
    range_operator,
    WHITESPACE,
    block_comment,
    COMMENT,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for PestParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn grammar_rules(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::grammar_rule(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::grammar_rule(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::grammar_rule(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn grammar_rule(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::grammar_rule, |state| state.sequence(|state| self::identifier(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::assignment_operator(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::modifier(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::opening_brace(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expression(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::closing_brace(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assignment_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assignment_operator, |state| state.match_string("="))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn opening_brace(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::opening_brace, |state| state.match_string("{"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn closing_brace(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::closing_brace, |state| state.match_string("}"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn opening_paren(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::opening_paren, |state| state.match_string("("))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn closing_paren(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::closing_paren, |state| state.match_string(")"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn opening_brack(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::opening_brack, |state| state.match_string("["))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn closing_brack(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::closing_brack, |state| state.match_string("]"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn modifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::silent_modifier(state).or_else(|state| self::atomic_modifier(state)).or_else(|state| self::compound_atomic_modifier(state)).or_else(|state| self::non_atomic_modifier(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn silent_modifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::silent_modifier, |state| state.match_string("_"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn atomic_modifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::atomic_modifier, |state| state.match_string("@"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn compound_atomic_modifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::compound_atomic_modifier, |state| state.match_string("$"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn non_atomic_modifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::non_atomic_modifier, |state| state.match_string("!"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expression, |state| state.sequence(|state| self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::infix_operator(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::infix_operator(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn term(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::term, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::prefix_operator(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::prefix_operator(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::node(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::postfix_operator(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::postfix_operator(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn node(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::opening_paren(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expression(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::closing_paren(state))).or_else(|state| self::terminal(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn terminal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::_push(state).or_else(|state| self::peek_slice(state)).or_else(|state| self::identifier(state)).or_else(|state| self::string(state)).or_else(|state| self::insensitive_string(state)).or_else(|state| self::range(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn prefix_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::positive_predicate_operator(state).or_else(|state| self::negative_predicate_operator(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn infix_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::sequence_operator(state).or_else(|state| self::choice_operator(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn postfix_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::optional_operator(state).or_else(|state| self::repeat_operator(state)).or_else(|state| self::repeat_once_operator(state)).or_else(|state| self::repeat_exact(state)).or_else(|state| self::repeat_min(state)).or_else(|state| self::repeat_max(state)).or_else(|state| self::repeat_min_max(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn positive_predicate_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::positive_predicate_operator, |state| state.match_string("&"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn negative_predicate_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::negative_predicate_operator, |state| state.match_string("!"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn sequence_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::sequence_operator, |state| state.match_string("~"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn choice_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::choice_operator, |state| state.match_string("|"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn optional_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::optional_operator, |state| state.match_string("?"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn repeat_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::repeat_operator, |state| state.match_string("*"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn repeat_once_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::repeat_once_operator, |state| state.match_string("+"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn repeat_exact(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::repeat_exact, |state| state.sequence(|state| self::opening_brace(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::number(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::closing_brace(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn repeat_min(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::repeat_min, |state| state.sequence(|state| self::opening_brace(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::number(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::comma(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::closing_brace(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn repeat_max(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::repeat_max, |state| state.sequence(|state| self::opening_brace(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::comma(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::number(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::closing_brace(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn repeat_min_max(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::repeat_min_max, |state| state.sequence(|state| self::opening_brace(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::number(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::comma(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::number(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::closing_brace(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::number, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_range('0'..'9').and_then(|state| state.repeat(|state| state.match_range('0'..'9'))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::number(state).or_else(|state| state.sequence(|state| state.match_string("-").and_then(|state| state.repeat(|state| state.match_string("0"))).and_then(|state| state.match_range('1'..'9')).and_then(|state| state.optional(|state| self::number(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comma(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::comma, |state| state.match_string(","))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn _push(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::_push, |state| state.sequence(|state| state.match_string("PUSH").and_then(|state| super::hidden::skip(state)).and_then(|state| self::opening_paren(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expression(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::closing_paren(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn peek_slice(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::peek_slice, |state| state.sequence(|state| state.match_string("PEEK").and_then(|state| super::hidden::skip(state)).and_then(|state| self::opening_brack(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::integer(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::range_operator(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::integer(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::closing_brack(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn identifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::identifier, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.lookahead(false, |state| state.match_string("PUSH")).and_then(|state| state.match_string("_").or_else(|state| self::alpha(state))).and_then(|state| state.repeat(|state| state.match_string("_").or_else(|state| self::alpha_num(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn alpha(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z'))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn alpha_num(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::alpha(state).or_else(|state| state.match_range('0'..'9'))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::string, |state| state.sequence(|state| self::quote(state).and_then(|state| self::inner_str(state)).and_then(|state| self::quote(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn insensitive_string(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::insensitive_string, |state| state.sequence(|state| state.match_string("^").and_then(|state| super::hidden::skip(state)).and_then(|state| self::string(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn range(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::range, |state| state.sequence(|state| self::character(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::range_operator(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::character(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn character(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::character, |state| state.sequence(|state| self::single_quote(state).and_then(|state| self::inner_chr(state)).and_then(|state| self::single_quote(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn inner_str(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::inner_str, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                let strings = ["\"", "\\"];
                                state.skip_until(&strings).and_then(|state| state.optional(|state| state.sequence(|state| self::escape(state).and_then(|state| self::inner_str(state)))))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn inner_chr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::inner_chr, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::escape(state).or_else(|state| self::ANY(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn escape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::escape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("\\").and_then(|state| state.match_string("\"").or_else(|state| state.match_string("\\")).or_else(|state| state.match_string("r")).or_else(|state| state.match_string("n")).or_else(|state| state.match_string("t")).or_else(|state| state.match_string("0")).or_else(|state| state.match_string("'")).or_else(|state| self::code(state)).or_else(|state| self::unicode(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn code(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::code, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("x").and_then(|state| self::hex_digit(state)).and_then(|state| self::hex_digit(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn unicode(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::unicode, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("u").and_then(|state| self::opening_brace(state)).and_then(|state| state.sequence(|state| self::hex_digit(state).and_then(|state| self::hex_digit(state)).and_then(|state| state.optional(|state| self::hex_digit(state))).and_then(|state| state.optional(|state| self::hex_digit(state))).and_then(|state| state.optional(|state| self::hex_digit(state))).and_then(|state| state.optional(|state| self::hex_digit(state))))).and_then(|state| self::closing_brace(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn hex_digit(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::hex_digit, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn quote(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::quote, |state| state.match_string("\""))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn single_quote(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::single_quote, |state| state.match_string("'"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn range_operator(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::range_operator, |state| state.match_string(".."))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::WHITESPACE, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::SPACE_SEPARATOR(state).or_else(|state| self::NEWLINE(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn block_comment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::block_comment, |state| state.sequence(|state| state.match_string("/*").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::block_comment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("*/")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::block_comment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("*/")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("*/"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::COMMENT, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::block_comment(state).or_else(|state| state.sequence(|state| state.match_string("//").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state)))))))))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SPACE_SEPARATOR)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::grammar_rules => rules::grammar_rules(state),
            Rule::grammar_rule => rules::grammar_rule(state),
            Rule::assignment_operator => rules::assignment_operator(state),
            Rule::opening_brace => rules::opening_brace(state),
            Rule::closing_brace => rules::closing_brace(state),
            Rule::opening_paren => rules::opening_paren(state),
            Rule::closing_paren => rules::closing_paren(state),
            Rule::opening_brack => rules::opening_brack(state),
            Rule::closing_brack => rules::closing_brack(state),
            Rule::modifier => rules::modifier(state),
            Rule::silent_modifier => rules::silent_modifier(state),
            Rule::atomic_modifier => rules::atomic_modifier(state),
            Rule::compound_atomic_modifier => rules::compound_atomic_modifier(state),
            Rule::non_atomic_modifier => rules::non_atomic_modifier(state),
            Rule::expression => rules::expression(state),
            Rule::term => rules::term(state),
            Rule::node => rules::node(state),
            Rule::terminal => rules::terminal(state),
            Rule::prefix_operator => rules::prefix_operator(state),
            Rule::infix_operator => rules::infix_operator(state),
            Rule::postfix_operator => rules::postfix_operator(state),
            Rule::positive_predicate_operator => rules::positive_predicate_operator(state),
            Rule::negative_predicate_operator => rules::negative_predicate_operator(state),
            Rule::sequence_operator => rules::sequence_operator(state),
            Rule::choice_operator => rules::choice_operator(state),
            Rule::optional_operator => rules::optional_operator(state),
            Rule::repeat_operator => rules::repeat_operator(state),
            Rule::repeat_once_operator => rules::repeat_once_operator(state),
            Rule::repeat_exact => rules::repeat_exact(state),
            Rule::repeat_min => rules::repeat_min(state),
            Rule::repeat_max => rules::repeat_max(state),
            Rule::repeat_min_max => rules::repeat_min_max(state),
            Rule::number => rules::number(state),
            Rule::integer => rules::integer(state),
            Rule::comma => rules::comma(state),
            Rule::_push => rules::_push(state),
            Rule::peek_slice => rules::peek_slice(state),
            Rule::identifier => rules::identifier(state),
            Rule::alpha => rules::alpha(state),
            Rule::alpha_num => rules::alpha_num(state),
            Rule::string => rules::string(state),
            Rule::insensitive_string => rules::insensitive_string(state),
            Rule::range => rules::range(state),
            Rule::character => rules::character(state),
            Rule::inner_str => rules::inner_str(state),
            Rule::inner_chr => rules::inner_chr(state),
            Rule::escape => rules::escape(state),
            Rule::code => rules::code(state),
            Rule::unicode => rules::unicode(state),
            Rule::hex_digit => rules::hex_digit(state),
            Rule::quote => rules::quote(state),
            Rule::single_quote => rules::single_quote(state),
            Rule::range_operator => rules::range_operator(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::block_comment => rules::block_comment(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
