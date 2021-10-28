use rand::{
    distributions::{Slice, Uniform},
    prelude::Distribution,
    Rng,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Employee<'a, 'b> {
    pub id: usize,
    name: &'a str,
    surname: &'b str,
    pesel: u64,
    account_number: String,
    salary: f32,
}

impl<'a, 'b> Employee<'a, 'b> {
    pub const fn new(
        id: usize,
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

pub fn expand_employee_vec(
    employees: &mut Vec<Employee>,
    count: usize,
    names: &Vec<&'static str>,
    surnames: &Vec<&'static str>,
    salary_min: f32,
    salary_max: f32,
) {
    let mut rng = rand::thread_rng();
    let name_distribution = Slice::new(names).unwrap();
    let surname_distribution = Slice::new(surnames).unwrap();
    let salary_distribution = Uniform::new(salary_min, salary_max);

    for _ in 0..count {
        employees.push(Employee::new(
            employees.len(),
            name_distribution.sample(&mut rng),
            surname_distribution.sample(&mut rng),
            random_pesel(),
            random_account_number(),
            salary_distribution.sample(&mut rng),
        ))
    }
}

fn random_pesel() -> u64 {
    rand::thread_rng().gen_range(0u64..=99_99_99_99999)
}

fn random_account_number() -> String {
    rand::thread_rng()
        .gen_range(0u128..=9999_9999_9999_9999_9999_9999)
        .to_string()
}
