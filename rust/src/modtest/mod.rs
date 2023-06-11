pub mod internal_module;

pub fn run_module_code(args: &Vec<String>) {
    if &args[2] == "testsamples" {
        //cargo run modtest testsamples
        internal_module::soms_sample_codes_internal();
    } else if &args[2] == "testIterators" {
        //cargo run modtest testIterators
        internal_module::iterator_codes();
    } else {
        println!("No Function found in test internal module")
    }
}

pub fn add_in_module(left: usize, right: usize) -> usize {
    left + right
}

pub fn minus_from_local_module(left: usize, right: usize) -> usize {
    return internal_module::minus123(left, right);
}

