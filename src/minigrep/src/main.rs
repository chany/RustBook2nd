use std::env;       // 인자 받기 위한 준비
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec::<String> = env::args().collect();
    // args()는 iterator 반환: 하나의 연속된 값
    // iterator에 collect() 호출을 통해 일련의 값들을 벡터로 변환

    //println!("{:?}", args);
    
    //let query = &args[1];
    //let filename = &args[2];
    //let (query, filename) = parse_config(&args);
    //let config = parse_config(&args);
    //let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {  // closure
        eprintln!("아규먼트 파싱 문제: {}", err);
        process::exit(1);
    });

    println!("무엇을? {}", config.query);
    println!("어디에서? {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}

