use rand;
use rand::Rng;
use std::io;
fn main() {
    println! ("[매크로 버전: 0.1.9]\n만든사람: Nick un BAN\n----------");
    println! ("씨오씨 캐릭터 만드는 매크로 프로그램\n");
    println! ("캐릭터 이름을 입력해 주세요.\n");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name)
        .expect("입력값을 읽지 못했습니다.");
    let user_name = user_name.trim();
    println!("\n캐릭터의 이름: {}", user_name);
    generater();
    loop {
    println!("\n다시 굴리고 싶으면 엔터를 눌러주세요.");
    println!("매크로 종료를 원하면 e 입력.");
    let mut word = String::new();
    io::stdin().read_line(&mut word)
        .expect("입력값을 읽지 못했습니다.");
    //엔터를 누르면 반복, e를 누르면 종료합니다
    if word == "\n" {
        println!("캐릭터의 이름: {}", user_name);
        generater();
        }
    let word = word.trim();
    if word == "e" {
    println! ("매크로를 종료합니다.");
            break;
        }
    }
}

fn generater() {
    let sta: i16 = dice_roll_3(); //근력
    let dex = dice_roll_3(); //민첩
    let pow = dice_roll_3(); //정신
    let con = dice_roll_3(); //건강
    let app = dice_roll_3(); //외모
    let edu = dice_roll_2(); //교육
    let siz = dice_roll_2(); //크기
    let int = dice_roll_2(); //지능
    let luk = dice_roll_3(); //행운
    let job = edu * 4;
    let hob = pow * 2;
    let all = job + hob;
    let ave = (sta+dex+pow+con+app+edu+siz+int+luk) / 9;
    println! ("[근력: {}| 민첩: {}| 정신: {}]\n[건강: {}| 외모: {}| 교육: {}]\n[크기: {}| 지능: {}| 행운: {}]\n", sta, dex, pow, con, app, edu, siz, int, luk);
    println! ("능력치 평균값  : {} \n직업점수 기댓값: {}\n관심점수 기댓값: {}\n종합점수 기댓값: {}", ave, job, hob, all);
}

fn dice_roll_3() -> i16 {
    let d = rand::thread_rng().gen_range(3..19); //3~18 사이의 숫자 생성
    return d * 5
}

fn dice_roll_2() -> i16 {
    let d = rand::thread_rng().gen_range(2..13);
    return d * 5
}