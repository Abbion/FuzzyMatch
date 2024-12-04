# fuzzy_match_flex [![Crates.io](https://img.shields.io/crates/v/fuzzy_match_flex?logo=Rust)](https://crates.io/crates/fuzzy_match_flex) ![GitHub](https://img.shields.io/github/license/Abbion/FuzzyMatchFlex)

**Fuzzy Match** is a library based on the popular **FuzzyWuzzy** library for python. It contains 4 basic fuzzy matching function. There are also four macros that take responsibility for cleaning strings.

# Match Examples

The substitution constant is set to $2$.

| Method | String_1 | String_2 | Result |
|------- | ------- | --------- | ------ |
| `ratio` | `This is a string` | `That is a string` | $0.875$ |
| `partial_ratio` | `Robert builds robots` | `roboty` | $0.833$ |

# Usage

Usage examples:

```rust
let str1 = "Hello";
let str2 = "Hallo";
let similiarity = fuzzy_match_flex::ratio(str1, str2, None);

assert_eq!(similiarity, 0.8);
```

`fuzzy_match_flex::ratio` uses the levenshtein distance to calculate the similarity of two strings. The third parameter tells the function if the strings were cleaned. Set to `Some(false)` if they ware and to `Some(true)` or `None` if they need to be cleaned.


```rust
let str1 = "Do we buy the airplane?";
let str2 = "Airplane";
let similiarity = fuzzy_match_flex::partial_ratio(str1, str2, None);

assert_eq!(similiarity, 1.0);
```

`fuzzy_match_flex::partial_ratio` splits the longest string into tokens and uses the levenshtein distance to calculate the similarity of tokens and shorter string.

```rust
let str1 = "My mom bought me ice cream";
let str2 = "The ice cream was bought by my mom";
let similiarity = fuzzy_match_flex::token_sort_ratio(str1, str2, None);

assert_eq!(similiarity, 0.8666667);
```

`fuzzy_match_flex::token_sort_ratio` splits both strings into tokens, sorts them and calculates the similarity between sorted words.

```rust
let str1 = "There are a lot of differences between Rust and C++";
let str2 = "differences in Rust C++";

let similiarity = fuzzy_match_flex::token_set_ratio(str1, str2, None);
assert_eq!(similiarity, 0.9230769);
```

`fuzzy_match_flex::token_set_ratio` finds the intersection of both strings and calculates the similarity of strings.

## Documentation

Documentation avaliable with command `cargo doc --open`
