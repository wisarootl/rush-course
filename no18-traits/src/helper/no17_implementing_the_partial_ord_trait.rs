#![allow(unused)]

use std::cmp::Ordering;

struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.salary.partial_cmp(&other.salary)

        // if self.salary == other.salary {
        //     Some(Ordering::Equal)
        // } else if self.salary < other.salary {
        //     Some(Ordering::Less)
        // } else if self.salary > other.salary {
        //     Some(Ordering::Greater)
        // } else {
        //     None
        // }
    }
}

pub fn main() {
    println!("=== no17_implementing_the_partial_ord_trait ===");

    // partial ord is from partial order
    // so we can use <, >, <=, >=, sort etc.

    let long_commute_job = Job {
        salary: 100000,
        commute_time: 2,
    };

    let short_commute_job = Job {
        salary: 75000,
        commute_time: 1,
    };

    println!("{}", long_commute_job > short_commute_job);
    println!("{}", long_commute_job < short_commute_job);
    println!("{}", long_commute_job == short_commute_job);
    println!("{}", long_commute_job >= short_commute_job);
    println!("{}", long_commute_job <= short_commute_job);

    let new_opportunity = Job {
        salary: 100000,
        commute_time: 1,
    };

    println!("{}", long_commute_job > new_opportunity);
    println!("{}", long_commute_job < new_opportunity);

    println!("{}", long_commute_job == new_opportunity);
    println!("{}", long_commute_job != new_opportunity);

    println!("{}", long_commute_job >= new_opportunity);
    println!("{}", long_commute_job <= new_opportunity);
}
