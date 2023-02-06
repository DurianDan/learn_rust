// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:34:20 to 1:45:53

mod show_info;
use crate::show_info::show_computer_specs;

fn main(){
    show_computer_specs::message::show_office(String::from("Intel"));
}