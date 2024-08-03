mod json;
mod expr;


fn main() {
    let json_str = r#"{
    "msg": "",
    "data": {
        "count": 1,
        "next": null,
        "previous": null,
        "results": [
            {
                "id": 7,
                "creator_name": "学堂在线",
                "updater_name": "学堂在线",
                "created": "2024-04-01 17:47:11",
                "modified": "2024-04-01 17:47:11",
                "start_time": "2024-04-01 17:47:11",
                "end_time": null,
                "display_client": [
                    1,
                    2,
                    3,
                    4
                ],
                "client_exposure": {
                    "1": 455966,
                    "3": 514711,
                    "2": 157490,
                    "4": 812955
                },
                "client_hits" :{
                    "1": 10973,
                    "3": 7139,
                    "2": 2291,
                    "4": 3254
                },
                "push_status": 2,
                "title": "突破人的思维框架",
                "content": "雨课堂V6.2版本基于AI的一键出题功能，为教学拓展无限可能",
                "link": "https://www.yuketang.cn/help?detail=459"
            }
        ],
        "jump_link" : "https://www.yuketang.cn/help?list=76",
        "has_message": true
    },
    "success": true
}"#;
    use json::{TableDrivenParser, DefiniteParser, IndefiniteParser, Lexer};
    let mut parser = TableDrivenParser::new(Lexer::new(json_str));
    match parser.parse() {
        Ok(value) => {
            println!("{}", value);
        }
        Err(msg) => { eprintln!("{}", msg) }
    }
}