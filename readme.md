## MovingAverage3D

3次元ベクトルの移動平均を計算する Rust 実装です。

`[x, y, z]` の順で受け取った `f32` 配列を、指定したウィンドウサイズで平滑化します。

## サンプルコード

```rust
mod moving_average3d;
use moving_average3d::MovingAverage3D;

fn main() {
    let mut moving_average = MovingAverage3D::new(10);

    loop {
        // センサーや外部入力から 3 次元データを取得する想定です。
        // 例:
        // let x = ...;
        // let y = ...;
        // let z = ...;

        let data = [1.0, 2.0, 3.0];

        moving_average.update(data);

        // 戻り値は [x, y, z] の順です。
        let average = moving_average.getdata();
        println!("x: {}, y: {}, z: {}", average[0], average[1], average[2]);
    }
}
```

## 仕様

デフォルトの動作はなく、`MovingAverage3D::new(size)` でウィンドウサイズを指定して使います。

```rust
use moving_average3d::MovingAverage3D;

let mut moving_average = MovingAverage3D::new(5);
```

### `update`

```rust
moving_average.update([1.0, 2.0, 3.0]);
```

- 入力は `[x, y, z]` の順です。
- それぞれ `f32` を使います。
- ウィンドウサイズに達するまでは、入っているデータ数で割った平均を返します。
- ウィンドウサイズを超えた分は、もっとも古い値から順に捨てられます。

### `getdata`

```rust
let average = moving_average.getdata();
```

- 戻り値は `[x, y, z]` の順です。
- 現在の移動平均結果をそのまま取得できます。

## 注意点

`MovingAverage3D::new(0)` は使用できません。ウィンドウサイズは 1 以上を指定してください。

## テスト

```bash
cargo test
```

テストでは、先頭の入力に対する平均計算と、ウィンドウサイズを超えた後に古い値が捨てられることを確認しています。