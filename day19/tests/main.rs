#[cfg(test)]
mod tests {

use std::fs;
use day19::*;

#[test]
pub fn test() {
    let mut rulemap = RuleMap::new();

    rulemap.parse(
       r#"0: 1 2 3
       1: "a"
       2: 4 5 | 5 4
       3: "b"
       4: 1 1 3
       5: 3 1 1"#);

    assert_eq!(rulemap.try_match("aaabbaab"), true);
    assert_eq!(rulemap.try_match("aaabbaaa"), false);
}

#[test]
pub fn p2_test_no_override() {
    let contents = fs::read_to_string("testcase.txt")
        .expect("bleh");
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let count = count_matches(parts[0], parts[1], false);
    assert_eq!(3, count);
}

#[test]
pub fn p2_test_with_override() {
    let contents = fs::read_to_string("testcase.txt")
        .expect("bleh");
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let count = count_matches(parts[0], parts[1], true);
    assert_eq!(12, count);
}

#[test]
pub fn p2_test_with_override_detailed() {
    // 
    // bbabbbbaabaabba
    // babbbbaabbbbbabbbbbbaabaaabaaa
    // aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    // bbbbbbbaaaabbbbaaabbabaaa
    // bbbababbbbaaaaaaaabbababaaababaabab
    // ababaaaaaabaaab
    // ababaaaaabbbaba
    // baabbaaaabbaaaababbaababb
    // abbbbabbbbaaaababbbbbbaaaababb
    // aaaaabbaabaaaaababaa
    // aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    // aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
    //
    let contents = fs::read_to_string("testcase.txt")
        .expect("bleh");
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let mut rulemap = RuleMap::new();
    rulemap.parse(parts[0]);
    rulemap.parse("8: 42 | 42 8\n11: 42 31 | 42 11 31");
    assert_eq!(true, rulemap.try_match(
        "bbabbbbaabaabba"));
    // this one:
    assert_eq!(true, rulemap.try_match(
        "babbbbaabbbbbabbbbbbaabaaabaaa"));
    assert_eq!(true, rulemap.try_match(
        "aaabbbbbbaaaabaababaabababbabaaabbababababaaa"));
    assert_eq!(true, rulemap.try_match(
        "bbbbbbbaaaabbbbaaabbabaaa"));
    assert_eq!(true, rulemap.try_match(
        "bbbababbbbaaaaaaaabbababaaababaabab"));
    assert_eq!(true, rulemap.try_match(
        "ababaaaaaabaaab"));
    assert_eq!(true, rulemap.try_match(
        "ababaaaaabbbaba"));
    assert_eq!(true, rulemap.try_match(
        "baabbaaaabbaaaababbaababb"));
    assert_eq!(true, rulemap.try_match(
        "abbbbabbbbaaaababbbbbbaaaababb"));
    assert_eq!(true, rulemap.try_match(
        "aaaaabbaabaaaaababaa"));
    assert_eq!(true, rulemap.try_match(
        "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"));
    assert_eq!(true, rulemap.try_match(
        "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"));
}

#[test]
pub fn simple_loop_test() {
    let contents = fs::read_to_string("simple_loop.txt")
        .expect("bleh");
    let mut rulemap = RuleMap::new();
    rulemap.parse(&contents);
    dbg!(&rulemap);
    //                                  0123456789
    assert_eq!(true, rulemap.try_match("aaaaaabbbbbbab"));
}

}  // mod tests
