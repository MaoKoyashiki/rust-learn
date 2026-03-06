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