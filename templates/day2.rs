use std::collections::{HashMap, HashSet};

// write your functions here

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    // -- first_repeated_char --

    #[test]
    fn finds_first_repeated_character() {
        assert_eq!(first_repeated_char("swiss"), Some('s'));
    }

    #[test]
    fn repeated_char_returns_none_when_no_character_repeats() {
        assert_eq!(first_repeated_char("kite"), None);
    }

    #[test]
    fn repeated_char_handles_unicode_scalars() {
        assert_eq!(first_repeated_char("aébcé"), Some('é'));
    }

    #[test]
    fn repeated_char_handles_empty_input() {
        assert_eq!(first_repeated_char(""), None);
    }

    // -- first_unique_char --

    #[test]
    fn finds_first_unique_character() {
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

    // -- unique_chars_in_order --

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

    // -- sum_pair --

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

    // -- light_action --

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
