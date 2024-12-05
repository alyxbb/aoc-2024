use crate::Solution;

fn parse(input: String) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let stringed: Vec<_> = rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .collect();

    let mut rule_res = vec![];
    for line in stringed {
        rule_res.push((line.0.parse().unwrap(), line.1.parse().unwrap()));
    }

    let page_res = pages
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    (rule_res, page_res)
}

fn check_valid(book: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    for (i, page) in book.iter().enumerate() {
        for rule in rules {
            if rule.0 == *page {
                let Some(index) = book.iter().position(|n| *n == rule.1) else {
                    continue;
                };
                if index < i {
                    return false;
                }
            }
        }
    }
    true
}

fn sort_book(book: &Vec<usize>, rules: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_book = vec![];
    let mut old_book = book.clone();
    while old_book.len() != 0 {
        for (i, page) in old_book.iter().enumerate() {
            let mut valid = true;
            for rule in rules {
                if rule.0 == *page {
                    if old_book.contains(&rule.1) {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                new_book.insert(0, *page);
                old_book.remove(i);
                break;
            }
        }
    }
    new_book
}

pub fn part_1(input: String) -> Solution {
    let (rules, books) = parse(input);

    let mut sol = 0;
    for book in books {
        if check_valid(&book, &rules) {
            sol += book.get(book.len() / 2).unwrap();
        }
    }

    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let (rules, books) = parse(input);

    let mut sol = 0;
    for book in books {
        if !check_valid(&book, &rules) {
            let sorted_book = sort_book(&book, &rules);
            sol += sorted_book.get(book.len() / 2).unwrap();
        }
    }

    Solution::from(sol)
}
