/// The PartialOrd trait indicates that a type can be ordered/sorted
/// trait partial Order
/// we can implement this trait on a type to indicate that type support the concept of order or comparison
/// with this trait implemented, we can allow our types to use symbols like
/// greater than
/// less than
/// greater than or equal to
/// less than or equal to
/// it also means that we can sort a collection of that type in order, because the type now support
/// the ideal of being ordered, of being placed inline in comparison with other value of the same type.
///
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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.salary.partial_cmp(&other.salary)
        // this is happen because  type u32 has PartialOrd trait

        /*        match self.salary.partial_cmp(&other.salary){
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Less) => Some(Ordering::Less),
            None => None
        }*/

        /*        if self.salary == other.salary {
            Some(Ordering::Equal)
        } else if self.salary > other.salary {
            Some(Ordering::Greater)
        } else if self.salary < other.salary {
            Some(Ordering::Less)
        } else {
            None
        }*/
    }
}
fn main() {
    let long_commute_job = Job {
        salary: 100000,
        commute_time: 2,
    };
    let short_commute_job = Job {
        salary: 75000,
        commute_time: 1,
    };
    println!("compare Greater {}", long_commute_job > short_commute_job);
    println!("compare smaller {}", long_commute_job < short_commute_job);
    println!("compare Equal {}", long_commute_job == short_commute_job);
    println!(
        "compare Greater or Equal {}",
        long_commute_job >= short_commute_job
    );
    println!(
        "compare smaller or Equal {}",
        long_commute_job <= short_commute_job
    );
    println!(
        "compare Not Equal {}",
        long_commute_job != short_commute_job
    );

    println!(
        "compare {:?}",
        long_commute_job.partial_cmp(&short_commute_job)
    );
}
