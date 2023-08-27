fn main() {
    let v = [1, 2, 3, 4, 5, 6];
    // map迭代器适配器map,对元素的每一项进行操作,返回新的容器
    let new_v: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("new_v: {:?}", new_v);
}
// new_v: [2, 4, 6, 8, 10, 12]