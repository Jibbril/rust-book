mod module_one;
// mod module_one {
//     pub mod file_1 {
//         pub fn print() {}
//     }

//     pub mod file_2 {
//         pub fn print() {}
//     }
// }

use module_one::{
    file_1::print as print1,
    file_2::print as print2
};

fn main() {
    module_one::file_1::print();
    module_one::file_2::print();

    print1();
    print2();
}
