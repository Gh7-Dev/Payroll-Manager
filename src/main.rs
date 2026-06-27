use std::collections::HashMap;
struct Employee{
    name:String,
    department:Department,
    salary: f64,
    is_active:bool
}
#[derive(PartialEq)]
#[derive(Debug)]
enum Department{
    Engineering,
    Sales,
    HR,
    Finance
}
struct PayrollSystem{
    employees : HashMap<String,Employee>,
}
impl PayrollSystem {
    fn new ()-> PayrollSystem{
        PayrollSystem{employees : HashMap::new()}
    }
    fn add_employee(& mut self, employee:Employee )-> Result<(), String>{
        if  self.employees.contains_key(&employee.name){
            Err(String::from("The Employee already exists"))
        }else{
            self.employees.insert(employee.name.clone(),employee);
            Ok(())
        }
    }  
    fn get_employee(&self, employeename:&str )-> Option<&Employee>{
        self.employees.get(employeename)
    }
    fn deactivate(&mut self, name:&str)->Result<(), String>{
       let condition = self.employees.get_mut(name);
       match condition {
        Some(employee)=>{
            employee.is_active = false;
             Ok(())
        }
        None => Err(String::from("Employee not found")),
       }
    }
    fn total_payroll(&self)->f64{
        let total = self.employees
            .iter()
            .filter(|(_, employee)|employee.is_active )
            .map(|(_, employee)|employee.salary)
            .sum();
        total
    }
    fn department_employees(&self , department:Department) -> Vec<&Employee>{
        let d = self.employees
            .iter()
            .filter(|(_, employee)|employee.department == department)
            .map(|(_, employee)|employee)
            .collect();
        d
    }
    fn highest_earner(&self ) -> Option<&Employee>{
        self.employees
            .iter()
            .filter(|(_, employee)|employee.is_active == true)
            .map(|(_, employee)| employee)
            .max_by(|a,b|a.salary.partial_cmp(&b.salary).unwrap())
            
    }
}   

fn main() {
   let mut payroll = PayrollSystem::new();

    match payroll.add_employee(Employee {
        name: String::from("Alice"),
        department: Department::Engineering,
        salary: 120000.0,
        is_active: true,
    }) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }

    match payroll.add_employee(Employee {
        name: String::from("Bob"),
        department: Department::Sales,
        salary: 85000.0,
        is_active: true,
    }) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }

    match payroll.add_employee(Employee {
        name: String::from("Carol"),
        department: Department::HR,
        salary: 70000.0,
        is_active: true,
    }) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }

    match payroll.add_employee(Employee {
        name: String::from("David"),
        department: Department::Finance,
        salary: 95000.0,
        is_active: true,
    }) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }

    match payroll.add_employee(Employee {
        name: String::from("Eve"),
        department: Department::Engineering,
        salary: 135000.0,
        is_active: true,
    }) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }

    match payroll.deactivate("Bob") {
        Ok(_) => println!("Bob has been deactivated."),
        Err(err) => println!("{}", err),
    }

    println!("\nTotal Active Payroll: ${:.2}", payroll.total_payroll());

    println!("\nEngineering Employees:");
    let engineering = payroll.department_employees(Department::Engineering);

    for employee in engineering {
        println!(
            "{} - Salary: ${:.2} - Active: {}",
            employee.name,
            employee.salary,
            employee.is_active
        );
    }

    match payroll.highest_earner() {
        Some(employee) => {
            println!(
                "\nHighest Earner: {} (${:.2})",
                employee.name,
                employee.salary
            );
        }
        None => println!("No active employees found."),
    }

    match payroll.get_employee("Alice") {
        Some(employee) => println!(
            "\nFound employee: {} ({:?})",
            employee.name,
            employee.department
        ),
        None => println!("Employee not found."),
    }
}



