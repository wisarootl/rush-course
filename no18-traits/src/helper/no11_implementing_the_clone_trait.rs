use std::clone::Clone;

#[derive(Clone, Debug)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

// impl Clone for Appointment {
//     fn clone(&self) -> Self {
//         println!("Cloning Appointment");

//         Self {
//             doctor: self.doctor.clone(),
//             start_time: self.start_time.clone(),
//             end_time: self.end_time.clone(),
//         }
//     }
// }

pub fn main() {
    println!("=== no11_implementing_the_clone_trait ===");
    let morning_appt = Appointment::new("Dr. Andrews", "9:00AM", "10:00AM");
    let replacement_appt = morning_appt.clone();
    println!(
        "{} is seeing the patient from {} to {}",
        replacement_appt.doctor, replacement_appt.start_time, replacement_appt.end_time
    );
    println!("{morning_appt:?}");
}
