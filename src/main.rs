
#[allow(unused_imports)]
extern crate mymacro;
use mymacro::{Falidate};


#[derive(Falidate)]
struct User {
    #[required]
    name: String, 
    
    #[in_array("1,2,3")]
    user_type: i8
}


fn main() {
    let _user = User {
        name: "Farhad".to_string(),
        user_type: 1
    };


    println!("{} ----- ", format!("sds {}, {}", _user.name.to_string(), _user.user_type.to_string()));
    falidate();

    assert_eq!(2,2);
}