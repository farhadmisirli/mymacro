
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
    User {
        name: "Farhad".to_string(),
        user_type: 1
    };

    falidate();

    assert_eq!(2,2);
}