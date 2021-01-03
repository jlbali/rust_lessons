use std::collections::HashMap;

fn main() {
    
    // Vectors

    let mut v: Vec<i32> = Vec::new();
    let w = vec![1,2,3,4,5];

    v.push(4);
    v.push(5);

    let third = &w[2];
    println!("The third element is {}", third);

    match w.get(2) {
        Some(elem) => println!("We obtain {}", elem),
        None => println!("There is no such an element."),
    }

    let v = vec![100,32,57];
    for e in &v {
        println!("{}",e);
    }

    let mut v = vec![100,32,57];
    for e in &mut v {
        *e += 50;
    } // Adding 50 to each element of v.
    println!("v equals {:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row equals {:?}", row);

    // Strings

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 fue movido y no se puede volver a usar.
    // s2 se puede usar, pues fue por referencia que es lo que espera el +.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1,s2,s3);
    println!("{}", s);

    println!("As bytes...");
    for c in "नमस्त".chars() {
        println!("{}", c);
    }
    println!("As bytes...");
    for c in "नमस्त".bytes() {
        println!("{}", c);
    }

    // Hash Map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("Scores: {:?}", scores);

    // Por algún motivo esto no anda.
    //let scores_vec = vec![(String::from("Blue"), 10), (String::from("Yellow"), 50)];
    //let scores: HashMap<String,i32> = scores_vec.iter().collect();

    // Tiene un comportamiento raro esto...
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];
    let scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let score = scores.get(&String::from("Blue"));
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // Updating a value.
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Conditional insertion (if it does not exists)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Update based on an old vlaue.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // Exercises.
    println!("Media: {}", mean(&vec![1.0,5.0,12.0]));
    println!("Median: {}", median(&vec![5.0,1.0,12.0]));
    println!("Mode: {}", mode(&vec![5,1,12, 5]));
}

#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn mean(v: &Vec<f32>) -> f32 {
    let mut accum = 0.0;
    for e in v {
        accum += e;
    }
    return accum / v.len() as f32;
}

fn median(v: &Vec<f32>) -> f32 {
    //v.sort();
    let mut v = v.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 0 {
        return (v[n/2] + v[n/2 + 1])/2 as f32;
    } else {
        return v[n/2];
    }
}

// Float does not have an Eq or Cmp trait.

fn mode(v: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for e in v{
        let res = map.get(e);
        match res {
            Some(num) => {
                map.insert(*e, num + 1);
            }
            None => {
                map.insert(*e, 0);
            }
        }
    }
    let mut max_count: usize = 0;
    let mut argmax = 0;
    for (key, value) in &map {
        if *value > max_count {
            argmax = *key;
            max_count = *value;
        }
    }
    return argmax;

}
