fn main() {
    println!("Hello, world!");
    let penguin_data = "\
    field_name,22,field_name,field_name CRLF
    aaa,33,bbb,ccc CRLF
    zzz,3.14,yyy,xxx CRLF
    ";

    println!("{:?}", penguin_data);

    let records = penguin_data.lines();

    // 跳过表头行和只含有空白符的行
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // Vec动态数组，_下划线表示rust推断出此动态数组的元素类型
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            // 标注输出错误
            eprintln!("debug {:?} ->{:?}", record, fields);
        }

        let name = fields[0];
        // 试图把该字段解析为一个浮点数，解析成功赋值给length变量
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{:?} {:?}", name, length);
        }
    }
}
