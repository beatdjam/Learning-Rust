<!-- TOC -->

- [1. Learning-Rust](#1-learning-rust)
    - [1.1. 変数](#11-変数)
    - [1.2. Result型](#12-result型)
    - [1.3. println!マクロのプレースホルダ](#13-printlnマクロのプレースホルダ)
    - [1.4. crate](#14-crate)
    - [1.5. シャドーイング](#15-シャドーイング)
    - [1.6. タプル](#16-タプル)
    - [1.7. 配列型](#17-配列型)
    - [1.8. 関数](#18-関数)
    - [1.9. 所有権](#19-所有権)
        - [1.9.1. 所有権の移動(ムーブ)](#191-所有権の移動ムーブ)
        - [1.9.2. 所有権と関数](#192-所有権と関数)
        - [1.9.3. 参照と借用](#193-参照と借用)
            - [1.9.3.1. 可変な参照](#1931-可変な参照)
            - [スライス型](#スライス型)

<!-- /TOC -->

# 1. Learning-Rust
[![CircleCI](https://circleci.com/gh/beatdjam/Learning-Rust.svg?style=svg)](https://circleci.com/gh/beatdjam/Learning-Rust)  
参考サイト : https://doc.rust-jp.rs/book/second-edition/  
参考書籍 :  [実践Rust入門　[言語仕様から開発手法まで]](https://amzn.to/2NfSDF5)

## 1.1. 変数  
変数宣言には `let`。デフォルトで不変でmutをつけると可変になる。  
変数の参照には `&` をつける。参照も `&mut` とすることで可変にできる。  
つまり、下記のようにすることで、`Stringとして宣言したguessを可変の状態でread_line()に渡す`事ができる。
```rust
let mut guess = String::new();
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

## 1.2. Result型  
標準ライブラリにはResultと名前のついた型を返す。  
Result型はenumで、成功した場合は`Ok`を、失敗した場合は`Err`を保持する。  
enumなのでそれぞれの列挙子から取り出すことも出来るが、`Result.expect()`を用いることで、  
`Ok`を取り出したのと同じ効果を得ることが出来る。  
Result型は、`Err`が返却されたときの挙動を記述していない場合はコンパイラが警告を出す。  

## 1.3. println!マクロのプレースホルダ  
Rustの`println!`のプレースホルダは、文字列中に含めた`{}`で表現する。  
変数は複数渡すことができ、渡した数に応じて文字列の先頭から`{}`に相当する値が挿入されていく。  

## 1.4. crate  
crateはRustのパッケージ。  
ライブラリとして読み込んで利用できる。  
cargoを利用したプロジェクトであれば、tomlのdependenciesに定義を足すだけで利用できる。  
crate内の個別のtraitを利用するときには、スコープ内で`use rand::Rng`などしてやる必要がある

## 1.5. シャドーイング  
Rustでは同じ名前の変数に値を上書きして置き換えることが出来る`シャドーイング`を利用できる。  
値の変換などで利用される事が多い。  
宣言済みの変数名に対して`let`で再度宣言することで行える。

## 1.6. タプル
複数の型をひとまとめにする型。パターンマッチングで分解もできる。  
`let tup: (i32, f64, u8) = (500, 6.4, 1);`  
タプルの要素に直接アクセスする場合はこう書く
```rust
tup.0 // 500
tup.1 // 6.4
tup.2 // 1
```

## 1.7. 配列型
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

## 1.8. 関数
関数は文と式を含んだ形で定義される。  
式の戻り値は値を返すので格納出来るが、文は戻り値が無い。  
`関数スコープの末尾の式の戻り値が関数の戻り値となる。`  
スコープの末尾の式は、行末にセミコロンを書かないことに注意。(書くと文として認識される)  
`returnキーワードを利用すればアーリーリターンを行うことも出来る。`  

## 1.9. 所有権
Rustの値は、所有者と呼ばれる変数に対応している。  
基本的に所有者は一つであり、所有者がスコープ上から消えると、値も破棄される。  

### 1.9.1. 所有権の移動(ムーブ)
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

### 1.9.2. 所有権と関数
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

### 1.9.3. 参照と借用
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

#### 1.9.3.1. 可変な参照
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

#### スライス型
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
    let result = first_word_slice(&value);
    println!("value : {}", value); // value : Hello world
    println!("result : {}", result); // result : Hello

    // resultがvalueの一部を不変の参照で借用しているため、
    // valueの値を変更することができずエラーとなるので実行できない
    // value = String::from("hogefuga");
}

// 文字列から最初の単語のsliceを取り出す関数
fn first_word_slice(s: &String) -> &str {
    match s.as_bytes().iter().enumerate().find(|(_i, &x)| x == b' ') {
        None => &s[..],
        Some(pair) => &s[..pair.0],
    }
}
```