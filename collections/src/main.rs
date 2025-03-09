fn main() {
    // Vector 
    let vect: Vec<i32> = Vec::new();

    // Vector with implied type
    let v = vec![1, 2, 3];

    // Pushing to the vector.
    v.push(5);
    v.push(6);
    v.push(7);

    // Reading the vector using a reference
    let third: &i32 = &v[2];
    println!("The third elemnt is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third elemente is {third}"),
        None => println!("There is no third elemement."),
    }
    
    // The code below will not work. Reason: When there is a push to a vector, it will move inside the memory
    // Since `first` holds the memory address of the first element, it considered immutable
    // v.push in other hand is multable.
    let v2 = vec![100,200,300];

    let first = &v[0];

    v.push(300);

    println!("This was the first value: {first}");

    // Iterating through a vector
    let v3 = vec![1,2,3,4,5];
    for i in &v3 {
        println!("Value is: {i}");
    }

    // Multating the values of the iteration
    let mut v4 = vec![10,20,30,40,50];
    for i in &mut v4 {
        //deference
        *i += 50;
    }

    // enum vector to hold different types
    enum SpreasheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec!{
        SpreasheetCell::Int(3),
        SpreasheetCell::Text(String::from("blue")),
        SpreasheetCell::Float(10,12),
    }
}
