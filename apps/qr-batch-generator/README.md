# PC名QRコード一括作成ツール

`input.txt`に書かれたWindows PC名を読み込み、PC名入りのQRコード画像を一括生成します。

## ビルド

モノレポのルートで次を実行します。

```powershell
cargo build --workspace --release
```

実行ファイルは`target\release\qr-batch-generator.exe`に作成されます。配布時は、このEXEを作業用フォルダへコピーしてください。

## 使い方

1. EXEと同じフォルダに`input.txt`を置きます。
2. PC名を1行ずつ入力します。
3. `qr-batch-generator.exe`を実行します。
4. `output`フォルダ内の`qr_001.png`、`qr_002.png`などを確認します。

`input.txt`がない場合は、初回実行時にサンプルが作成されます。

```text
PC-ROOM-001
PC-ROOM-002
PC-ROOM-003
```

再実行時は、前回このツールが作成した`qr_数字.png`だけを削除してから生成します。`output`内の別名ファイルは削除しません。

## PC名のルール

- 15文字以内の半角英数字とハイフンを使用してください。
- 英字を1文字以上含めてください。
- ハイフンは先頭・末尾には使用できません。

このルールは、PC名変更ツールと共有crateの`pc-name`で統一されています。
