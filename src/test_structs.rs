use std::ops::Drop;

pub struct TestStuct1;

impl Drop for TestStuct1 {

    fn drop(&mut self) {

        println!("Dropping TestStuct1!");

    }
    
}

pub struct TestStuct2;

impl Drop for TestStuct2 {

    fn drop(&mut self) {

        println!("Dropping TestStuct2!");

    }

}

pub enum DropEnum
{

    Test1(TestStuct1),
    Test2(TestStuct2)

}

