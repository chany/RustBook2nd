# Rust 관련
* 2016년 11월 v1.14부터 시작
* 대부분 http://rust-lang.github.io/book/second-edition/ 요약
* 2021년 1월 v1.49로 다시 정리하면서 개인적인 추가설명 더함

----
1. [기본환경 및 설치](./0.Rust_Install.md)
1. [cargo 기본 사용법](./1.cargo.md)
1. [기본 문법](./2.syntax.md)
1. [소유권(Ownership)](./3.ownership.md)
   - 소유권 이동(move), 복제(clone), 복사(copy) 
1. [참조(Reference)와 빌림(Borrowing)](./4.reference.md)
1. [슬라이스](./5.slice.md)
   - String Slice: `&str`
1. [구조체(struct)와 메소드(method)](6.struct.md)
   - `struct`, `impl`
1. [열거형(enum), Option<T>, match, if let](7.enum.md)
   - `enum`, `struct`
   - `Option<T>`
   - `if let`
1. [Package, Crate, Module](8.Package.md)
   - `module`, `super`, `self`
   - `use`
   - `as`
1. [Collections](9.Collections.md)
   - 벡터(Vector), 문자열(String), 해시맵(HashMap)
1. [에러 처리](10.Error.md)
   - `panic!`, `Result<T,E>`, `unwrap()`, `expect()`, `?`
1. [제네릭 타입](11.Generic_Type.md)
1. [트레잇(trait)](12.Traits.md)
   - trait bounds
   - blanket implementation
1. [라이프타임(Lifetime)](13.Lifetime.md)
   - `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {...`
   - lifetime 생략 규칙
   - `'static`
1. [테스트(Testing)](14.Testing.md)
   - `assert!`, `assert_eq!`, `assert_ne!`
   - `should_panic`
   - `cargo test` {`--test-threads=1`, `--nocapture` `--ignored`}
   - `tests` 디렉토리
1. [클로저(Closure)](15.Closures.md)
   - 익명 함수
   - `Fn` 트레잇
1. [반복자(Iterator)](16.Iterators.md)
   - `std::env::args()`
1. [Cargo 및 Crates.io](17.Cargo.md)
1. [스마트 포인터](18.SmartPointer.md)
1. [동시성(concurrency)](19.Threads.md)