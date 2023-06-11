use std::env;
use lib1;
pub mod modtest; 

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("Arguments are {:?}", &args);


    // cargo run TestNumber 1234
    if &args[1] == "TestNumber" {
        println!("The number value is {}", &args[2])
    }

    // cargo run modtest
    if &args[1] == "modtest" {
        modtest::run_module_code(&args);
    }
    
}










/*

    Modules are Rust’s namespaces. They’re containers for the functions, types, constants,
    and so on that make up your Rust program or library. Whereas crates are about code
    sharing between projects, modules are about code organization within a project. They
    look like this:

    Here lib1 and lib2 are the crates.   here they are in the same directory but in practice
    creates are usually on shared place like git.  create are codes that can be shared between 
    programs. create has to be imported in the program in cargo.toml 

    Module are to organize local codebase.    they can be in a separate directory like in this case
    modtest  in this cae there must be a file     mod.rs         modules can also be in the same
    directory like in modtest there is minus.rs    that can be used within mod.rs 

    main.rs 
        modtest
        mod.rs
        minus.rs 


    The :: operator is used to access features of a module. Code anywhere in your project
    can refer to any standard library feature by writing out its absolute path:
    if s1 > s2 {
        ::std::mem::swap(&mut s1, &mut s2);
    }

    This function name, ::std::mem::swap , is an absolute path, because it starts with a
    double colon. The path ::std refers to the top-level module of the standard
    library. ::std::mem is a submodule within the standard library,
    and ::std::mem::swap is a public function in that module.    

    The alternative is to import features into the modules where they’re used:
    use std::mem;
    if s1 > s2 {
        mem::swap(&mut s1, &mut s2);
    }    

    Several names can be imported at once:
    use std::collections::{HashMap, HashSet};           // import both
    use std::io::prelude::*;            // import everything

*/




/*
    macros has ot be in a separate program 
*/




/*
Implemeitng trails 
impl Sheep  -  this extend the Sheep object and you can add more interfaces in it
impl Animal for Sheep   -     this will implement Animal interface for the Sheep 
*/