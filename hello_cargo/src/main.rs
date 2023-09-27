#[derive(Debug)]
/* struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self{
        Self{
            width,
            height,
        }
    }
}

struct Circle{
    radius: f64,
}

const PI_VALUE: f64 = 3.142;

impl Circle{
    fn new(radius: f64) -> Self{
        Self{radius}
    }

    fn area(self){
        PI_VALUE * self.radius * self.radius;
    }

    fn can_hold(&self, other: &Circle) -> bool{
        self.radius > other.radius
    }
}

struct Date{
    day: u64,
    month: u64,
    year: u64,
}

impl Date{
    fn new(day: u64, month: u64, year: u64) -> Self {
        Self{
            day,
            month,
            year
        }
    }

    // creating getters
    fn day(&day)->u64{
        self.day
    }

    fn month(&month)->u64{
        self.month
    }

    // for getters field name should be same of the constructor
    fn year(&year)->u64{
        self.year
    }

    fn isLeap(&self)-> String{
        if self.year % 4 == 0 && self.year % 100 == 4 || self.year % 400 == 0 {
            println!("This is leap year.");
        }
        else {
            println!("This is not leap year.");
        }
    }

    /* fn addDay(&self) -> u64{
        
    } */
} */


fn main() {
//     1. Cargo is a package manager and it works like it and control dependency management etc. 

// 2. Rust use snake notation which use underscore and there should be useful variable names.

// 3. cargo new hello_cargo (it is for creating cargo)

// 4. tom's obvious minimal language 

// 5. rust extension and codeLLDB then connect with cloudspace.

// * Keywords in rust

// 6. In rust, variables are immutable by default. to change a variable we use 'mut' so they can change 
// otherwise they are unchanged by default which support data concurrency and memory safety.
    // println!("Name:Daniyal");
    // println!("Roll Number: CS211167");

    // mutable and immutable
    // let x = 15;


    // let mut y = 10;
    // y = 14;

    // println!("The value of x is: {x}");

    // type inference keeps the code concise and it is smart it is recognized byself the data type we dont have to mention the data types
    // follow type 
    // let x: i32 = 14;
    // let mut y: i32 = 2;
    // y = 3;
 

    // constants must have a specified type and they must be uppercase and use underscore
    // we cant use mut in constants
    // const NEW_VALUE: u32 = 60+40;
    // println!("The value of x is: {NEW_VALUE}");

    // shadowing: 
 /*    let y = 5;
    let y = y + 1; // old y will be discarded as C++ doesnt support we will not get error on compile time as we are reassigning but once we use let then it is ok and it now immutable and we can reuse the name as we wrote before but type can be changed by using let

    // let spaces = " ";
    // let spaces = spaces.len(); keep in mind we create new var when we are using let
    
    { //creationg a new scope
        let y = y * 2;
        println!("The value of y in inner scope: {y}");
    }

    println!("The value of y in outer scope: {y}"); */
    //  shadowing

    // DATA types: scaler (int, floating, char, bool) and compound (arrays, tuples)
    // f32 or f64 which is by default and all floating points number are signed which is + and -ve
    // bits spaces depends on architecture
    // char type let x: char = "Daniyal";

    // tuple : varity of types into one single compound element and it comma separated and type and size will be fixed and tuple is a single unit
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (a, b, c) = tup; // x y z are separate variables and this is destructuring and pattern matching

    // if tuple is empty that is unit if it doesn't have value then it will return unit
    // println!("The value of b: {b}");

    // we can access tuples by using dot operator which is period 
    // let tup1: () = ();

    // ARRAY is of fixed length we use with with stack because its size is known
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let b: [i32; 5] = [3; 5];
    // let mut count = 0;
    // loop{
    //     println!("{:?}", b[count]);
    //     count += 1;

    //     if count == 5{
    //         break;
    //     }
    // };

    // placeholder of value is {} and & is dereferencing

    // to insert in arrays
    // let a = [3; 5];
    // println!("Numbers:");
    

    // task 2

    // let x: i32 = 14;
    // let mut y: i32 = 2;
    // y = 3;

    // println!("{x} , {y}");
    // task 2 ended

    // task 3
    // const PI_VALUE: f64 = 3.14159;
    // let Radius_value = 5;
    // let area = PI_VALUE*Radius_value*Radius_value;    

    // println!("The area of a circle is given as {area}");

    // function
    // println!("{}", sum(5, 4));

    // expressions return a value either value or bool 
    // statement is variable declaration
    // x + 1 is expression it is not modified

    // println!("{}", sum1(5, 4));
    

    // let condition = true;
    // let check = if condition {5} else {6};
    
    // println!("The value of number is: {check}");

    // loops
    // iterable var should be mutable and loop runs infinity and we use break to stop

    // let mut counter = 0;

    // let result = loop{
    //     counter += 1;

    //     if counter == 10{
    //         break counter * 2;
    //     }
    // };
    // println!("{}", {result});

    // tuple creation: let (divisor, number)
    // Function with conditional return

    // assesment
    /* let mut counter = 1;

    while counter <= 100 {
        if counter % 3 == 0 && counter % 5 == 0 {
            println!("FizzBuzz");
        }
        else if counter % 5 == 0{
            println!("Buzz");
        }
        else if counter % 3 == 0 {
            println!("Fizz");
        }
        else {
            println!("{counter}");
        }
        counter += 1;
    }; */

    /* let y = {
        let x = 3;
        x + 1
        };
        println!("The value of y is: {y}"); */

    
        // println!("{}", fact(10));

        // Garbage collector scans memory heap all time and clear unreference memory and memory is going to free. It keep track of and cleans up memory that is not being used anymore and we dont think about it
        // Ownership decide how memory will work when to allocate and deallocate
        // pushing in stack is faster and in heap we have to search and either we get the memory or not. access in heap it is also slow

        // Ownership
        // each value in rust has an owner and once get out of scope its value is dropped. There can be only one owner at a time.
        // Known size can be push and pop into stack
        // string literal is not convinient for every situation for eg input and we can't change
        // for this we have STring and it is store in heap because when size is not known we have heap 
        // String literals are immutable
        // In the case of a string literal, we know the contents at compile time, 
        // so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.
        // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope
/*         There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope.
         When a variable goes out of scope, Rust calls a special function for us.
          This function is called drop, and it’s where the author of String can put the code to return the memory. 
          Rust calls drop automatically at the closing curly bracket. */
         /*  let s = String::from("hello");
          println!("{}", s);
          size of text is known = use string literal 
          size of text is unknown = use String (stores in heap) */

        //   pointer point out data on heap
        // s is actually a stack len is bascially memory use and capacity is memory available

//         shallow copy = reference to data is copied
//         deep copy = whole data is copied
/* 

let  s1 = String::from("Hello"); //string 
let s2 = s1;
// strings are mutated and string literals are immutable

- this will only give the ptr of s1 to s2
1- this way there are now two owners of the same memory location
2- this would lead to freeing the same memory twice, this causes security vulnerabilities. It is called a double free error

is this correct Maam? 

Questions:
- which security vulnerability will there be?
- so it isnt allowed to write let s2 = s1 here? ya we can write but it overtakes ownership? */

// when variable holding stack data is assigned to another variable = copy of it is assigned

// when variable holding address heap data is assigned to another variable = ownership change

    // **************************** 15-09-2023 **********************************
    // freeing memory twice can lead to memory corruption
    // when we return a value to a function now the function is the owner of that 
    // reference:  is like a pointer in that its an address we can folow to access the data
    // move is ownership transferring
    // in rust, there is no concept of deep copy as it never allow to create automatically deep copies of your data
    // if you do want to copy the heap data of the string we can use a method clone so s1 and s2 both accessible
    // ************* OWNERSHIP AND BORROWING *******************
/*     let s = String::from("Daniyal");
    let (s1, length) = calculate_str(s);

    println!("the size of the string {s1} is {length}."); */
    // references don't take ownership they just points the respective thing in the heap 
    // passing in references as function parameters called borrowing because we borrow the value not taking ownership
    /* Again doing by references the above task */
    // let s1 = String::from("HELLO");
    // let len = calculate_Str(&s1);
    // println!("the size of the string {s1} is {len}.");

    // references are immutable by default
    // let mut s = String::from("HELLO");
    // change(&mut s); //&mut is mutable parameter reference
    // data race: two pointers can't access the same data at a time, at least one of the pointers is being used to write the data, there is no synchronization
    // rust prevents data races at the compile time
    // it is okay to have multiple immutable references as there is no changing 

    // let r = String::from("HELLO");
    // let r1 = &s; //immutable
    // let r2 = &s; //here we are reading only so they can't afford to change by later 
    // println!("{r1}, {r2}");

    // let r3 = &mut s;
    // we cannot have a mutable reference while we have an immutable one to the same value 
    // println!("{r3}, {r2}");
    // remember reference scope 
    
    // ************** DANGLING POINTER
    // dangling references is that what if our references points to invalid data
    // &s: reference to a string s is out of scope and then refernces must be valid.
    // In other words,if you have a reference to some data,
    // the compiler will ensure that the data will not go out of scope before the reference to the data goes out of scope.
    // let reference_to_nothing = create_dangling_reference();

    // *********************** 23-09-2023 ****************************
    // in struct only string var ownership has gone when we pass into function
    // struct instance is mutable 
        // &str is string slice
        /* let user1 = User{
            active: true,
            username: String::from("daniyal6429"),
            email: String::from("daniyal@gmail.com"),
            sign_in_count: 1,
        };

        let mut user2 = User{
            email: String::from("hammad@gmail.com"),
            ..user1
        };

        // user1.username cant print here as ownership has transfered but you can modify them as you create user1 a mutable reference
        // structs are immutable by default
        // now debugging: dbg! take ownership for some time to print 
        println!("{:#?}", user1); */

        // ************ SELF ****************
        // self: &Self 
        // &self is immutable reference = this, it is always 1st parameter which is passing an object
        // rect1.area() between paranthesis rect1 is gone as reference
        // it is used within the methods to refer to the current instance of a struct, enum or trait
        // we have two selfs 1. self for referenec and 2. Self as keyword used to represent current type it is actually you implement your type and useful when you don't know your type 
        // impl methods are associated functions
        /* let rect1 = Rectangle::new(5, 4);
        println!("{} {}.", rect1.width, rect1.height); */

        /* let circle1 = Circle::new(5.00);
        let circle2 = Circle::new(10.00);

        println!("{}", Circle::can_hold(&circle1, &circle2)); */

        /* let check = Date::new(09, 09, 2001);
        println!("{}", Date::day(&check)); */

        // **************** ENUM ************************
        

}





// fn sum(num1: i64, num2: i64) -> i64
// {
//     let adds = num1 + num2;
//     return adds;
// }

/* fn create_dangling_reference() -> &String {
    let greeting_msg = String::from("Greetings from the future");

    &greeting_msg
} */

// fn sum1(num1: i64, num2: i64) -> i64
// {
//     num1 + num2
//     // no need of return as last line is expression if there will be semi colon then it will give error
// }



/* fn fact(num: i64) -> i64 {
    if num == 0 {
        1
    }else{
        num * fact(num - 1)
    }
} */

/* fn calculate_str(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

fn calculate_Str(s: &String) -> usize{
    let length = s.len();
    length
}

fn change(some: &mut String){
    some.push_str(", World");
    println!("{some}");
} */