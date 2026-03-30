use std::collections::{HashMap, HashSet};

// `use` brings a path into scope so you can refer to it with a shorter name
// python analogy: `from collections import defaultdict`
fn first_repeated_char(input: &str) -> Option<char> {
    let mut seen = HashSet::new();

    // loop through input.chars
    for ch in input.chars(){
        if seen.contains(&ch){
            return Some(ch);
        }
        seen.insert(ch);
    }
    None
}

fn first_unique_char(input: &str) -> Option<char> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for ch in input.chars(){
        // `entry(ch).or_insert(0)` is similar to Python:
        // `counts[ch] = counts.get(ch, 0) + 1`
        let count = counts.entry(ch).or_insert(0);
        *count += 1; 
    }

    for ch in input.chars(){
        // `get(&ch)` returns `Option<&usize>`, so we compare against `Some(&1)`.
        // `Some(...)` is needed because the key might not exist, and `&1` is
        // needed because `get` returns a reference to the stored value
        if counts.get(&ch) == Some(&1) {
            return Some(ch);
        }
    }
    None
}

fn unique_chars_in_order(input: &str) -> Vec<char> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for ch in input.chars(){
        let count = counts.entry(ch).or_insert(0);
        *count += 1;
    }

    let mut result = Vec::new();

    for ch in input.chars(){
        if counts.get(&ch) == Some(&1){
            result.push(ch);
        }
    }
    result
}

// `Result<T, E>` means the function returns either `Ok(T)` on success or
// `Err(E)` on failure. Here that is `Ok(i32)` or `Err(String)`.
// `split(',')` creates an iterator over comma-separated pieces.
// `next()` returns `Option<&str>`, so `ok_or(...)?` converts a missing piece
// into an `Err(...)` and returns early from the function
// `trim()` removes surrounding whitespace before parsing
// `parse()` tries to turn text into an `i32`
// `map_err(|_| ...)` replaces Rust's parse error with your own string message
// `?` unwraps `Ok(value)` and returns early on `Err(...)`

fn sum_pair(input: &str) -> Result<i32, String> {
    let mut parts = input.split(',');
    let left = parts.next().ok_or("expected two numbers".to_string())?;
    let right = parts.next().ok_or("expected two numbers".to_string())?;

    let left_num: i32 = left.trim().parse().map_err(|_| "invalid number".to_string())?;
    let right_num: i32 = right.trim().parse().map_err(|_| "invalid number".to_string())?;

    Ok(left_num + right_num)
}

// `enum` lets you define a type with a fixed set of variants
// `match` lets you branch on those variants explicitly
// This is similar to switching on a tagged value, but Rust requires you to handle every case
enum Light {
    Red,
    Yellow,
    Green
}

// Return the action a driver should take for each light
// `&'static str` means "a string slice that lives for the entire program"
// String literals like `"stop"` have this type, so it fits this function well
// rule of thumb:
// `&str` for reading borrowed text,
// `String` for owned or newly built text,
// `&'static str` for fixed string literals like `"stop"`
fn light_action(light: Light) -> &'static str {
    match light {
        Light::Red => "stop",
        Light::Yellow => "slow down",
        Light::Green => "go",
    }
}

fn main() {
    let word_examples = ["swiss", "alphabet", "kite", ""];
    let pair_examples = ["3,5", "10, 20", "7,x", "42"];
    let light_examples = [Light::Red, Light::Yellow, Light::Green];

    println!("first_repeated_char:");
    for word in word_examples {
        println!("{word:?} -> {:?}", first_repeated_char(word));
    }

    println!("first_unique_char:");
    for word in word_examples{
        println!("{word:?} -> {:?}", first_unique_char(word));
    }

    println!("unique_chars_in_order:");
    for word in word_examples {
        println!("{word:?} -> {:?}", unique_chars_in_order(word));
    }

    println!("sum_pair:");
    for pair in pair_examples {
        println!("{pair:?} -> {:?}", sum_pair(pair));
    }

    println!("light_action:");
    for light in light_examples {
        println!("{}", light_action(light));
    }
}

// only compile this module when running `cargo test`,
// In Python, the closest equivalent is usually keeping tests in separate
// `test_*.py` files and running them with a test runner like `pytest`
#[cfg(test)]
mod tests {
    use super::{
        first_repeated_char, first_unique_char, light_action, sum_pair,
        unique_chars_in_order, Light,
    };

    #[test]
    fn finds_first_repeated_character() {
        assert_eq!(first_repeated_char("swiss"), Some('s'));
    }

    #[test]
    fn repeated_char_returns_none_when_no_character_repeats() {
        assert_eq!(first_repeated_char("kite"), None);
    }

    #[test]
    fn repeated_char_handles_unicode_scalars(){
        assert_eq!(first_repeated_char("aébcé"), Some('é'));
    }

    #[test]
    fn repeated_char_handles_empty_input(){
        assert_eq!(first_repeated_char(""), None);
    }

    #[test]
    fn finds_first_unique_character(){
        assert_eq!(first_unique_char("swiss"), Some('w'));
    }

    #[test]
    fn unique_char_returns_none_when_everything_repeats() {
        assert_eq!(first_unique_char("aabb"), None);
    }

    #[test]
    fn unique_char_handles_unicode_scalars() {
        assert_eq!(first_unique_char("ééabc"), Some('a'));
    }

    #[test]
    fn unique_char_handles_empty_input() {
        assert_eq!(first_unique_char(""), None);
    }

    #[test]
    fn returns_unique_chars_in_original_order() {
        assert_eq!(unique_chars_in_order("swiss"), vec!['w', 'i']);
    }

    #[test]
    fn returns_empty_vec_when_everything_repeats() {
        assert_eq!(unique_chars_in_order("aabb"), Vec::<char>::new());
    }

    #[test]
    fn unique_chars_handles_unicode_scalars() {
        assert_eq!(unique_chars_in_order("ééabc"), vec!['a', 'b', 'c']);
    }

    #[test]
    fn unique_chars_handles_empty_input() {
        assert_eq!(unique_chars_in_order(""), Vec::<char>::new());
    }

    #[test]
    fn sums_two_numbers() {
        assert_eq!(sum_pair("3,5"), Ok(8));
    }

    #[test]
    fn trims_spaces_before_parsing() {
        assert_eq!(sum_pair("10, 20"), Ok(30));
    }

    #[test]
    fn returns_error_for_invalid_number() {
        assert_eq!(sum_pair("7,x"), Err("invalid number".to_string()));
    }

    #[test]
    fn returns_error_when_a_number_is_missing() {
        assert_eq!(sum_pair("42"), Err("expected two numbers".to_string()));
    }

    #[test]
    fn red_means_stop() {
        assert_eq!(light_action(Light::Red), "stop");
    }

    #[test]
    fn yellow_means_slow_down() {
        assert_eq!(light_action(Light::Yellow), "slow down");
    }

    #[test]
    fn green_means_go() {
        assert_eq!(light_action(Light::Green), "go");
    }
}