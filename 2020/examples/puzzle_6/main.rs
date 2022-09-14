use std::collections::HashSet;
use std::time::Instant;

/* Count unique questions per group and then sum */
fn part_1(contents: &str) {
    let mut total_questions: i32 = 0;
    let mut questions_answered_by_group: HashSet<char> = HashSet::new();
    for line in contents.lines() {
        if line.len() == 0 {
            total_questions += questions_answered_by_group.len() as i32;
            questions_answered_by_group = HashSet::new();
        } else {
            for question in line.chars() {
                questions_answered_by_group.insert(question);
            }
        }
    }
    // deal with final group
    total_questions += questions_answered_by_group.len() as i32;

    println!("Answer to part 1 is: {}", total_questions);
}

/* Count questions per group answered by all of group, then sum. */
fn part_2(contents: &str) {
    let mut total_questions: i32 = 0;
    let mut questions_answered_by_all: HashSet<char> = HashSet::new();
    let mut first_in_group: bool = true;

    for line in contents.lines() {
        if line.len() == 0 {
            total_questions += questions_answered_by_all.len() as i32;
            questions_answered_by_all = HashSet::new();
            first_in_group = true;
        } else if !first_in_group && questions_answered_by_all.len() == 0 {
            // already no common questions for people in this group
            continue;
        } else {
            first_in_group = false;
            let mut questions_answered_by_person = HashSet::new();
            for question in line.chars() {
                questions_answered_by_person.insert(question);
            }

            if questions_answered_by_all.len() == 0 {
                questions_answered_by_all = questions_answered_by_person
            } else {
                questions_answered_by_all = questions_answered_by_all
                    .into_iter()
                    .filter(|e| questions_answered_by_person.contains(e))
                    .collect();
            }
        }
    }
    // deal with final group
    total_questions += questions_answered_by_all.len() as i32;

    println!("Answer to part 2 is: {}", total_questions);
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("./input.txt");
    part_1(contents);
    part_2(contents);
    let duration = start.elapsed();
    println!("Took {:?} to solve this puzzle", duration);
}
