use std::collections::HashMap;
//use std::io;

//use crate::Department::Sales;
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
    fn department_employees(&self , department:&Department) -> Vec<&Employee>{
        let d = self.employees
            .iter()
            .filter(|(_, employee)|employee.department == *department)
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
    fn least_earner(&self)->Option<&Employee>{
        self.employees.iter()
        .filter(|(_,employee)|employee.is_active==true)
        .map(|(_, employee)|employee)
        .min_by(|a,b|a.salary.partial_cmp(&b.salary).unwrap())
    }
    
}
fn read_input(prompt: &str)-> String{
        println!("{}", prompt);
        let mut input =String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }   

fn main() {
   /*let mut payroll = PayrollSystem::new();

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
    }*/

  let mut payroll = PayrollSystem::new();   

  loop {
    println!("\n=Payroll Manager=");
    println!("1. Add new employee");
    println!("2. Deactivate employee");
    println!("3. View employee's status");
    println!("4. View Total payroll budget");
    println!("5. Veiw department employees");
    println!("6. View highest earner and least earner");
    println!("Enter q to quit");

    let choice = read_input("Enter the number that corresponds to your prefered option");
    match choice.as_ref() {
        "1"=> {
            let name = read_input("Enter name:");
            
            let department = match read_input("Enter department:").as_ref(){
                    "Engineering"=>Department::Engineering,
                    "HR"=>Department::HR,
                    "Sales"=>Department::Sales,
                    "Finance"=> Department::Finance,
                    _=>{
                        println!("Invalid department");
                        continue;
                    }
            };
            let salary= read_input("Enter salary:").parse().unwrap_or(0.0);
            let employee = Employee { name, department, salary, is_active: true };
            match payroll.add_employee(employee){
                Ok(_)=>println!("Employee added sucessfully"),
                Err(e)=>println!("{}",e)
            }
        }
        "2"=> {
            let name = read_input("Enter the name of the employee to be deactivated:");
            match payroll.deactivate(&name) {
                Ok(_)=>println!("{} has been deactivated", name),
                Err(e)=> println!("Error: {}",e),
            }            
        }
        "3"=> {
            let name =read_input("Enter employee's name to view status");
            match payroll.get_employee(&name){
                Some(employee)=> println!("Found !
                                    {}- Department: {:?}- Salary: {:.2} - Active:{}", employee.name, employee.department, employee.salary,employee.is_active),         
                 None=>println!("Employee not found ")
            }
        } 
        "4"=>println!("\nTotal budget is {}",payroll.total_payroll()),
        "5"=>{
            let department_name =match read_input("Enter department  to view its employees").as_ref() {
                "Engineering"=>Department::Engineering,
                "Sales"=>Department::Sales,
                "HR"=>Department::HR,
                "Finance"=>Department::Finance,
                _=>{
                    println!("Enter a valid department");
                    continue;
                }
            };
            payroll.department_employees(&department_name);
            for employee in payroll.department_employees(&department_name){
                println!("{} - Salary: {:.2} - Active :{} ", employee.name, employee.salary,employee.is_active);
            }
        }
        "6"=>{
            match payroll.highest_earner(){
                Some(employee)=> println!("The highest earner is {} and earns {}",employee.name,employee.salary), 
                None => println!("No active record is found")
            }
            match payroll.least_earner(){
                Some(employee)=>println!("The least earner is {} and earns {}",employee.name,employee.salary),
                None=> println!("No active record is found")
            }
        }
        "q"=>break,
        _ => println!("Invalid choice, try again"),
    }
  }
}
    





