use derive_macro::Describe;
extern crate lib1;
extern crate lib2;
extern crate log;
use std::str::FromStr;


pub fn minus123(left: usize, right: usize) -> usize {
    left - right
}



pub fn iterator_codes() {
    /*
        Iterator   the following are the same  
        The for loop uses IntoIterator::into_iter to convert its operand &v into an itera‐
        tor, and then calls Iterator::next repeatedly. Each time that returns
        Some(element) , the for loop executes its body; and if it returns None , the loop fin‐
        ishes.
    */
    //-----------------------------------------------
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    for element in &v {
        println!("{}", element);
    }

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }    
    //-----------------------------------------------    


    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    let numtemp = text.split_whitespace().filter_map(|w| { 
        f64::from_str(w).ok()
    });
    for number in numtemp {
        println!("{:4.4}", number);
    }


    use std::collections::HashMap;
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }    



    let iter = (0..10).scan(0, |sum, item| {
        println!("{} - {}", sum, item );
        *sum += item;
        if *sum > 10 {
            None
        } else {
            Some(item * item)
        }
    });

    println!("{:?}", iter.collect::<Vec<i32>>());


    let array = [1u8, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }


    let mut sequence = 0..3;
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }


    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }    


}



pub fn soms_sample_codes_internal() {

    let sum: usize = lib1::add(100, 200);
    let sum2: usize = lib2::divide(4, 2);
    println!("Hello, world! {} - {}", sum, sum2);    


    struct FRange {
        val: f64,
        end: f64,
        incr: f64
    }
    
    fn range(x1: f64, x2: f64, skip: f64) -> FRange {
        FRange {val: x1, end: x2, incr: skip}
    }
    
    impl Iterator for FRange {
        type Item = f64;
    
        fn next(&mut self) -> Option<Self::Item> {
            let res = self.val;
            if res >= self.end {
                None
            } else {
                self.val += self.incr;
                Some(res)
            }
        }
    }
    
    for x in range(0.0, 1.0, 0.1) {
        println!("{:.1} ", x);
    }

    let v: Vec<f64> = range(0.0, 1.0, 0.2).map(|x| x.sin()).collect();

    println!("{:?}", v);


    #[derive(Describe)]
    struct MyStruct {
        my_string: String,
        my_enum: MyEnum,
        my_number: f64,
    }
    let ss = MyStruct {
        my_string: "ABC".to_string(),
        my_enum: MyEnum::VariantA,
        my_number: 10.0
    };
    println!("{} {:?} {}", ss.my_string, ss.my_enum, ss.my_number);


    #[derive(Describe)]
    struct MyTupleStruct(u32, String, i8);
        

    #[derive(Debug)]
    #[derive(Describe)]
    enum MyEnum {
        VariantA,
        VariantB,
    }
    println!("{:?} - {:?}", MyEnum::VariantA, MyEnum::VariantB);


    // A union declaration uses the same syntax as a struct declaration, except with union in place of struct.
    #[derive(Describe)]
    union MyUnion {
        unsigned: u32,
        signed: i32,
    }
    // The key property of unions is that all fields of a union share common storage. As a result, 
    // writes to one field of a union can overwrite its other fields, and size of a union is 
    // determined by the size of its largest field.
    let u = MyUnion{unsigned: 10};
    println!("{:?} - {:?}", unsafe{u.signed}, unsafe{u.unsigned});    


    MyStruct::describe();
    MyTupleStruct::describe();
    MyEnum::describe();
    MyUnion::describe();    


    //Using closure as a variable and passing into iterator
    let closear_var = |sum, item|{ sum + item };
    let sum_of_closuer: i32 = (1..100).fold(0, closear_var);
    println!("Closure Sum is {}", sum_of_closuer);


}













//--------------  Fibonacci Test --------------
#[derive(Debug)]
struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        println!("{:?}", self);

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator` 
        // will never return `None`, and `Some` is always returned.
        Some(current)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}
//--------------  Fibonacci Test --------------