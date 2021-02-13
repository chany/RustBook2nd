fn main() {
    let s1 = String::from("01234 5678");
    let pos1 = first_word(&s1);
    println!("position of s1: {}", pos1);

    let s2 = String::from("012345678");
    let pos2 = first_word(&s2);
    println!("position of s2: {}", pos2);

    let s3 = String::from("영일이 삼사");
    let pos3 = first_word(&s3);
    println!("position of s3: {}", pos3);

    //----
    let s = String::from("hello world");
    let _hello1 = &s[0..5];
    let _hello2 = &s[..5];
    let _world1 = &s[6..11];
    let _world2 = &s[6..];
    let _hello_world = &s[..];

    //----
    let mut s = String::from("hello world");
    let word = first_word2(&s);     // s의 불변참조 생성
    s.clear();      // 오류 발생
                    // s의 불변참조가 존재하는 상황에서 내부적으로 가변참조 생성
                    // data race 규칙 위반
    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}