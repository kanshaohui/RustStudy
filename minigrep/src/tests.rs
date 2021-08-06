use super::*;
#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "dUct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
}