
mod awesome {
    pub trait Trait {
        fn do_something_awesome(&self) -> String {
            String::from("that might be aswesome")
        }
    }

    pub struct Struct<T> {
        pub awesome_field: T
    }
    
    impl<T: std::fmt::Display> Trait for Struct<T> {
        fn do_something_awesome(&self) -> String {
            format!("{} was awesome!", self.awesome_field)
        }
    }
}

mod cool {
    pub trait Trait {
        fn do_something_cool(&self) -> ();
    }

    pub struct Struct<T> {
        pub cool_field: T
    }

    impl<T> Trait for Struct<T> {
        fn do_something_cool(&self) {
            println!("{}",String::from("that was cool"))
        }
    }
    
}

mod awesome_and_cool {
    pub trait Trait {
        fn do_something_awesome_and_cool(&self) -> () {
            println!("it was awesome and cool")
        }
    }

    pub struct Struct<T> {
        pub awesome_and_cool_field: T
    }

    use crate::cool::Trait as CoolTrait;
    use crate::awesome::Trait as AwesomeTrait;

    impl<T: Trait> AwesomeTrait for T {
    } 
    
    impl<T: Trait> CoolTrait for T {
        fn do_something_cool(&self) -> () {
            println!("{} (and also it was cool)", self.do_something_awesome())
        }  
    } 

    impl<T> Trait for Struct<T> {
    }
    
    
}

use awesome::Trait as AwesomeTrait;
use cool::Trait as CoolTrait;
use awesome_and_cool::Trait as AwesomeAndCoolTrait;
use awesome::Struct as AwesomeStruct;
use cool::Struct as CoolStruct;
use awesome_and_cool::Struct as AwesomeAndCoolStruct;

fn main() {
    let awesome_struct_example = AwesomeStruct {
        awesome_field: String::from("awesome_value")
    };
    println!("{}", awesome_struct_example.do_something_awesome());
    
    let cool_struct_example = CoolStruct {
        cool_field: false
    };
    cool_struct_example.do_something_cool();

    let cool_and_awesome_struct_example = AwesomeAndCoolStruct {
        awesome_and_cool_field: false
    };
    println!("{}", cool_and_awesome_struct_example.do_something_awesome());
    cool_and_awesome_struct_example.do_something_cool();
    cool_and_awesome_struct_example.do_something_awesome_and_cool();
}