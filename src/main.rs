mod Persona {
    pub struct User {
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub email: String,
        pub age: u8,
    }
    impl User {
        pub fn say_hello(&self, name: &str) {
            println!("Hello {}, my name is {}", name, self.first_name);
        }
    }
}

#[test]
fn test_persone() {
    let user: Persona::User = Persona::User {
        first_name: String::from("Jack"),
        last_name: String::from("Ma"),
        username: String::from("Mama"),
        email: String::from("example.com"),
        age: 20,
    };

    user.say_hello("Mama");
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, b, c) = data;

    println!("{}, {}, {}", a, b, c);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit() {
    println!("Hello, world!");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);
}

#[test]
fn array_mut() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let lenght = array.len();
    println!("lenght {}", lenght);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("chandra");
    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("hp");
    println!("{} {}", a, b);
}

#[test]
fn ownership() {
    //copy data. keyword "copy"
    let a = 10;
    let b = a; //copy the value. b=10
    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Jack");

    //ownership from name1 moved to name2
    let name2 = name1;
    //name1 cannot be accessed here
    //println!("{}",  name1); doesn't work for value stored in heap
    println!("{}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("Jack");
    let name2 = name1.clone(); //keyword "clone". this causes heavy computation
    println!("{} {}", name1, name2);
}

#[test]
fn loop_expression() {
    let mut count = 0;
    loop {
        count += 1;
        if count > 10 {
            break;
        } else if count % 2 == 0 {
            continue;
        }

        println!("counter {}", count);
    }
}
#[test]
fn loop_with_return_val() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_with_label() {
    let mut num = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if num > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", num, i, num * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        num += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range() {
    let range = 0..5;
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=4;
    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}

fn print_number(number: i32) {
    println!("{}", number);
}

fn hi(name: String) {
    println!("{}", name);
}

#[test]
fn test_function_ownership() {
    let number = 10; //stored in stack
    print_number(number); //copy data. because copying data in stack is cheap
    println!("Number: {}", number);

    let name = String::from("Jack");
    hi(name); //transfer ownership. because copying data in heap is expensive
    //println!("Name: {}", name); //doesn't work
}

fn combine_string(first: String, second: String) -> String {
    format!("{}-{}", first, second)
}

fn combine_string_return(first: String, second: String) -> (String, String, String) {
    let full_name = format!("{}-{}", first, second);
    (first, second, full_name)
}

#[test]
fn test_return_ownership() {
    let first_name = String::from("Jack");
    let second_name = String::from("Ma");

    let name = combine_string(first_name, second_name); //this return the value ownership to this variable

    println!("{}", name);
    //println!("{}", second_name);//ownership has changed and lost
    //println!("First : {}", first_name);// ownership has changed and lost
}

#[test]
fn test_return_ownership2() {
    let first_name = String::from("Jack");
    let second_name = String::from("Ma");

    let (a, b, full_name) = combine_string_return(first_name, second_name);

    println!("{}", full_name);
    println!("{}", a);
    println!("{}", b);
}

fn combine_string_reference(first: &String, second: &mut String) -> String {
    //first.push_str(second);// this will cause an error. this is a "borrowing" problem
    second.push_str("+"); // mutable reference
    // the mutable reference only "live" within this scope
    format!("{}-{}", first, second)
}

#[test]
fn test_reference() {
    let mut first = String::from("Jack");
    let mut second = String::from("Ma");

    first.push_str("-"); // you can edit this. because it's not "borrowed" value

    let name = combine_string_reference(&first, &mut second);
    println!("{}", name);
    println!("{}", first);
    println!("{}", second);

    // at the same moment you only allowed to create 1 mutable reference

    // this is actually not breaking the rule. because of the 'scope' thing.
    let _name = combine_string_reference(&first, &mut second);
    let name = combine_string_reference(&first, &mut second);
    let name = combine_string_reference(&first, &mut second);

    // violation case
    let valueBorrow1 = &mut second;
    //let  valueBorrow2 = &mut second; //this is not allowed. because the two lives in the same scope in the same time
    let name = combine_string_reference(&first, valueBorrow1);
    //let name = combine_string_reference(&first, valueBorrow2); //this is not allowed
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn greet(&self, name: &str) {
        println!("Hello, {} i'm {}", name, self.middle_name);
    }
}

#[test]
fn test_person_method() {
    let person = Person {
        first_name: String::from("Max"),
        middle_name: String::from("Ver"),
        last_name: String::from("Stephen"),
        age: 20,
    };
    person.greet("Jack");
}

trait Helloable {
    fn hello(&self) -> String {
        String::from("Hello!")
    }
    fn introduce(&self) -> String;
    fn greet(&self, name: &str) -> String;
}

impl Helloable for Person {
    fn introduce(&self) -> String {
        format!("I am, {} {}!", self.first_name, self.middle_name)
    }
    fn greet(&self, name: &str) -> String {
        format!("I'm {}, Hello {}!", self.first_name, name)
    }
}

fn say_hello_from_trait(pers: &impl Helloable) {
    println!("{}", pers.introduce())
}

#[test]
fn test_trait_function() {
    let person = Person{
        first_name: String::from("Max"),
        middle_name: String::from("Ver"),
        last_name: String::from("Stephen"),
        age: 20,
    };

    say_hello_from_trait(&person);
    hello_and_goodbye(&person);
}


trait CanSayGoodBye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.first_name)
    }
}

fn hello_and_goodbye(value: &(impl Helloable + CanSayGoodBye)) {
    println!("{}", value.introduce());
    println!("{}", value.good_bye());
}
