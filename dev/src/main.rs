#[derive(Debug)]
enum Event {
    // 创建了包含三个事件的枚举体，其中最后一个值为无法识别的事件
    Update,
    Delete,
    Unknown,
}

type Message = String;   // 定义新类型（起别名），给String一个更合适的名字，可以在这个create的上下文使用

fn parse_log(line: &str) -> (Event, Message) {   // <4>
    let parts: Vec<_> = line                       // <5>
        .splitn(2, ' ')
        .collect();                // <6>
    if parts.len() == 1 {                          // <7>
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];                // <8>
    let rest = String::from(parts[1]);   // <8>

    match event { // 当我们匹配到一个数据时，就返回结构化的数据
        "UPDATE" | "update" => (Event::Update, rest),  // <9>
        "DELETE" | "delete" => (Event::Delete, rest),  // <9>
        _ => (Event::Unknown, String::from(line)),    // 不能识别这个事件类型的时候，就把整行数据返回
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}