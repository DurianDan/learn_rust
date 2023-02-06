// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:34:20

// pub fn show(){
//     crate::show_info::show_computer_specs::message::show_specs();
//     // "crate" represents the root path
//     // the main.rs use "crate" to access all custome crates
// }

pub mod show_computer_specs{
    // create a module
    // module can include multiple functions
    pub struct Computer{
        // to publish any objects (structs, module, functions, traits)
        // use "pub"
        pub cpu: String,
        pub ram: i32,
        pub gpu: String,
    }

    impl Computer{
        pub fn office(cpu: String) -> Computer{
            Computer{cpu: cpu , ram: 4, gpu: "Intel".to_string()}
        }
    }
    pub mod message{
        fn greeting(computer_type: String){
            println!("This is the specs of your {} computer:", computer_type)
        }
        fn show_specs(computer: super::Computer){
            println!("cpu: {}",computer.cpu);
            println!("ram: {}",computer.ram);
            println!("gpu: {}",computer.gpu);
        }
        pub fn show_office(cpu: String){
            // this function will be published/ able to be accessed,
            // but other functions (show_specs, greeting) will not 
            greeting("Office".to_string());
            let computer: super::Computer = super::Computer::office(cpu);
            show_specs(computer);
        }
    }
}

