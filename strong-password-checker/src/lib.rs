/// A password is considered strong if the below conditions are all met:
///
/// - It has at least 6 characters and at most 20 characters.
/// - It contains at least one lowercase letter, at least one uppercase letter, and at least one digit.
/// - It does not contain three repeating characters in a row (i.e., "Baaabb0" is weak, but "Baaba0" is strong).
///
/// Given a string password, return the minimum number of steps required to make password strong. if password is already strong, return 0.
///
/// In one step, you can:
///
/// - Insert one character to password,
/// - Delete one character from password, or
/// - Replace one character of password with another character.
pub fn strong_password_checker(password: String) -> i32 {
    let password = password.chars().collect::<Vec<_>>();

    // check how many of lower/upper/number we still need to satisfy
    let mut needs_lower = true;
    let mut needs_upper = true;
    let mut needs_number = true;

    for c in password.iter() {
        match c {
            'a'..='z' => needs_lower = false,
            'A'..='Z' => needs_upper = false,
            '0'..='9' => needs_number = false,
            '.' | '!' => continue,
            _ => unreachable!()
        }
    }

    let mut requirements = needs_lower as u32 + needs_upper as u32 + needs_number as u32;

    // check how many runs of 3 we need to break
    let mut unbroken_runs = 0_u32;

    if password.len() > 2 {
        let mut i = 0;

        while i < password.len() - 2 {
            if password[i] == password[i + 1] && password[i] == password[i + 2] {
                unbroken_runs += 1;
                i += 3;
                continue;
            }

            i += 1;
        }
    }

    // break as many runs as possible using required chars (if any)
    let broken_with_requirements = unbroken_runs.min(requirements);
    requirements -= broken_with_requirements;

    if password.len() < 6 {
        let mut to_add = 6 - password.len() as u32;

        // break remaining unbroken runs by adding chars in the middle of them
        let added_to_break_run = to_add.min(unbroken_runs - broken_with_requirements);
        to_add -= added_to_break_run;

        // add as many chars as possible using required chars (if any)
        let added_to_satisfy = to_add.min(requirements);
        to_add -= added_to_satisfy;

        return (to_add + requirements + unbroken_runs) as i32;
    }

    if password.len() > 20 {
        let mut to_remove = password.len() as u32 - 20;

        // break remaining unbroken runs by removing chars
        let removed_to_break_run = to_remove.min(unbroken_runs - broken_with_requirements);
        to_remove -= removed_to_break_run;

        // trim any excess chars after breaking runs and do replacements for remaining reqs and runs
        return (to_remove + requirements + unbroken_runs) as i32;
    }

    // password length is fine, so do replacements for remaining runs and/or requirements
    (unbroken_runs + requirements) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn already_valid() {
        assert_eq!(strong_password_checker("abcdE1".into()), 0)
    }

    #[rstest]
    #[case("", 6)]
    #[case("a", 5)]
    #[case("abcd", 2)]
    #[case("abcdE", 1)]
    #[case("abcde", 2)]
    fn add_simple(#[case] input: String, #[case] expected: i32) {
        let result = strong_password_checker(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("aaade", 2)]
    #[case("aaadE", 2)]
    #[case("aaaB1", 1)]
    fn add_with_runs(#[case] input: String, #[case] expected: i32) {
        let result = strong_password_checker(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("a1.!.!", 1)]
    #[case("a..!.!", 2)]
    #[case(".!.!.!", 3)]
    fn replace_simple(#[case] input: String, #[case] expected: i32) {
        let result = strong_password_checker(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("aaabb1", 1)]
    #[case("aaabbb", 2)]
    #[case("......", 3)]
    #[case("1111111111", 3)] // 1111111111 -> 11a11A11x1
    fn replace_with_runs(#[case] input: String, #[case] expected: i32) {
        let result = strong_password_checker(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("abcdefghijklmnopqrZ3a", 1)]
    #[case("abcdefghijklmnopqrZ3abc", 3)]
    #[case("abcdefghijklmnopqrZxa", 2)]
    #[case("abcdefghijklmnopqrZxabc", 4)]
    fn trim_simple(#[case] input: String, #[case] expected: i32) {
        let result = strong_password_checker(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("abcdefghijklmnopqrZ3aaa", 3)]
    #[case("abcdefghijklmnopqrZxaaa", 4)]
    #[case("abcdefghixxxmnopqrZxaaa", 4)]
    #[case("bbaaaaaaaaaaaaaaacccccc", 8)] // bbaaaaaaaaaaaaaaacccccc -> bbaa4aa5aa6aa7accCcc
    fn trim_with_runs(#[case] input: String, #[case] expected: i32) {
        let result = strong_password_checker(input);
        assert_eq!(result, expected);
    }
}
