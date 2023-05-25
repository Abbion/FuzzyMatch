mod fuzzy_match{
    use std::{collections::HashSet, fmt::format};

    pub fn ratio(str1 : &str, str2 : &str, to_lower: Option<bool>) -> f32{
        let convert_to_loer = to_lower.unwrap_or(true);

        let str1_low = if convert_to_loer { str1.to_lowercase() } else { str1.to_owned() };
        let str2_low = if convert_to_loer { str2.to_lowercase() } else { str2.to_owned() };

        return levenshtein(&str1_low, &str2_low, 2);
    }

    pub fn partial_ratio(str1 : &str, str2 : &str, to_lower: Option<bool>) -> f32{
        if str1 == str2 {
            return 1.0;
        }

        let str1_len = str1.len();
        let str2_len = str2.len();

        let convert_to_loer = to_lower.unwrap_or(true);

        let str1_low = if convert_to_loer { str1.to_lowercase() } else { str1.to_owned() };
        let str2_low = if convert_to_loer { str2.to_lowercase() } else { str2.to_owned() };
        
        let tokens = if str1_len > str2_len { str1_low.split(' ') } else { str2_low.split(' ') };

        let mut best_ratio : f32 = 0.0;

        for token in tokens{
             let current_ratio = if str1_len > str2_len { levenshtein(token, &str2_low, 2) } else { levenshtein(token, &str1_low, 2) };
             best_ratio = best_ratio.max(current_ratio);
        }

        return best_ratio;
    }

    pub fn token_sort_ration(str1 : &str, str2 : &str, to_lower: Option<bool>) -> f32{
        if str1 == str2 {
            return 1.0;
        }

        let convert_to_loer = to_lower.unwrap_or(true);

        let str1_low = if convert_to_loer { str1.to_lowercase() } else { str1.to_owned() };
        let str2_low = if convert_to_loer { str2.to_lowercase() } else { str2.to_owned() };

        let mut tokens_str1 : Vec<&str> = str1_low.split(' ').collect();
        tokens_str1.sort();

        let mut tokens_str2 : Vec<&str> = str2_low.split(' ').collect();
        tokens_str2.sort();

        let str1_low_sorted = tokens_str1.join(" ");
        let str2_low_sorted = tokens_str2.join(" ");

        return levenshtein(&str1_low_sorted, &str2_low_sorted, 0);
    }

    pub fn token_set_ratio(str1 : &str, str2 : &str, to_lower: Option<bool>) -> f32{
        if str1 == str2 {
            return 1.0;
        }

        let convert_to_loer = to_lower.unwrap_or(true);

        let str1_low = if convert_to_loer { str1.to_lowercase() } else { str1.to_owned() };
        let str2_low = if convert_to_loer { str2.to_lowercase() } else { str2.to_owned() };

        let tokens_str1 : Vec<&str> = str1_low.split(' ').collect();
        let tokens_str2 : Vec<&str> = str2_low.split(' ').collect();
        
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

        let mut max = ratios[0];
        for val in ratios.iter().skip(1){
            if *val > max{
                max = *val;
            }
        };

        max
    }

    fn levenshtein(str1 : &str, str2 : &str, substitution_const: usize) -> f32{
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
        return (sum_length - distance[rows-1][columns-1] as f32) / sum_length;

    }
}

fn main() {
    //let a = fuzzy_match::ratio("This is a string", "That is a string", None);
    //let a = fuzzy_match::partial_ratio("Boston dynamic builds robots", "roboty", None);
    //let a = fuzzy_match::token_sort_ration("Nasa landed a man on the moon", "A man landed on the moon due to Nasa", None);
    let a = fuzzy_match::token_set_ratio("Todays match is the first match between Lakers and Galaxis", "Lakers v Galaxis", None);
    println!("{a}");
}
