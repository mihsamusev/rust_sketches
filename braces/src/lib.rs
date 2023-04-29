use std::collections::HashMap;

fn valid_braces(s: &str) -> bool {
    let bracket = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    let mut stack: Vec<char> = Vec::new();

    if s.is_empty() {
        return true;
    }

    for c in s.chars() {
        // open bracket
        if bracket.contains_key(&c) {
            stack.push(*bracket.get(&c).unwrap());

        // closed bracket test
        } else if bracket.values().any(|x| *x == c) {
            if stack.last() != Some(&c) {
                stack.push(c);
                break;
            } else {
                stack.pop();
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        expect_true("()");
        expect_false("[(])");
        expect_false("(");
        expect_false(")");
    }

    fn expect_true(s: &str) {
        assert!(
            valid_braces(s),
            "Expected {s:?} to be valid. Got false",
            s = s
        );
    }

    fn expect_false(s: &str) {
        assert!(
            !valid_braces(s),
            "Expected {s:?} to be invalid. Got true",
            s = s
        );
    }
}
