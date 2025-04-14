/*
Elision rules are as follows: (Inferred to to omit lifetime annotation)

- The compiler assigns a lifetime to each parameter that is a reference.

- If there is one reference parameter and the return value is a reference,
  the borrow checker will infer that their lifetimes are related.

- If there are multiple reference parameters but one of them is self (instance),
  the borrow checker will assume the lifetime of the instance (self) is connected
  to the lifetime of the return value

Otherwise, it is an error to elide an output lifetime.
*/

#[derive(Clone)]
struct DentistAppointment {
    doctor: String,
}

impl DentistAppointment {
    // - third Elision rule.
    fn book1(&self, check_in_time: &str, check_out_time: &str) -> &str {
        println!(
            "You are booked from {} to {} with doctor {}",
            check_in_time, check_out_time, self.doctor
        );
        &self.doctor
    }

    // - overriding third Elision rule.
    fn book<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
        println!(
            "You are booked from {} to {} with doctor {}",
            check_in_time, check_out_time, self.doctor
        );
        check_in_time
    }
}

pub fn main() {
    println!("=== no11_lifetime_elision_rules_ii ===");

    let appt = DentistAppointment {
        doctor: String::from("David"),
    };
    let result = appt.book1("03:00PM", "11:00AM");
    // drop(appt);
    println!("{result}"); // if we drop `appt`, result cannot be used here.
    // from inferred lifetime by third rule: result lifetime will related to appt

    let appt = DentistAppointment {
        doctor: String::from("David"),
    };
    let result = appt.book("03:00PM", "11:00AM");
    drop(appt);
    println!("{result}");
}
