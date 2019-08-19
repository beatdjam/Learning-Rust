# Learning-Rust
[![CircleCI](https://circleci.com/gh/beatdjam/Learning-Rust.svg?style=svg)](https://circleci.com/gh/beatdjam/Learning-Rust)

## 変数  
変数宣言には `let`。デフォルトで不変でmutをつけると可変になる。  
変数の参照には `&` をつける。参照も `&mut` とすることで可変にできる。  
つまり、下記のようにすることで、`Stringとして宣言したguessを可変の状態でread_line()に渡す'事ができる。
```rust
let mut guess = String::new();
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

## Result型  
標準ライブラリにはResultと名前のついた型を返す。  
Result型はenumで、成功した場合は`Ok`を、失敗した場合は`Err`を保持する。  
enumなのでそれぞれの列挙子から取り出すことも出来るが、`Result.expect()`を用いることで、  
`Ok`を取り出したのと同じ効果を得ることが出来る。  
Result型は、`Err`が返却されたときの挙動を記述していない場合はコンパイラが警告を出す。  

## println!マクロのプレースホルダ  
Rustの`println!`のプレースホルダは、文字列中に含めた`{}`で表現する。  
変数は複数渡すことができ、渡した数に応じて文字列の先頭から`{}`に相当する値が挿入されていく。  

## crate  
crateはRustのパッケージ。  
ライブラリとして読み込んで利用できる。  
cargoを利用したプロジェクトであれば、tomlのdependenciesに定義を足すだけで利用できる。  
crate内の個別のtraitを利用するときには、スコープ内で`use rand::Rng`などしてやる必要がある

## シャドーイング  
Rustでは同じ名前の変数に値を上書きして置き換えることが出来る`シャドーイング`を利用できる。  
値の変換などで利用される事が多い。  
宣言済みの変数名に対して`let`で再度宣言することで行える。

## タプル
複数の型をひとまとめにする型。パターンマッチングで分解もできる。  
`let tup: (i32, f64, u8) = (500, 6.4, 1);`  
タプルの要素に直接アクセスする場合はこう書く
```rust
tup.0 // 500
tup.1 // 6.4
tup.2 // 1
```

## 配列型
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

## 関数
関数は文と式を含んだ形で定義される。  
式の戻り値は値を返すので格納出来るが、文は戻り値が無い。  
`関数スコープの末尾の式の戻り値が関数の戻り値となる。`  
スコープの末尾の式は、行末にセミコロンを書かないことに注意。(書くと文として認識される)  
`returnキーワードを利用すればアーリーリターンを行うことも出来る。`  

## 所有権
Rustの値は、所有者と呼ばれる変数に対応している。  
常に所有者は一つであり、所有者がスコープ上から消えると、値も破棄される。  
### 所有権の移動(ムーブ)
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
### 所有権と関数
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