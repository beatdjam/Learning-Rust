<!-- TOC -->

- [1. Learning-Rust](#1-learning-rust)
    - [1.1. 普遍的なプログラミング概念](#11-普遍的なプログラミング概念)
        - [1.1.1. 変数](#111-変数)
        - [1.1.2. Result型](#112-result型)
        - [1.1.3. println!マクロのプレースホルダ](#113-printlnマクロのプレースホルダ)
        - [1.1.4. crate](#114-crate)
        - [1.1.5. シャドーイング](#115-シャドーイング)
        - [1.1.6. タプル](#116-タプル)
        - [1.1.7. 配列型](#117-配列型)
        - [1.1.8. 関数](#118-関数)
    - [1.2. 所有権を理解する](#12-所有権を理解する)
        - [1.2.1. 所有権](#121-所有権)
            - [1.2.1.1. 所有権の移動(ムーブ)](#1211-所有権の移動ムーブ)
            - [1.2.1.2. 所有権と関数](#1212-所有権と関数)
            - [1.2.1.3. 参照と借用](#1213-参照と借用)
                - [1.2.1.3.1. 可変な参照](#12131-可変な参照)
                - [1.2.1.3.2. スライス型](#12132-スライス型)
    - [1.3. 構造体](#13-構造体)
        - [1.3.1. 構造体の定義とインスタンスの生成](#131-構造体の定義とインスタンスの生成)
            - [1.3.1.1. フィールド初期化省略記法](#1311-フィールド初期化省略記法)
            - [1.3.1.2. 構造体更新記法](#1312-構造体更新記法)
            - [1.3.1.3. 構造体のデバッグ](#1313-構造体のデバッグ)
        - [1.3.2. タプル様構造体](#132-タプル様構造体)
        - [1.3.3. 構造体のメソッドと関連関数](#133-構造体のメソッドと関連関数)
            - [1.3.3.1. メソッド](#1331-メソッド)
            - [1.3.3.2. 関数](#1332-関数)
    - [1.4. Enumとパターンマッチング](#14-enumとパターンマッチング)
        - [1.4.1. Enumの定義](#141-enumの定義)
        - [1.4.2. パターンマッチング](#142-パターンマッチング)
            - [1.4.2.1. matchフロー演算子](#1421-matchフロー演算子)
            - [1.4.2.2. if let](#1422-if-let)
            - [1.4.2.3. Option](#1423-option)

<!-- /TOC -->

# 1. Learning-Rust
[![CircleCI](https://circleci.com/gh/beatdjam/Learning-Rust.svg?style=svg)](https://circleci.com/gh/beatdjam/Learning-Rust)  
参考サイト : https://doc.rust-jp.rs/book/second-edition/  
参考書籍 :  [実践Rust入門　[言語仕様から開発手法まで]](https://amzn.to/2NfSDF5)

## 1.1. 普遍的なプログラミング概念

### 1.1.1. 変数  
変数宣言には `let`。デフォルトで不変でmutをつけると可変になる。  
変数の参照には `&` をつける。参照も `&mut` とすることで可変にできる。  
つまり、下記のようにすることで、`Stringとして宣言したguessを可変の状態でread_line()に渡す`事ができる。
```rust
let mut guess = String::new();
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

### 1.1.2. Result型  
標準ライブラリにはResultと名前のついた型を返す。  
Result型はenumで、成功した場合は`Ok`を、失敗した場合は`Err`を保持する。  
enumなのでそれぞれの列挙子から取り出すことも出来るが、`Result.expect()`を用いることで、  
`Ok`を取り出したのと同じ効果を得ることが出来る。  
Result型は、`Err`が返却されたときの挙動を記述していない場合はコンパイラが警告を出す。  

### 1.1.3. println!マクロのプレースホルダ  
Rustの`println!`のプレースホルダは、文字列中に含めた`{}`で表現する。  
変数は複数渡すことができ、渡した数に応じて文字列の先頭から`{}`に相当する値が挿入されていく。  

### 1.1.4. crate  
crateはRustのパッケージ。  
ライブラリとして読み込んで利用できる。  
cargoを利用したプロジェクトであれば、tomlのdependenciesに定義を足すだけで利用できる。  
crate内の個別のtraitを利用するときには、スコープ内で`use rand::Rng`などしてやる必要がある

### 1.1.5. シャドーイング  
Rustでは同じ名前の変数に値を上書きして置き換えることが出来る`シャドーイング`を利用できる。  
値の変換などで利用される事が多い。  
宣言済みの変数名に対して`let`で再度宣言することで行える。

### 1.1.6. タプル
複数の型をひとまとめにする型。パターンマッチングで分解もできる。  
`let tup: (i32, f64, u8) = (500, 6.4, 1);`  
タプルの要素に直接アクセスする場合はこう書く
```rust
tup.0 // 500
tup.1 // 6.4
tup.2 // 1
```

### 1.1.7. 配列型
同じ型をひとまとめにする型。Rustの配列は固定長であることに注意。  
```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```
配列の要素にアクセスする場合は下記。
```rust
months[1]; // January
```
配列外アクセスを行った場合は`panic`して終了する。

### 1.1.8. 関数
関数は文と式を含んだ形で定義される。  
式の戻り値は値を返すので格納出来るが、文は戻り値が無い。  
`関数スコープの末尾の式の戻り値が関数の戻り値となる。`  
スコープの末尾の式は、行末にセミコロンを書かないことに注意。(書くと文として認識される)  
`returnキーワードを利用すればアーリーリターンを行うことも出来る。`
  
## 1.2. 所有権を理解する

### 1.2.1. 所有権
Rustの値は、所有者と呼ばれる変数に対応している。  
基本的に所有者は一つであり、所有者がスコープ上から消えると、値も破棄される。  

#### 1.2.1.1. 所有権の移動(ムーブ)
変数の型が値を持つプリミティブな型(`Copyトレイトを実装している型`)の場合は異なる変数に値をコピーすることができる  
```rust
let s1 = "hello";
let s2 = s1;
println!("{}", s1);
println!("{}", s2);
```
が、プリミティブではない型で同様の操作を行った場合、`所有権の移動(ムーブ)`が起きる。  
```rust
let s1 = "hello";
let s2 = s1;
println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
println!("{}", s2);
```
Rustでは所有権のスコープから離れると変数の値のメモリが破棄されるため、複数の変数が同じポインタを見ることは許容されない。    
そのため、シャローコピーの仕組みが存在せず、所有権の移動のみが起きる。  

ムーブを行わず、ディープコピーを行いたい場合にはクローンを利用する。  
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}", s1);
println!("{}", s2);
```

#### 1.2.1.2. 所有権と関数
関数の引数に値を渡すことは、Rustでは変数に値を代入することと同様に扱われる。  
そのため関数に値を渡した場合には、ムーブや値のコピーが行われてしまう。  
ムーブが行われた場合、関数の引数に所有権が渡され、関数内のスコープから離れると同時に値が破棄されてしまう。  
```rust
fn main() {
    let s1 = String::from("hello");
    move_s1(s1);
    println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
}

fn move_s1(s1: String) {
    println!("{}", s1);
}
```
ただし、関数が値を返すことによって、更にムーブや値のコピーが起こるため、下記のようなコードは有効である。
```rust
fn main() {
    let s1 = String::from("hello");
    let s1 = move_s1(s1); // 関数の戻り値で得た所有権をシャドーイングで再度割当て
    println!("{}", s1);
}

fn move_s1(s1: String) -> String {
    s1
}
```

#### 1.2.1.3. 参照と借用
先述のような所有権のやり取りを都度行うのは効率的でないため、`参照`と`借用`という仕組みがある。  
Rustでは、変数に`&`をつけることで、`変数の値への参照`を生成することが出来る。  
また、参照を関数の引数に取ることを`借用`と呼ぶ。
```rust
fn main() {
    let s1 = String::from("hello");
    // 関数に値の参照を渡す
    borrowing_s1(&s1); 
    // 関数で使用した後も値が生存してる！
    println!("{}", s1); 
}

fn borrowing_s1(s1: &String) {
    // 参照を借用しているだけなので所有権は発生していない
    println! {"No Move! {}",s1} 
}
```
この参照は関数に借用されるなどしても、所有権のやり取りが発生することはない。  
関数内に所有権が移されていないので、関数のスコープを抜けても元の変数・値は生存している。  
参照は`不変の状態で渡される`ため、関数内でこの値を変更することはできない。  

##### 1.2.1.3.1. 可変な参照
関数などに可変な状態で値を渡したい時には、`可変な参照`を利用することが出来る。  
可変で宣言した変数を、`&mut`をつけて関数に渡し、引数の宣言に`&mut {型}`を記述ことで可能になる。  
この値は`可変参照`として与えられたスコープの中で中身を変更することができ、  
元のスコープに戻った後もその値が維持される。  
ただし、可変な参照を持てるのは`一つの値に対してスコープ内で1つ`のみとなる。  
また、`不変な借用が行われているスコープ内で可変の借用を行うことはできない`。
```rust
fn main() {
    // 可変な値を宣言する
    let mut s1 = String::from("hello");
    mutable_s1(&mut s1); // 関数に可変な参照を渡す
    println!("{}", s1); // 可変な参照を経由して値が変更されている
}

fn mutable_s1(s1: &mut String) {
    // 値を加工
    s1.push_str(" world"); 
}
```

##### 1.2.1.3.2. スライス型
例えば文字列の中から最初の単語を取り出したいようなとき、  
最初の単語の長さを取得して文字列から取り出すことで実現することが出来る。  
ただし、単語の長さは `取得時点の値` でしか無いため、その値がどの時点でも参照できるかは保証されない。
```rust
fn main() {
    let mut value = String::from("Hello world");
    let strlen = first_word_length(&value);
    // 得られた単語の長さで、文字列から単語を取り出すことは出来る
    println!("{}", value.get(0..strlen).expect("err")); // Hello

    // でもvalueの値を変えてしまえば値がstrlenの値は意味がなくなってしまう
    value = String::from("hogefuga");
    println!("{}", value.get(0..strlen).expect("err")); // hogef
}

// valueの最初の空白または行末までの長さを求める関数
fn first_word_length(s: &String) -> usize {
    match s.as_bytes().iter().enumerate().find(|(_i, &x)| x == b' ') {
        None => s.len(),
        Some(pair) => pair.0,
    }
}
```

これを解決するために、スライス型を利用することが出来る。  
スライス型とはコレクションの中の一連の部分について参照を持つための型。  
文字列スライスの形で単語を取り出すことによって、元の文字列に対して不変の参照が発生し、  
その参照が有効な状態では元の文字列を変更できなくなる。  
```rust
fn main() {
    let mut value = String::from("Hello world");
    {
        let result = first_word_slice(&value);
        println!("value : {}", value); // value : Hello world
        println!("result : {}", result); // result : Hello

        // resultがvalueの一部を不変の参照で借用しているため、
        // valueの値を変更することができずエラーとなるので実行できない
        // value = String::from("hogefuga");
    }

    // sliceによって不変な参照が発生しているのは別スコープのため、
    // valueの値を変更することが出来る
    value = String::from("hogefuga");
}

// 文字列から最初の単語のsliceを取り出す関数
fn first_word_slice(s: &String) -> &str {
    match s.as_bytes().iter().enumerate().find(|(_i, &x)| x == b' ') {
        None => &s[..],
        Some(pair) => &s[..pair.0],
    }
}
```

## 1.3. 構造体

### 1.3.1. 構造体の定義とインスタンスの生成
構造体とは、タプルのように値をひとかたまりで持つことの出来る型のこと。  
タプルと異なる点は、  
* 構造体自体に名前をつけて型として扱えること
* 構造体に持たせる値に型だけでなく名前をつけてフィールドとして扱うことが出来ること
* 構造体の型に値をもたせたインスタンスを生成出来ること

フィールドには`インスタンス.フィールド名`でアクセス出来る。  
基本的には不変だが、可変のインスタンスを作成することで、中の値を編集できる。  
**※フィールド単位での可変・不変の制御は不可**
```rust
// 構造体の宣言
struct Person {
    name: String,
    email: String,
    age: u64,
    is_adult: bool,
}

fn main() {
    // 太郎の情報を持ったインスタンス
    let tarou = Person {
        name: String::from("Tarou"),
        email: String::from("tarou@example.com"),
        age: 24,
        is_adult: true,
    };
    // 値にはドットでアクセス可能
    println!("{}", tarou.name);

    // 可変のインスタンスを作成することで、中の値を編集できる
    let mut hiroshi = Person {
        name: String::from("Hiroshi"),
        email: String::from("hiroshi@example.com"),
        age: 15,
        is_adult: false,
    };
    hiroshi.email = String::from("hiroshi-forever@example.com");
    println!("{}", hiroshi.email);
}
```

#### 1.3.1.1. フィールド初期化省略記法
また、フィールドと変数名が同じ時には、フィールドの初期化を簡易的に行う記法を利用できる。  
```rust
fn build_person(name: String, email: String, age: u64) -> Person {
    // インスタンスの作成時に、フィールド名と同名の変数はkeyを省略できる
    Person {
        name,
        email,
        age,
        is_adult: false,
    }
}
```

#### 1.3.1.2. 構造体更新記法
別の構造体をベースにしつつ、一部の値を変更して新しいインスタンスを作成することも出来る  
```rust
let tarou2 = Person {
    name: String::from("AnotherTarou"),
    // 残りの値を別のインスタンスの値で初期化する
    ..tarou
};
```

#### 1.3.1.3. 構造体のデバッグ
構造体は、そのままでは`println!`マクロでフィールドなどを出力して確認することができない。  
これは、`println!`は標準では`Displayトレイト`を受け取るが、構造体はこのトレイトを持っていないため。  
RustにはDebug用の出力を行う機能と`Debugトレイト`が存在しており、下記のように注釈をすることで、  
インスタンスの中の値などを確認することが出来る。  
```rust
// Debugトレイトを継承する注釈
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // Debug出力を行うための指定子 {:?}
    // 出力結果: rect1 is Rectangle { width: 30, height: 50 }
    println!("rect1 is {:?}", rect1); }
}
```

### 1.3.2. タプル様構造体
タプルのように値だけをもつが、構造体として意味をもたせられるタプル様構造体も存在する  
下記のように、同じ型を同じ数だけ持っているタプル様構造体であっても、  
自身の型が異なっているためそれぞれを同様に扱うなどは行えない
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
```

### 1.3.3. 構造体のメソッドと関連関数
構造体には、`impl`ブロック内でインスタンスやその型に紐づくメソッド・関数を定義することが出来る。  
これらは複数定義することができるが、通常の利用ケースでは分ける意味は薄い。

#### 1.3.3.1. メソッド
引数に`&self`や`self`がある場合、インスタンスに紐づくメソッドとして定義される。   
```rust
struct Name {
    first_name : String,
    last_name : String,
}

// 構造体のインスタンスから呼び出せる関数を定義できる
impl Name {
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, &self.last_name)
    }
}

fn main() {
    let tarou = Name {
        first_name: String::from("Tarou"),
        last_name:  String::from("Tanaka"),
    };
    println!("{}", tarou.get_full_name());
}
```

#### 1.3.3.2. 関数
引数に`&self`や`self`がない場合、型に紐づく関数として定義される。    
```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
// 型名::で呼び出す。
// String::new()などでよく見る形
let sq = Rectangle::square(3);
```

## 1.4. Enumとパターンマッチング

### 1.4.1. Enumの定義
`Enum`は、取りうる値をすべて列挙したものを1つの型として扱う仕組み。  
`Enum型::列挙子名`でインスタンスを生成出来る。
それぞれの列挙子毎にフィールドを持つことが可能で、  
このフィールドは型が同じである必要はなく、匿名の構造体を持つことなども出来る。  
また、構造体のように`impl`を用いてメソッドなどを定義することも可能である。
```rust
enum Message {
    // フィールドを持たない
    Quit,
    // 匿名構造体を持つ
    Move { x: i32, y: i32 },
    // 文字列を持つ
    Write(String),
    // タプル様の値を持つ
    ChangeColor(i32, i32, i32),
}

// Enum型にメソッドを定義する
impl Message {
    fn get_message(&self) {
        match self {
            Message::Write(s) => println!("{}", s),
            _ => println!("None"),
        }
    }
}

fn main() {
    let message = Message::Write(String::from("hello"));
    // メソッド呼び出し
    message.get_message();
}
```

### 1.4.2. パターンマッチング

#### 1.4.2.1. matchフロー演算子
`match`演算子はパターンに対して与えた値を比較し、パターンに応じたコードを実行させる演算子。   
`match`のアームは式であり、実行された時の戻り値が`match`の戻り値となる。  
`match`は取りうる値のケースを網羅する必要がある。ただし、一部の可能性のみを対象としたい場合、  
`_`というパターンを用いることで、残りのケースをマッチさせることが出来る。  
パターンにマッチした値の一部に束縛することで、列挙子のインスタンスから値を取り出すことも出来る。 
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn get_message(&self) -> &str {
        // matchのアームはすべて&strの値を返しており、
        // そのまま関数の戻り値となっている。
        match self {
            // Message::Writeのインスタンスが持つStringの値をsとして束縛
            // 式のスコープ内で利用している。
            Message::Write(s) => &s[..],
            _ => "None",
        }
    }
}

fn main() {
    let message = Message::Write(String::from("hello"));
    println!("{}", message.get_message());
}
```

#### 1.4.2.2. if let
matchを利用したいようなケースでも、該当するパターンが1つだけの場合は`if let`記法が利用できる。  
`if let`は`else`を持つことができるが、これは1つだけしかアームのないmatch式のようなもので、  
取りうる型を網羅することは求められない。  
また、matchと同様に式を実行すること、パターンマッチさせることができる。つまり、以下は等価となる。  
```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (), // ()は何も処理を行わないユニット値
}
```

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```
`if let`は、合わせて`else`や、`else`に続けて再度`else if let`の形で利用して、  
マッチさせるパターンを増やすことも出来るが、この場合は`match`を利用するのが良い。
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn get_message(msg: Message) {
    if let Message::Quit = msg {
        println!("Quit")
    } else if let Message::Move { x, y } = msg {
        println!("x: {}, y: {}", x, y)
    } else if let Message::Write(s) = msg {
        println!("{}", s)
    } else {
        println!("ChangeColor")
    }
}
```

#### 1.4.2.3. Option
Rustには、値を取るか取らないかがわからない状態を表現するために、`Option<T>`型が存在する。  
これは、下記の形のEnumである。  
```rust
enum Option<T> {
    Some(T),
    None,
}
```
このEnumは、T型の値を持っていることを表す`Some(T)`と、値の存在しない`None`の2つの列挙子を持っている。  
つまり、`Option<T>`型は、パターンマッチによって`Some(T)`を取り出せたときに、値が存在することを表すことができる。