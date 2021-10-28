use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
pub struct Employee<'a, 'b> {
    pub id: u32,
    name: &'a str,
    surname: &'b str,
    pesel: u64,
    account_number: String,
    salary: f32,
}

impl<'a, 'b> Employee<'a, 'b> {
    pub const fn new(
        id: u32,
        name: &'a str,
        surname: &'b str,
        pesel: u64,
        account_number: String,
        salary: f32,
    ) -> Self {
        Self {
            id,
            name,
            surname,
            pesel,
            account_number,
            salary,
        }
    }
}

pub fn random_pesel() -> u64 {
    rand::thread_rng().gen_range(0u64..=99_99_99_99999)
}

pub fn random_account_number() -> String {
    rand::thread_rng()
        .gen_range(0u128..=9999_9999_9999_9999_9999_9999)
        .to_string()
}
