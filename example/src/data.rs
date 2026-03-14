//! Sample data and column definitions for the example.

use yew_datatable::prelude::{ColumnDef, ColumnDefBuilder};

/// Sample employee data used in all example demonstrations.
///
/// Represents a person with identifying information, employment
/// details, and an active status flag.
#[derive(Clone, PartialEq)]
pub struct Person {
    /// Unique employee identifier.
    pub id: usize,

    /// First name of the employee.
    pub first_name: String,

    /// Last name of the employee.
    pub last_name: String,

    /// Age of the employee in years.
    pub age: u32,

    /// Corporate email address.
    pub email: String,

    /// Department name.
    pub department: String,

    /// Annual salary in USD.
    pub salary: f64,

    /// Whether the employee is currently active.
    pub active: bool,
}

/// Generates 100 sample employee records for demonstration.
///
/// # Returns
///
/// - `Vec<Person>`: A vector of 100 deterministically generated persons.
pub fn generate_sample_data() -> Vec<Person> {
    // Define the pool of first names.
    let first_names = [
        "Alice", "Bob", "Carol", "David", "Eve", "Frank", "Grace", "Henry", "Ivy", "Jack", "Karen", "Leo", "Mia",
        "Nathan", "Olivia", "Peter", "Quinn", "Rachel", "Sam", "Tina", "Uma", "Victor", "Wendy", "Xavier", "Yuki",
        "Zach", "Amelia", "Ben", "Clara", "Daniel", "Emma", "Felix", "Gina", "Hugo", "Iris", "James", "Kira", "Liam",
        "Maya", "Noah", "Oscar", "Pam", "Raj", "Sofia", "Tom", "Uma", "Vera", "Will", "Xena", "Yara",
    ];

    // Define the pool of last names.
    let last_names = [
        "Johnson",
        "Smith",
        "Williams",
        "Brown",
        "Davis",
        "Miller",
        "Wilson",
        "Moore",
        "Taylor",
        "Anderson",
        "Thomas",
        "Jackson",
        "White",
        "Harris",
        "Martin",
        "Thompson",
        "Garcia",
        "Martinez",
        "Robinson",
        "Clark",
        "Rodriguez",
        "Lewis",
        "Lee",
        "Walker",
        "Hall",
        "Allen",
        "Young",
        "King",
        "Wright",
        "Scott",
        "Green",
        "Baker",
        "Adams",
        "Nelson",
        "Hill",
        "Campbell",
        "Mitchell",
        "Roberts",
        "Carter",
        "Phillips",
        "Evans",
        "Turner",
        "Torres",
        "Parker",
        "Collins",
        "Edwards",
        "Stewart",
        "Flores",
        "Morris",
        "Murphy",
    ];

    // Define the pool of department names.
    let departments = [
        "Engineering",
        "Marketing",
        "Sales",
        "HR",
        "Finance",
        "Operations",
        "Legal",
        "Support",
    ];

    // Pre-allocate the result vector.
    let mut data = Vec::with_capacity(100);

    // Generate 100 deterministic employee records.
    for i in 0..100 {
        // Select names and department using deterministic index rotation.
        let first_idx = i % first_names.len();
        let last_idx = (i * 7 + 3) % last_names.len();
        let dept_idx = i % departments.len();

        let first_name = first_names[first_idx];
        let last_name = last_names[last_idx];
        let department = departments[dept_idx];

        // Calculate base salary from department.
        let base_salary = match department {
            "Engineering" => 90000.0,
            "Finance" => 85000.0,
            "Legal" => 95000.0,
            "Marketing" => 70000.0,
            "Sales" => 65000.0,
            "HR" => 60000.0,
            "Operations" => 55000.0,
            "Support" => 50000.0,
            _ => 60000.0,
        };

        // Calculate age and salary with experience-based bonus.
        let age = 22 + (i * 3 + 5) % 40;
        let experience_bonus = ((age as f64 - 22.0) / 40.0) * 50000.0;
        let salary = base_salary + experience_bonus + (((i * 17) % 20000) as f64);

        // Construct and push the employee record.
        data.push(Person {
            id: i + 1,
            first_name: first_name.into(),
            last_name: last_name.into(),
            age: age as u32,
            email: format!("{}.{}@company.com", first_name.to_lowercase(), last_name.to_lowercase()),
            department: department.into(),
            salary: (salary * 100.0).round() / 100.0,
            active: i % 7 != 0,
        });
    }

    data
}

/// Creates the column definitions for the Person data type.
///
/// # Returns
///
/// - `Vec<ColumnDef<Person>>`: A vector of eight column definitions covering
///   all fields of the `Person` struct.
pub fn create_columns() -> Vec<ColumnDef<Person>> {
    vec![
        ColumnDefBuilder::new("id", "ID")
            .accessor(|p: &Person| p.id as i32)
            .default_width(60.0)
            .build(),
        ColumnDefBuilder::new("first_name", "First Name")
            .accessor(|p: &Person| p.first_name.clone())
            .build(),
        ColumnDefBuilder::new("last_name", "Last Name")
            .accessor(|p: &Person| p.last_name.clone())
            .build(),
        ColumnDefBuilder::new("age", "Age")
            .accessor(|p: &Person| p.age as i32)
            .default_width(80.0)
            .build(),
        ColumnDefBuilder::new("email", "Email")
            .accessor(|p: &Person| p.email.clone())
            .build(),
        ColumnDefBuilder::new("department", "Department")
            .accessor(|p: &Person| p.department.clone())
            .build(),
        ColumnDefBuilder::new("salary", "Salary")
            .accessor(|p: &Person| p.salary)
            .default_width(100.0)
            .build(),
        ColumnDefBuilder::new("active", "Active")
            .accessor(|p: &Person| if p.active { "Yes".to_string() } else { "No".to_string() })
            .default_width(80.0)
            .build(),
    ]
}
