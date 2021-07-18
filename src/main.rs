mod test_structs;

use test_structs::*;

fn main() {

    let _ets1 = DropEnum::Test1(TestStuct1);

    let _ets2 = DropEnum::Test2(TestStuct2);

    println!("initialised!");

}
