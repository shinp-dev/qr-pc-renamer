# 実習室PC名変更ツール

実習室・教室向けのWindows PC名変更ツールです。QRコードまたは手入力でPC名を指定できます。

PC名の変更には管理者権限が必要です。

## 最新版をダウンロード

[GitHub Releasesの最新版から `pc-renamer.zip` をダウンロードする](https://github.com/shinp-dev/qr-pc-renamer/releases/latest/download/pc-renamer.zip)

## 使用方法

1. `pc-renamer.zip` をダウンロードします。
2. ZIPを展開します。
3. 必要に応じて、展開したフォルダの `qr-batch-generator.exe` でPC名のQR画像を一括作成します。
4. `run_renamer_qr.bat` を起動します。
5. UAC画面が表示されたら「はい」を選びます。
6. QRコードを読み取るか、PC名を手入力します。
7. 表示された現在のPC名と新しいPC名を確認します。
8. PC名の変更を実行します。
9. 最後にPCを再起動します。

ZIPには次の3ファイルが含まれています。

- `pc_renamer.exe`：PC名を変更するツール
- `run_renamer_qr.bat`：管理者権限でPC名変更ツールを起動するファイル
- `qr-batch-generator.exe`：PC名一覧からQR画像を一括作成するツール

## PC名のルール

- 15文字以内の半角英数字とハイフンを使用してください。
- 英字を1文字以上含めてください。
- ハイフンを先頭・末尾に使用しないでください。
- QRコード一括作成時は、同じPC名を大文字・小文字違いで登録することもできません。

## ログ

変更履歴は次の場所に保存されます。

`%ProgramData%\ShinpStudio\PcRenamer\rename.log`

ログには日時、変更前後のPC名、ツールバージョン、実行ユーザーが記録されます。PC名変更後にログ保存だけが失敗した場合は、画面に警告を表示します。

## 注意事項

- Windows専用です。
- PC名変更には管理者権限が必要です。
- Active Directoryドメイン参加PCでは使用できません。
- PC名変更後は再起動が必要です。

## QRコード一括作成

展開したフォルダで `qr-batch-generator.exe` を実行してください。初回は同じフォルダに `input.txt` が作成されるので、PC名を1行ずつ入力してからもう一度実行します。PCごとのQR画像が `output` フォルダに作成されます。空行は無視されます。

入力に無効なPC名や重複したPC名がある場合は、既存のQR画像を削除せずにエラー終了します。重複判定では大文字・小文字を区別しません。
