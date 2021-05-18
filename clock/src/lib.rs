pub mod clock {
    use std::{
        cmp::Ordering, 
        fmt::{Display, Debug, Formatter, Result}
    };
    
    // Clock

    pub const MAX_MINUTES: i32 = 60;
    pub const MAX_HOURS: i32 = 24;
    pub const MAX_TOTAL_MINUTES: i32 = MAX_MINUTES * MAX_HOURS;

    #[derive(Debug, PartialEq)]
    pub struct Clock {
        total_minutes: i32
    }
    
    impl Clock {
        pub fn new(hours: i32, minutes: i32) -> Self {
            let total_minutes = (hours * 60 + minutes).rem_euclid(MAX_TOTAL_MINUTES);
    
            Clock { total_minutes }
        }
    
        pub fn add_minutes(&self, minutes: i32) -> Self {
            let new_total = (self.total_minutes + minutes).rem_euclid(MAX_TOTAL_MINUTES);
            Clock { total_minutes: new_total }
        }
    
        pub fn add_hours(&self, hours: i32) -> Self { // Extra - Unnecessary
            self.add_minutes(hours * 60)
        }

        // Getters

        pub fn get_minutes(&self) -> i32 { self.total_minutes.rem_euclid(MAX_MINUTES) }
        pub fn get_hours(&self) -> i32 { self.total_minutes.div_euclid(MAX_MINUTES) }
        pub fn get_time(&self) -> (i32, i32) { (self.get_hours(), self.get_minutes()) }
    }
    
    impl Display for Clock {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
            write!(formatter, "{:02}:{:02}", self.get_hours(), self.get_minutes())
        }
    }
    
    impl PartialOrd for Clock { // Extra - Unnecessary
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let hour_case = self.get_minutes().partial_cmp(&other.get_hours())?;
            let minute_case = self.get_minutes().partial_cmp(&other.get_minutes())?;
    
            let time_cases = [hour_case, minute_case];
            let mut result = Ordering::Equal;
            for ordering in time_cases.iter() {
                if *ordering != Ordering::Equal {
                    result = *ordering;
                    break;
                }
            }
            Some(result)
        }
    }
}

pub use crate::clock::Clock;