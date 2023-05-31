//! `Fuzzy Match` is a fuzzy matching library based on the popular `FuzzyWuzzy` library for python.
//! It contains 4 basic fuzzy matching functions.

use std::{collections::HashSet};
use regex::Regex;

/// Uses the levenshtein distance to calculate the similarity of two strings.
///
/// # Example
///
/// ```
/// let str1 = "Hello";
/// let str2 = "Hallo";
/// let similiarity = fuzzy_match_flex::ratio(str1, str2, None);
///
/// assert_eq!(similiarity, 0.8);
/// ```
/// 
/// # Third parameter
/// 
/// `clean_str: Option<bool>` - tells the function if the strings were cleaned. Set to `false` if they ware and to `true` or `None` if they need to be cleaned.
/// 
pub fn ratio(str1 : &str, str2 : &str, clean_str: Option<bool>) -> f32 {
    if str1 == str2 {
        return 1.0;
    }

    let clean_str = clean_str.unwrap_or(true);

    let str1_cleaned = if clean_str { clean_string(str1) } else { str1.to_owned() };
    let str2_cleaned = if clean_str { clean_string(str2) } else { str2.to_owned() };

    levenshtein(&str1_cleaned, &str2_cleaned, 2)
}

/// Splits the longest string into tokens and uses the levenshtein distance to calculate the similarity of tokens and shorter string.
///
/// # Example
///
/// ```
/// let str1 = "Do we buy the airplane?";
/// let str2 = "Airplane";
/// let similiarity = fuzzy_match_flex::partial_ratio(str1, str2, None);
///
/// assert_eq!(similiarity, 1.0);
/// ```
/// 
/// # Third parameter
/// 
/// `clean_str: Option<bool>` - tells the function if the strings were cleaned. Set to `false` if they ware and to `true` or `None` if they need to be cleaned.
/// 
pub fn partial_ratio(str1 : &str, str2 : &str, clean_str: Option<bool>) -> f32 {
    if str1 == str2 {
        return 1.0;
    }

    let str1_len = str1.len();
    let str2_len = str2.len();

    let clean_str = clean_str.unwrap_or(true);

    let str1_cleaned = if clean_str { clean_string(str1) } else { str1.to_owned() };
    let str2_cleaned = if clean_str { clean_string(str2) } else { str2.to_owned() };
        
    let tokens = if str1_len > str2_len { str1_cleaned.split(' ') } else { str2_cleaned.split(' ') };

    let mut best_ratio : f32 = 0.0;
    let shortest_str : &str = if str1_len < str2_len { &str1_cleaned } else { &str2_cleaned };

    for token in tokens {
        best_ratio = best_ratio.max(levenshtein(token, shortest_str, 2));
    }

    best_ratio
 }

/// Splits both strings into tokens, sorts them and calculates the similarity between sorted words.
///
/// # Example
///
/// ```
/// let str1 = "My mom bought me ice cream";
/// let str2 = "The ice cream was bought by my mom";
/// let similiarity = fuzzy_match_flex::token_sort_ratio(str1, str2, None);
///
/// assert_eq!(similiarity, 0.8666667);
/// ```
/// 
/// # Third parameter
/// 
/// `clean_str: Option<bool>` - tells the function if the strings were cleaned. Set to `false` if they ware and to `true` or `None` if they need to be cleaned.
/// 
pub fn token_sort_ratio(str1 : &str, str2 : &str, clean_str: Option<bool>) -> f32 {
    if str1 == str2 {
        return 1.0;
     }

    let clean_str = clean_str.unwrap_or(true);

    let str1_cleaned = if clean_str { clean_string(str1) } else { str1.to_owned() };
    let str2_cleaned = if clean_str { clean_string(str2) } else { str2.to_owned() };

    let mut tokens_str1 : Vec<&str> = str1_cleaned.split(' ').collect();
    tokens_str1.sort();

    let mut tokens_str2 : Vec<&str> = str2_cleaned.split(' ').collect();
    tokens_str2.sort();

    let str1_low_sorted = tokens_str1.join(" ");
    let str2_low_sorted = tokens_str2.join(" ");

    levenshtein(&str1_low_sorted, &str2_low_sorted, 0)
}

/// Finds the intersection of both strings and calculates the similarity of strings.
///
/// # Example
///
/// ```
/// let str1 = "There are a lot of differences between Rust and C++";
/// let str2 = "differences in Rust C++";
/// let similiarity = fuzzy_match_flex::token_set_ratio(str1, str2, None);
///
/// assert_eq!(similiarity, 0.9230769);
/// ```
/// 
/// # Third parameter
/// 
/// `clean_str: Option<bool>` - tells the function if the strings were cleaned. Set to `false` if they ware and to `true` or `None` if they need to be cleaned.
/// 
pub fn token_set_ratio(str1 : &str, str2 : &str, clean_str: Option<bool>) -> f32 {
    if str1 == str2 {
        return 1.0;
    }

    let clean_str = clean_str.unwrap_or(true);

    let str1_cleaned = if clean_str { clean_string(str1) } else { str1.to_owned() };
    let str2_cleaned = if clean_str { clean_string(str2) } else { str2.to_owned() };

    let tokens_str1 : Vec<&str> = str1_cleaned.split(' ').collect();
    let tokens_str2 : Vec<&str> = str2_cleaned.split(' ').collect();
        
    let unique_tokens_str1: HashSet<&str> = tokens_str1.into_iter().collect();
    let unique_tokens_str2: HashSet<&str> = tokens_str2.into_iter().collect();

    let mut intersection : Vec<&str> = unique_tokens_str1.intersection(&unique_tokens_str2).copied().collect();
    let mut difference1 : Vec<&str> = unique_tokens_str1.difference(&unique_tokens_str2).copied().collect();
    let mut difference2 : Vec<&str> = unique_tokens_str2.difference(&unique_tokens_str1).copied().collect();

    intersection.sort();
    difference1.sort();
    difference2.sort();

    let sorted_intersection_str = intersection.join(" ");
    let sorted_difference1_str = difference1.join(" ");
    let sorted_difference2_str = difference2.join(" ");

    let combined_intersection_and_difference1 = format!("{} {}", sorted_intersection_str, sorted_difference1_str);
    let combined_intersection_and_difference2 = format!("{} {}", sorted_intersection_str, sorted_difference2_str);
        
    let ratios = vec![levenshtein(&sorted_intersection_str, &combined_intersection_and_difference1, 2),
                      levenshtein(&sorted_intersection_str, &combined_intersection_and_difference2, 2),
                      levenshtein(&combined_intersection_and_difference1, &combined_intersection_and_difference2, 2)];

    let mut max_ratio = ratios[0];
    for val in ratios.iter().skip(1){
        max_ratio = max_ratio.max(*val);
    };

    max_ratio
}

/// Levenshtein distance that calculates the similarity of two strings.
///
/// # Example
///
/// ```rust,ignore
/// let str1 = "hello";
/// let str2 = "hallo";
/// let similiarity = fuzzy_match_flex::levenshtein(str1, str2, 2);
///
/// assert_eq!(similiarity, 0.8);
/// ```
/// 
/// # Third parameter
/// 
/// `substitution_const: usize` - sets the cost of changing one letter into another.
/// 
fn levenshtein(str1 : &str, str2 : &str, substitution_const: usize) -> f32 {
    let rows = str1.len() + 1;
    let columns = str2.len() + 1;
    let mut distance: Vec<Vec<usize>> = vec![vec![0; columns]; rows];

    for i in 1..rows {
        distance[i][0] = i;
    }
        
    for i in 1..columns {
        distance[0][i] = i;
    }
        
    for i in 1..rows{
        for j in 1..columns{
            let cost = if str1.chars().nth(i-1) == str2.chars().nth(i-1) { 0 } else { substitution_const };

            distance[i][j] = (distance[i-1][j] + 1)
                            .min(distance[i][j-1] + 1)
                            .min(distance[i-1][j-1] + cost);
        }
    }

    let sum_length = (str1.len() + str2.len()) as f32;

    (sum_length - distance[rows-1][columns-1] as f32) / sum_length
}

/// Cleans the string leaving only lower case letters, numbers and one space between words.
///
/// # Example
///
/// ```rust,ignore
/// let str1 = "   It IS   imp^^^^0rtant";
/// let cleaned_str = fuzzy_match_flex::clean_string(str1);
///
/// assert_eq!(cleaned_str, "it is imp0rtant");
/// ```
fn clean_string(str : &str) -> String {
    let mut regex = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
    let cleared_str = regex.replace_all(&str, "").to_string().to_lowercase();

    regex = Regex::new(r"\s+").unwrap();
    regex.replace_all(cleared_str.trim(), " ").to_string()
}

/// Uses the levenshtein distance to calculate the similarity of two strings.
///
/// # Example
///
/// ```rust,ignore
/// let str1 = "Hello";
/// let str2 = "Hallo";
/// let similiarity = fuzzy_match_flex::ratio!(str1, str2);
///
/// assert_eq!(similiarity, 0.8);
/// ```
#[macro_export]
macro_rules! ratio {
    ($str1:expr, $str2:expr) => {
        fuzzy_match_flex::ratio($str1, $str2, None)
    };
}

/// Splits the longest string into tokens and uses the levenshtein distance to calculate the similarity of tokens and shorter string.
///
/// # Example
///
/// ```rust,ignore
/// let str1 = "Do we buy the airplane?";
/// let str2 = "Airplane";
/// let similiarity = fuzzy_match_flex::partial_ratio!(str1, str2);
///
/// assert_eq!(similiarity, 1.0);
/// ```
#[macro_export]
macro_rules! partial_ratio {
    ($str1:expr, $str2:expr) => {
        fuzzy_match_flex::partial_ratio($str1, $str2, None)
    };
}

/// Splits both strings into tokens, sorts them and calculates the similarity between sorted words.
///
/// # Example
///
/// ```rust,ignore
/// let str1 = "My mom bought me ice cream";
/// let str2 = "The ice cream was bought by my mom";
/// let similiarity = fuzzy_match_flex::token_sort_ratio!(str1, str2);
///
/// assert_eq!(similiarity, 0.8666667);
/// ```
#[macro_export]
macro_rules! token_sort_ratio {
    ($str1:expr, $str2:expr) => {
        fuzzy_match_flex::token_sort_ratio($str1, $str2, None)
    };
}

/// Finds the intersection of both strings and calculates the similarity of strings.
///
/// # Example
///
/// ```rust,ignore
/// let str1 = "There are a lot of differences between Rust and C++";
/// let str2 = "differences in Rust C++";
/// let similiarity = fuzzy_match_flex::token_set_ratio!(str1, str2);
///
/// assert_eq!(similiarity, 0.9230769);
/// ```
#[macro_export]
macro_rules! token_set_ratio {
    ($str1:expr, $str2:expr) => {
        fuzzy_match_flex::token_set_ratio($str1, $str2, None)
    };
}

 #[cfg(test)]
 mod tests {
    use super::*;

    #[test]
    fn clean_string_trims_string_and_deletes_non_alphanumerics() {
        let str = "  duck && cats";
        assert_eq!(clean_string(&str), "duck cats");
    }

    #[test]
    fn clean_string_deletes_repeaded_spaces_and_non_alphanumerics() {
        let str = "this1       is 4 u   :*   ";
        assert_eq!(clean_string(&str), "this1 is 4 u");
    }

    #[test]
    fn clean_string_turns_letters_into_lower_case() {
        let str = "THERE IS NO LOWERCASE LETTER IN THIS SENTENCE HAHAHA";
        assert_eq!(clean_string(&str), "there is no lowercase letter in this sentence hahaha");
    }

    #[test]
    fn clean_string_turns_letters_into_lower_case_delets_non_alphanumerics_and_trims() {
        let str = "  SUper Mega    H4Rd s**************tR1N$$$G";
        assert_eq!(clean_string(&str), "super mega h4rd str1ng");
    }

    #[test]
    fn levenshtein_no_spaces_sub_count_2() {
        let str1 = "haouse";
        let str2 = "home";
        let sum_length = str1.len() + str2.len();
        let result = (sum_length - 8) as f32 / sum_length as f32;

        assert_eq!(levenshtein(&str1, &str2, 2), result);
    }

    #[test]
    fn levenshtein_spaces_sub_count_1() {
        let str1 = "this is a string";
        let str2 = "that is a string";
        let sum_length = str1.len() + str2.len();
        let result = (sum_length - 2) as f32 / sum_length as f32;

        assert_eq!(levenshtein(&str1, &str2, 1), result);
    }

    #[test]
    fn ratio_words_with_spaces() {
        let str1 = "This is a string";
        let str2 = "That is a string";
        let sum_length = str1.len() + str2.len();
        let result = (sum_length - 4) as f32 / sum_length as f32;

        assert_eq!(ratio(&str1, &str2, None), result);
    }

    #[test]
    fn partial_ratio_words_with_spaces() {
        let str1 = "My Friend Builds :* Robots";
        let str2 = "Roboto";
        let sum_length = "Robots".len() + str2.len();
        let result = (sum_length - 2) as f32 / sum_length as f32;

        assert_eq!(partial_ratio(&str1, &str2, None), result);
    }

    #[test]
    fn token_sort_ratio_words_with_spaces() {
        let str1 = "Nasa landed a man on the moon";
        let str2 = "A man landed on the moon due to Nasa";
        let sum_length = str1.len() + str2.len();
        let result = (sum_length - 7) as f32 / sum_length as f32;

        assert_eq!(token_sort_ratio(&str1, &str2, None), result);
    }

    #[test]
    fn token_set_ratio_words_with_spaces() {
        let str1 = "The library has my favorite book its called rain & world";
        let str2 = "rain and world";
        let sum_length = 24;
        let result = (sum_length - 4) as f32 / sum_length as f32;

        assert_eq!(token_set_ratio(&str1, &str2, None), result);
    }
}