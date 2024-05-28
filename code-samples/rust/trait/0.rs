trait AwesomeTrait {
    fn do_something_awesome(&self) -> String;  
}

trait CoolTrait {
    fn do_something_cool(&self) -> ();
}

struct AwesomeStruct<T> {
    awesome_field: T
}

impl AwesomeTrait for AwesomeStruct<String> {
    fn do_something_awesome(&self) -> String {
        format!("{} was awesome!", self.awesome_field)
    }
}

impl<T> CoolTrait for AwesomeStruct<T> {
    fn do_something_cool(&self) {
        println!("{}",String::from("that was cool"))
    }
}

fn main() {
    let awesome_struct_example = AwesomeStruct {
        awesome_field: String::from("awesome_value")
    };
    println!("{}", awesome_struct_example.do_something_awesome());
    
    let awesome_struct_example: AwesomeStruct<bool> = AwesomeStruct {
        awesome_field: false
    };
    awesome_struct_example.do_something_cool();
}