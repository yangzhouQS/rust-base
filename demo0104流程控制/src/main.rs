fn main11() {
    let a = 23;
    if a < 1 {
        println!("You are a baby.");
    } else if a < 3 {
        println!("You are a toddler.");
    } else if a < 5 {
        println!("You are a preschooler.");
    } else if a < 18 {
        println!("You are a teenagers.");
    } else {
        println!("You are an adult");
    }
}

fn main22() {
    let mut count = 0;
    let counter = loop {
        count += 1;
        let counter = count * 2;
        println!("count = {}, counter = {}", count, counter);
        if count == 10 {
            // break;//停止循环
            break counter;
        }
    };
}

fn main33() {
    let mut count = 0;
    let mut sum = 0;
    while count <= 100 {
        sum += count;
        count += 1;
    };
    println!("sum = {}", sum);
}

fn main44() {
    let mut count = 100;
    let mut sum = 0;
    for i in 0..=count {
        sum += i;
    }
    println!("sum = {}", sum);
}

// match模式匹配
fn main() {
    let age = 200;
    match age {
        0..=5 => println!("You are a preschooler."),
        5..=18 => println!("You are a teenagers."),
        18..=100 => println!("成年人."),
        _ => println!("成神了")
    }
}