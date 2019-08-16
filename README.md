# Learning-Rust
[![CircleCI](https://circleci.com/gh/beatdjam/Learning-Rust.svg?style=svg)](https://circleci.com/gh/beatdjam/Learning-Rust)

## 変数  
変数宣言には `let`。デフォルトで不変でmutをつけると可変になる。  
変数の参照には `&` をつける。参照も `&mut` とすることで可変にできる。  
つまり、下記のようにすることで、`Stringとして宣言したguessを可変の状態でread_line()に渡す'事ができる。
```
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
```
tup.0 // 500
tup.1 // 6.4
tup.2 // 1
```

## 配列型
同じ型をひとまとめにする型。Rustの配列は固定長であることに注意。  
```
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```
配列の要素にアクセスする場合は下記。
```
months[1]; // January
```
配列外アクセスを行った場合はpanicして終了する。