 #[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let mut v = Vec::new();//creamos un vector

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v2 = vec![1, 2, 3, 4];//rust infiere el tipo si creamos el vector de esta manera
    let third: &i32 = &v[2];
    let x: i32 = v[3];
    let third2: Option<&i32> = v2.get(2);
    println!("tercer elemento en vector v {}", third);
    println!("cuarto elemento en vector x {}", x);
    println!("tercer elemento vector v2 {:?}", third2);
    //let does_not_exist = &v[100]; //generara un panic
    let does_not_exist = v.get(100);// retorna un None
    println!("tercer elemento vector v2 {:?}", does_not_exist);

    let mut v3 = vec![1, 2, 3, 4, 5];
    {
        let first = &v3[0];
    }
    v3.push(6);
    for i in &v3 {
        println!("element: {}", i);
    }

    for i in &mut v3 {
        *i += 50;
        println!("element: {}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:?}", i);
    }
}
