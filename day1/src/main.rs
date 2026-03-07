// fn main() {
//     println!("Hello, world!");
// }

// 2.1変数とデータ型
// 変数、束縛、代入
// ダメな例
// fn main() {
//     let message = "hello, foo";
//     println!("{}", message);

//     message = "hello, bar";
//     println!("{}", message); // error[E0384]: cannot assign to `message`
// }

// 良い例
// fn main() {
//     let mut message = "hello, foo";
//     println!("{}", message);

//     message = "hello, bar";
//     println!("{}", message);
// }

// シャドーイング
// use url::Url;
// fn main() {
//     let url = Url::parse("https://www.rust-lang.org");
//     match url {
//         Ok(url) => println!("{}", url),
//         Err(e) => println!("Error: {}", e),
//     }
//     let url = Url::parse("https://www.rust-lang.org").unwrap();
//     println!("{}", url);
// }

// 定数
// const URL: &str = "https://www.rust-lang.org";
// fn main() {
//     println!("{}", URL);
// }

// 文字型
// fn main() {
//     let a = 'a';
//     let b: char = 'b';
//     println!("{}", a);
//     println!("{}", b);
// }

// タプル
// fn main() {
//     let a = ("hoge", 123);
//     println!("{}", a.0);
//     println!("{}", a.1);
// }

// 配列
// fn main() {
//     let target = ["hoge", "fuga", "piyo"];
//     println!("{}", target[0]);
//     println!("{}", target[1]);
//     println!("{}", target[2]);
// }

// 配列
//ダメな例
// fn main() {
//     let target = ["hoge", "fuga", "piyo"];
//     println!("{}", target[3]); // error: this operation will panic at runtime
// }

// ベクター型
// fn main() {
//     let mut v: Vec<i32> = Vec::new();
//     v.push(99);
//     println!("{:?}", v);
// }

// 文字列
// fn main() {
//     let message = "hello, world";
//     println!("{:?}", message);

//     let message = String::from("hello, world");
//     println!("{:?}", message);

//     let message = "hello, world";
//     let message_string = message.to_string();
//     println!("{:?}", message_string);

//     let mut s = String::from("hello");
//     s.push_str(", world");
//     println!("{:?}", s);
// }

// ハッシュマップ
// fn main() {
//     let mut scores = std::collections::HashMap::new();
//     scores.insert("Sato", 100);
//     scores.insert("Tanaka", 90);
//     println!("{:?}", scores);
//     scores.entry("Tanaka").or_insert(100);
//     println!("{:?}", scores);

//     let solar_distance = std::collections::HashMap::from([
//         ("Mercury", 0.4),
//         ("Venus", 0.7),
//         ("Earth", 1.0),
//         ("Mars", 1.5),
//     ]);
//     println!("{:?}", solar_distance);
// }

// ハッシュマップの実装
// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};

// // 1. 各要素を格納する「ノード」の定義
// #[derive(Debug)]
// struct Node<K, V> {
//     key: K,
//     value: V,
// }

// // 2. ハッシュマップ構造体の定義
// pub struct MyHashMap<K, V> {
//     // 衝突を避けるため、各スロットを Vec（リスト）にする（チェイン法）
//     buckets: Vec<Vec<Node<K, V>>>,
//     capacity: usize,
// }

// impl<K, V> MyHashMap<K, V>
// where
//     K: Hash + Eq + Clone, // キーはハッシュ可能で比較可能である必要がある
// {
//     // 初期化
//     pub fn new(capacity: usize) -> Self {
//         let mut buckets = Vec::with_capacity(capacity);
//         for _ in 0..capacity {
//             buckets.push(Vec::new());
//         }
//         Self { buckets, capacity }
//     }

//     // ハッシュ値を計算してインデックスを割り出す内部メソッド
//     fn calculate_index(&self, key: &K) -> usize {
//         let mut hasher = DefaultHasher::new();
//         key.hash(&mut hasher);
//         (hasher.finish() as usize) % self.capacity
//     }

//     // 挿入処理
//     pub fn insert(&mut self, key: K, value: V) {
//         let index = self.calculate_index(&key);
//         let bucket = &mut self.buckets[index];

//         // すでにキーが存在すれば更新、なければ追加
//         for node in bucket.iter_mut() {
//             if node.key == key {
//                 node.value = value;
//                 return;
//             }
//         }
//         bucket.push(Node { key, value });
//     }

//     // 取得処理
//     pub fn get(&self, key: &K) -> Option<&V> {
//         let index = self.calculate_index(key);
//         let bucket = &self.buckets[index];

//         bucket.iter().find(|n| &n.key == key).map(|n| &n.value)
//     }
// }

// fn main() {
//     let mut map = MyHashMap::new(10);
//     map.insert("Earth", 1.0);
//     map.insert("Mars", 1.5);

//     if let Some(dist) = map.get(&"Earth") {
//         println!("Earth distance: {}", dist);
//     }
// }

// 2.2 関数の実装
// fn main() {
//     println!("Hello, world!");
// }

// 暗黙的な戻り値
// fn void() {
//     // nothing
// }

// fn return_void() {
//     return ();
// }

// fn main() {
//     let a = void();
//     println!("{:?}", a);
//     let a = return_void();
//     println!("{:?}", a);
// }

// 明示的な戻り値があるときの型宣言
// fn add(x: i32, y: i32) -> i32 {
//     return x + y
// }

// fn main() {
//     println!("{}", add(1, 2));
// }

// if文　ダメな例
// fn add(x: i32, y: i32) -> i32 {
//     x + y; // error[E0308]: mismatched types
// }

// fn main() {
//     println!("{}", add(1, 2));
// }

// enum
// fn main() {
//     #[derive(Debug)]
//     enum Color {
//         Red,
//         Blue,
//         Green,
//         Hex(String),
//     }

//     // 以下どちらもColor型
//     let red = Color::Red;
//     let hex = Color::Hex("ffffff".to_string());

//     println!("{:?}, {:?}", red, hex);
// }

// Option<T>
// fn main() {
//     let some_value: Option<&str> = Some("こんにちは");
//     match some_value {
//         Some(msg) => {println!("{}", msg); },
//         None => {}
//     }
// }

// enum Result<T, E>
// fn main() {
//     let some: Result<&str, &str> = Ok("ok");
//     println!("{:?}", some);
//     let err: Result<&str, &str> = Err("err");
//     println!("{:?}", err);
// }

// ?演算子 即座に返すパターン
// fn always_error() -> Result<(), String> {
//     Err("常にエラーが発生します。".to_string())
// }

// fn might_fail() -> Result<(), String> {
//     let _result = always_error()?;
//     Ok(())
// }

// fn main() {
//     let message = match might_fail() {
//         Ok(_) => "処理に成功しました。".to_string(),
//         Err(cause_message) => cause_message,
//     };
//     println!("{}", message);
// }

// 回復不能なエラー
// fn main() {
//     println!("before panic!");
//     panic!("hoge");
//     println!("after panic!");
// }

// unwrap/expect
// fn main() {
//     let input: Result<&str, &str> = Ok("test");
//     let input = input.unwrap();
 
//     println!("{:?}", input);
// }

// マクロ
// 宣言的マクロ
// macro_rules! sum {
//     ( $( $x:expr),* ) => {
//         {
//             let mut result = 0;
//             $(
//                 result = result + $x;
//             )*
//             result
//         }
//     };
// }

// fn main() {
//     println!("{}", sum![1, 2, 3, 4, 5]);
// }

// 手続きマクロ
// #[get("/")]
// fn index() {
//     // ...（省略）
// }

// 2.3 制御構造
// if式
// fn fizz_buzz(value: i32) -> String {
//     let result = if value %15 == 0 {
//         "fizz buzz".to_string()
//     } else if value % 5 == 0 {
//         "buzz".to_string()
//     } else if value % 3 == 0 {
//         "fizz".to_string()
//     } else {
//         value.to_string()
//     };
//     result
// }

// fn main() {
//     println!("{}", fizz_buzz(1));
//     println!("{}", fizz_buzz(3));
//     println!("{}", fizz_buzz(5));
//     println!("{}", fizz_buzz(15));
// }

// if文
// fn main() {
//     if true {
//         println!("inner");
//     };
//     println!("outer");
// }

// match式
// #[derive(Debug, Hash)]
// enum Color {
//     Red,
//     Blue,
//     Green,
//     White,
//     Hex(String),
// }
// fn string_to_color_token(value: &str) -> Option<Color> {
//     match value {
//         "red" => Some(Color::Red),
//         "blue" => Some(Color::Blue),
//         "green" => Some(Color::Green),
//         "white" => Some(Color::White),
//         "gold" => Some(Color::Hex("gold".to_string())),
//         _ => None,
//     }
// }

// fn main() {
//     println!("{:?}",string_to_color_token("red"));
//     println!("{:?}",string_to_color_token("blue"));
//     println!("{:?}",string_to_color_token("green"));
//     println!("{:?}",string_to_color_token("white"));
//     println!("{:?}",string_to_color_token("gold"));
//     println!("{:?}",string_to_color_token(""));
// }

// use std::str::FromStr;

// #[derive(Debug, PartialEq)] // 比較もできるようにPartialEqを追加
// enum Color {
//     Red, Blue, Green, White,
//     Hex(String),
// }

// impl FromStr for Color {
//     type Err = (); // 失敗した時のエラー型（今回は単純化のため空）

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "red"   => Ok(Color::Red),
//             "blue"  => Ok(Color::Blue),
//             "green" => Ok(Color::Green),
//             "white" => Ok(Color::White),
//             "gold"  => Ok(Color::Hex("gold".to_string())),
//             _ => Err(()),
//         }
//     }
// }

// fn main() {
//     // parse() を使うと、OptionではなくResultで返ってきます
//     // 文字列に対して .parse() を呼ぶと、自動的に from_str が動く
//     let color: Result<Color, _> = "red".parse();
//     println!("{:?}", color.ok()); // .ok() で Option に変換可能
// }

// match 2
// fn fizz_buzz(value: i32) -> String {
//     let result = match value {
//         v if v % 15 == 0 => "fizz buzz".to_string(),
//         v if v % 5 == 0 => "buzz".to_string(),
//         v if v % 3 == 0 => "fizz".to_string(),
//         _ => value.to_string(),
//     };
//     result
// }

// fn main() {
//     println!("{}", fizz_buzz(1));
//     println!("{}", fizz_buzz(3));
//     println!("{}", fizz_buzz(5));
//     println!("{}", fizz_buzz(15));
// }

// match 3
// fn main() {
//     let data = Some("Some text");
//     let print_data = match data {
//         Some(text) => text,
//         None => "None text",
//     };
//     println!("{:?}", print_data);
// }

// if let
// use std::str::FromStr;
// #[derive(Debug, Hash)]
// enum Color {
//     Red,
//     Blue,
//     Green,
//     White,
//     Hex(String),
// }

// fn string_to_color_token(value: &str) -> Option<Color> {
//     match value {
//         "red" => Some(Color::Red),
//         "blue" => Some(Color::Blue),
//         "green" => Some(Color::Green),
//         "white" => Some(Color::White),
//         "gold" => Some(Color::Hex("gold".to_string())),
//         _ => None,
//     }
// }

// fn main() {
//     if let Some(color) = string_to_color_token("red") {
//         println!("red");
//     }
// }
// impl FromStr for Color {
//     type Err = (); // 失敗した時のエラー型（今回は単純化のため空）

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "red"   => Ok(Color::Red),
//             "blue"  => Ok(Color::Blue),
//             "green" => Ok(Color::Green),
//             "white" => Ok(Color::White),
//             "gold"  => Ok(Color::Hex("gold".to_string())),
//             _ => Err(()),
//         }
//     }
// }

// fn main() {
//     // parse() を使うと、OptionではなくResultで返ってきます
//     // 文字列に対して .parse() を呼ぶと、自動的に from_str が動く
//     let result = "red".parse::<Color>();
//     if let Ok(c) = result {
//         println!("{:?}", c); // .ok() で Option に変換可能
//     }
// }

// loop式
// fn add_until(start: i32, end: i32) -> i32 {
//     let mut sum = 0;
//     let mut temp = start;
//     loop {
//         sum += temp;
//         if temp == end {
//             break sum;
//         }
//         temp += 1;
//     }
// }

// fn main() {
//     let result = add_until(1, 3);
//     println!("{}", result);
// }

// while式
// fn add_until(start: i32, end: i32) -> i32 {
//     let mut sum = 0;
//     let mut temp = start;
//     while temp <= end {
//         sum += temp;
//         temp += 1;
//     }
//     sum
// }

// fn main() {
//     let result = add_until(100, 300);
//     println!("{}", result);
// }

// for文
// fn main() {
//     let scores = vec![100, 30, 80, 70, 95];
//     for score in scores.iter() {
//         println!("score is {}", score);
//     }
// }

// 所有権 ダメな例
// fn main() {
//     let a = String::from("hoge");
//     let b = a;
//     println!("{}", a); // error[E0382]: borrow of moved value: `a`
// }

// fn main() {
//     let a = 100;
//     let b = a;
//     println!("{}", a); // i32はCopyトレイトを実装している
// }

/*
所有権まとめ
・所有権は1つの変数でのみ保持される
・変数束縛時に値がCopyトレイトを実装していると値がcopyされる
・変数束縛時に値がCopyトレイトを実装していないと所有権がmoveする
*/

// 借用 ダメな例
// fn hello_print(message: String) {
//     println!("hello, {}", message);
// }

// fn main() {
//     let world = String::from("world");
//     hello_print(world);
//     println!("{}", world);
// }

// fn hello_print(message: &str) {
//     println!("{}", message);
// }

// fn main() {
//     let world = String::from("world");
//     hello_print(&world);
//     println!("{}", world); // 借用なのでエラーにならない
// }

// 参照はずし
// fn print(value: i32) {
//     println!("{}", value);
// }

// fn main() {
//     let a = 999;
//     let b = &a;
//     print(*b);
//     print(a);
//     let a = String::from("hello");
//     let b = &a;
//     println!("{}", *b);
//     println!("{}", a);
// }

// clone
// fn take<T>(_value: T) {}
// fn fizz(value: i32) -> String {
//     let result = if value % 3 == 0 {
//         String::from("fizz")
//     } else {
//         format!("{}", value)
//     };
//     let cloned_result = result.clone();
//     take(cloned_result);
//     result
// }

// fn main() {
//     let message = fizz(3);
//     println!("{}", message);
//     let message = fizz(1);
//     println!("{}", message);
// }

/*
croneまとめ
・croneは明示的に値をコピーする。一方でcopyは束縛時に暗黙的に実行される
・#[derive(Clone)]で簡単に実装される
 */

 // ライフタイム
 // ダメな例
//  fn hello() -> &str {
//     "hello" // error[E0106]: missing lifetime specifier
//  }

//  fn main() {
//     let message = hello();
//  }
// OKな例
// fn hello() -> &'static str {
//     "hello" 
//  }

//  fn main() {
//     let message = hello();
//  }
// fn hello(message: &str) -> &str {
//     println!("{}", message);
//     "hello" 
//  }

//  fn main() {
//     let message = "world";
//     let message = hello(message);
//  }
// fn hello<'a>() -> &'a str {
//     "hello" 
//  }

//  fn main() {
//     let message = hello();
//  }

// Box<T>
// fn main() {
//     let a = Box::new("test");
//     println!("{}", a);
// }

// 2.5 データ構造
// 構造体
// struct User {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let user = User {
//         name: "sato".to_string(),
//         age: 30,
//     };
//     println!("name is {}, age is {}", user.name, user.age);
// }

// メソッド、関連関数
// struct User {
//     name: String,
//     age: u32,
// }

// impl User {
//     // 関連関数
//     fn new(name: String, age: u32) -> Self {
//         // selfはここではUserをあらわす
//         Self {
//             name,
//             age,
//         }
//     }

//     // メソッド
//     fn description(&self) -> String {
//         format!("user name is {}, age is {}", self.name, self.age)
//     }
// }

// fn main() {
//     let user = User::new(String::from("sato"), 20);
//     println!("{}", user.description());
// }

// トレイト 1
// use std::fmt::{self, Display};

// struct User {
//     name: String,
//     age: u32,
// }

// impl Display for User {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "user name is {}, age is {}", &self.name, &self.age)
//     }
// }

// impl User {
//     fn new(name: String, age: u32) -> Self {
//         User {name, age}
//     }
// }

// fn main() {
//     let user = User::new("sato".to_string(), 20);
//     println!("{}", user);
// }

// トレイト 2
// trait Area {
//     fn area(&self) -> u32;
// }

// // 1辺の長さを保持
// struct Square(u32);

// impl Area for Square {
//     fn area(&self) -> u32 {
//         self.0.pow(2)
//     }
// }

// impl Square {
//     fn new(side: u32) -> Self {
//         Self(side)
//     }
// }

// fn main() {
//     let my_square = Square::new(5);
//     println!("{}", my_square.area());
// }

// トレイト 3
// use std::fmt::{self, Display};

// // Fraction: 分数
// struct Fraction(u32, u32);

// impl Fraction {
//     // numerator: 分子、denominator: 分母
//     fn new(numerator: u32, denominator: u32) -> Self {
//         let gcf_value = Self::gcf(numerator, denominator);
//         Self(numerator / gcf_value, denominator / gcf_value)
//     }

//     // 最大公約数を計算(greatest common factor: gcf)を計算
//     fn gcf(value1: u32, value2: u32) -> u32 {
//         // ユークリッドの互除法
//         let (mut a, mut b) = if value1 > value2 {
//             (value1, value2)
//         } else {
//             (value2, value1)
//         };
//         let mut r = a %b;
//         while r != 0{
//             a = b;
//             b = r;
//             r = a % b;
//         }
//         b
//     }
// }

// impl Display for Fraction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}/{}", &self.0, &self.1)
//     }
// }

// fn main() {
//     let a = Fraction::new(8, 18);
//     println!("{}", a);
// }

// use std::ops::Add;

// impl Add for Fraction {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         let lcm = self.1 / Self::gcf(self.1, other.1) * other.1;
//         let a = self.0 * (lcm / self.1);
//         let b = other.0 * (lcm / other.1);
//         Fraction::new(a + b, lcm)
//     }
// }

// fn main() {
//     let a = Fraction::new(8, 18);
//     let b = Fraction::new(1, 2);
//     println!("{}", a + b);
// }

