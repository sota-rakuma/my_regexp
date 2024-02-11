use std::vec;

use crate::{matcher::{executor::backtracer::BackTracer, nfa::builder::thompson::ThompsonWayBuilder, Matcher}, parser::ll0_parser::LL0Parser};

use super::create_nfa;

#[test]
fn empty_test() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "");
    let matcher = BackTracer::new(nfa); 

    let raw = "";
    let expected: Vec<String> = vec![""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "a";
    let expected: Vec<String> = vec!["", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaa";
    let expected: Vec<String> = vec!["", "", "", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn only_char_test() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "a");
    let matcher = BackTracer::new(nfa);

    let raw = "";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    //
    let raw = "a";
    let expected: Vec<String> = vec!["a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aa";
    let expected: Vec<String> = vec!["a", "a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaaa";
    let expected: Vec<String> = vec!["a", "a", "a", "a", "a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "babab";
    let expected: Vec<String> = vec!["a", "a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "bbb";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "bbba";
    let expected: Vec<String> = vec!["a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn only_char_test_with_regexp() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "((a))");
    let matcher = BackTracer::new(nfa);

    let raw = "";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "a";
    let expected: Vec<String> = vec!["a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aa";
    let expected: Vec<String> = vec!["a", "a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaaa";
    let expected: Vec<String> = vec!["a", "a", "a", "a", "a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "babab";
    let expected: Vec<String> = vec!["a", "a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "bbb";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "bbba";
    let expected: Vec<String> = vec!["a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn concat_test() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "aaa");
    let matcher = BackTracer::new(nfa);

    let raw = "";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aa";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaa";
    let expected: Vec<String> = vec!["aaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaaa";
    let expected: Vec<String> = vec!["aaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaaaa";
    let expected: Vec<String> = vec!["aaa", "aaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aabaa";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn concat_test_with_regexp() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "(a)a((a))");
    let matcher = BackTracer::new(nfa);

    let raw = "";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aa";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaa";
    let expected: Vec<String> = vec!["aaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaaa";
    let expected: Vec<String> = vec!["aaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaaaa";
    let expected: Vec<String> = vec!["aaa", "aaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aabaa";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn altanative_test() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "aaa|bbb");
    let matcher = BackTracer::new(nfa);
    
    let raw = "";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aa";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaa";
    let expected: Vec<String> = vec!["aaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "bbb";
    let expected: Vec<String> = vec!["bbb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
    
    let raw = "aaabbb";
    let expected: Vec<String> = vec!["aaa", "bbb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aabb";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaabb";
    let expected: Vec<String> = vec!["aaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aabbb";
    let expected: Vec<String> = vec!["bbb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "baa";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aab";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "baab";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "abba";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn altanative_test_with_regexp() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "a(aa|b)bb");
    let matcher = BackTracer::new(nfa);

    let raw = "";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "a";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aa";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaa";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "b";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "bb";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "bbb";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aabb";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaab";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "abb";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aab";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaabb";
    let expected: Vec<String> = vec!["aaabb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "abbb";
    let expected: Vec<String> = vec!["abbb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaabb";
    let expected: Vec<String> = vec!["aaabb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "abbbb";
    let expected: Vec<String> = vec!["abbb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaabbbb";
    let expected: Vec<String> = vec!["aaabb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "babbbaaabbb";
    let expected: Vec<String> = vec!["abbb", "aaabb"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

}

#[test]
fn kleene_test() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "a*");
    let matcher = BackTracer::new(nfa); 

    let raw = "";
    let expected: Vec<String> = vec![""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "a";
    let expected: Vec<String> = vec!["a", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaa";
    let expected: Vec<String> = vec!["aaaa", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "baab";
    let expected: Vec<String> = vec!["", "aa", "", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aabaaba";
    let expected: Vec<String> = vec!["aa", "", "aa", "", "a", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn kleene_test_with_following() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "a*a");
    let matcher = BackTracer::new(nfa); 

    let raw = "";
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "a";
    let expected: Vec<String> = vec!["a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aaaa";
    let expected: Vec<String> = vec!["aaaa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "baab";
    let expected: Vec<String> = vec!["aa"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "aabaaba";
    let expected: Vec<String> = vec!["aa", "aa", "a"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn recursive_kleene() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "1.*0.*0.*1");
    let matcher = BackTracer::new(nfa);

    let raw = "101010101";
    let expected = vec!["101010101"];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);
}

#[test]
fn complex_test() {
    let builder = ThompsonWayBuilder::new();
    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "<.*>.*</.*>");
    let matcher = BackTracer::new(nfa);

    let raw = "<html><h1>aaa</h1></html>";
    let expected = vec!["<html><h1>aaa</h1></html>"];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "<h1>aaa</h1>";
    let expected: Vec<String> = vec!["<h1>aaa</h1>"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "<h1><h1>"; // no match
    let expected: Vec<String> = vec![];
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "(0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*");
    let matcher = BackTracer::new(nfa);

    let raw = "1";
    let expected: Vec<String> = vec!["1"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "12345543";
    let expected: Vec<String> = vec!["12345543"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "10a2f4f";
    let expected: Vec<String> = vec!["10", "2", "4"].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);


    let nfa = create_nfa(&builder, &mut LL0Parser::new(), "abc*(d.*e)|(fg*|hi)*");
    let matcher = BackTracer::new(nfa);

    let raw = "";
    let expected: Vec<String> = vec![""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "abde";
    let expected: Vec<String> = vec!["abde", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "abdfdasfdsafe abdfdasfdsafeabdfdasfdsafe";
    let expected: Vec<String> = vec!["abdfdasfdsafe abdfdasfdsafeabdfdasfdsafe", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "f";
    let expected: Vec<String> = vec!["f", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "fggggggggggfgfffffffff";
    let expected: Vec<String> = vec!["fggggggggggfgfffffffff", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "hihihi";
    let expected: Vec<String> = vec!["hihihi", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

    let raw = "abc"; // not match
    let expected: Vec<String> = vec!["", "", "", ""].into_iter().map(|v| v.to_string()).collect();
    let actual = matcher.exec(raw);
    assert_eq!(expected, actual);

}