#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {
        //TODO! (implement printing info about Student
        // use match statement)
        println!("My name is {}", self.name);
        match self.grade{
            GradeLevel::Bachelor => println!("and I am currently pursuing a Bachelors "),
            GradeLevel::Master => println!("and I am currently pursuing a Masters "),
            GradeLevel::PhD => println!("and I am currently pursuing a Doctorates "),
        }
        match self.major{
            Major::ComputerScience => println!("in Computer Science"),
            Major::ElectricalEngineering => println!("in Electrical Engineering"),
        }
       
    }
}

fn main() {
    let s1 = Student::new("John".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
s1.introduce_yourself();
}
//Code should compile. Run and submit a screenshot