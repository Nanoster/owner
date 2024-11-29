// fn main() {
// {                       // S가 아직 선언되지 않아서 여기서는 유효X
//     let s = "hello";    // 이 지점부터 S가 유효
// }                       // 이 스코프가 종료되었고, S가 더 이상 유효X

// let mut s = String::from("hello");
// s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가(뒤에 문자열을 붙여줌)
// println!("{}", s); // 'hello, world!'를 출력
// {
//     let s = String::from("hello"); // s는 이 지점부터 유효

//     // S를 가지고 무언가를 함
// }
// {
// let x = 5;
// let y = x;

// let s1 = String::from("hello");
// let s2 = s1;    //이 시점에서 s1은 더 이상 유효하지 않다.

// println("{}, world!",s1);    //따라서 여기서 s1은 유효하지않은 참조자이므로 에러

// let s1 = String::from("hello");
// let s2 = s1.clone();

// println!("s1 = {}, s2 = {}", s1, s2);
// }

//     let s = String::from("hello");    // s가 스코프 안에 들어옴

//     takes_ownership(s);               // s의 값이 함수 안으로 이동 -> 따라서 s는 더 이상 이 스코프 안에서 유효X

//     let x = 5;                        // x가 스코프 안에 들어옴

//     makes_copy(x);                    // x가 함수로 복사, x는 스택에 들어가는 스칼라값(i32)으로 Copy가 일어나고 여전히 이 스코프 안에서 유효o

// }

// fn takes_ownership(some_string: String) {  // some_string이 스코프 안으로 들어옴
//     println!("{}", some_string);
// }  // some_string이 스코프 밖으로 벗어나며 'drop'이 호출, 메모리가 해제 된다.

// fn makes_copy(some_integer: i32) {   // some_integer가 스코프 안으로 들어옴]
//     println!("{}", some_integer);
// }  // some_integer가 스코프 밖으로 벗어나고 별다른 호출은 하지않음,(스코프밖으로 나갔으므로 값은 버려짐)

// fn main() {
//     let s1 = gives_ownership();                     // gives_ownership 반환값(String임)을 s1로 "이동"
//     let s2 = String::from("hello");                 // s2가 스코프 안으로 들어옴
//     let s3 = takes_and_gives_back(s2);    // takes_and_gives_back에 s2가 "이동", 이 함수 또한 반환값을 s3로 "이동"
//     println!("{}",s1);
//     // println!("{}",s2);    // 이동되어 소멸하였으므로 작동x
//     println!("{}",s3);
// }    // s3가 스코프 밖으로 벗어나며 버려짐, s2는 이미 이동되며 소멸했으므로 아무일도 없음, s1도 스코프 밖으로 벗어나며 버려짐

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");    // some_string이 스코프안으로 들어옴
//     some_string                                         // some_string이 반환되고 호출자 함수로 "이동"
// }

// // 이 함수는 String을 취하고 같은 것을 반환함
// fn takes_and_gives_back(a_string: String) -> String {   // 스코프 안으로 a_string이 들어옴
//     a_string    // a_string이 반환되고 호출자 함수쪽으로 "이동"
// }

// fn main() {
//     let mut s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1);
//     s1 = s2;

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();    // s의 길이를 반환

//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     // let s = String::from("hello");
//     let mut s = String:;from("hello");

//     // change(&s);    // 빌린 값을 변경하는 함수, 작동하지않는다.
//     change(&mut s);
// }

// // fn change(some_string: &String) {
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
fn main() {
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // println!("{}", r1);

    // let r2 = &mut s;

    // // println!("{}, {}", r1, r2);
    // println!("{}", r2);
    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;

    //     // println!("{}",r1);
    // } // 여기서 r1이 스코프 밖으로 벗어나며 사라지고 이후 아무 문제없이 새 참조자 생성가능
    // let r2 = &mut s;

    // let mut s = String::from("hello");

    // let r1 = &s; // 불변 참조자
    // let r2 = &s; // 마찬가지로 불변 참조자라 여러개 생성되어도 문제 없음

    // let r3 = &mut s; // 가변 참조자, 이전에  참조사용으로 오류
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");
    {
        let r1 = &s; // 불변 참조자
        let r2 = &s; // 마찬가지로 불변 참조자라 여러개 생성되어도 문제 없음
        println!("{}, {}", r1, r2);
    } // 이 지점 이후로 변수 r1과 r2는 사용되지 않음

    {
        let r3 = &mut s; // 다른 참조자가 이후로 사용되지않으므로 생성에 문제없음(컴파일러가 판단)
        println!("{}", r3);
    }

    // let mut s = String::from("hello");

    // {
    //     let r1 = &s; // 불변 참조자
    //     let r2 = &s; // 마찬가지로 불변 참조자라 여러개 생성되어도 문제 없음
    //     println!("{}, {}", r1, r2);
    // }

    //     let r3 = &mut s; // 가변 참조자, 이전에 다른 참조사용으로 오류
    //     println!("{}", r3);
}
