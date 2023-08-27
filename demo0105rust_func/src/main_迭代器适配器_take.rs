fn main() {
    let v = [1, 2, 3, 4, 5, 6];
    let result = v.iter().take(2);
    println!("result: {:?}", result);
    for value in result {
        println!("value: {}", value);
    }
}
// result: Take { iter: Iter([1, 2, 3, 4, 5, 6]), n: 2 }
// value: 1
// value: 2