/* enum IpAddrKind{
    v4,
    v6,
    // v8(String) another method when you don't want to create struct
} */
// you can create struct then pass them to enum 
// we can attach user-defined data types in enum very helpful in functions while using scope
// difference between struct and enum

/* struct IpAddress{
    kind: IpAddrKind
}
 */

/* enum T{
    None,
    Some(T), //T is type i32 is default type
    enum comes with name spaces ::
} */

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Values{
    Penny, 
    Nickel, 
    Dime, 
    Quarter(UsState),
}

fn value_in_cents(values: Values) -> u8 {
    match values{
        Values::Penny => 1,
        Values::Nickel => 5,
        Values::Dime => 15,
        Values::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn main() {
    // let four = IpAddrKind::v4; //instance of enum
    // let six = IpAddrKind::v6;

    // calling methods
    // route(six);
    // route(IpAddrKind::v4);

    // option
    // we handle null by using enums
    // option t is by default
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // unwrap wraps the desired type and then add
    // let sum = x + y.unwrap();

    // println!("{sum}");

    // ********** MATCH ************* like switch case
    /* match sum {
        1 => println!("ONE"),
        2 => println!("TWO"),
        10 => println!("TEN"),
        _ => println!("OTHER!"), //_ is a wild card
    } */

    value_in_cents(Values::Quarter(UsState::Alabama));
}

// fn route(ip_kind: IpAddrKind){}