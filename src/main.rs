use anyhow::{bail, Context, Result};
use chrono::Local;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs::OpenOptions;
use std::io::Write;

mod pc_rename;
mod qr_scanner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("\n[エラー] {:?}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // 1. 管理者権限チェック
    if !is_elevated::is_elevated() {
        bail!("管理者権限がありません。管理者として実行してください。");
    }

    let cli = Cli::parse();

    // 2. 新しいPC名の取得（引数 > QR/手入力）
    let new_name = get_new_name(&cli)?;

    if new_name.is_empty() {
        bail!("PC 名が空です。有効な名前を指定してください。");
    }

    // 3. 現在の PC 名を取得
    let old_name =
        pc_rename::get_current_name().context("現在のコンピュータ名の取得に失敗しました")?;

    println!("\n現在の PC 名 : {}", old_name);
    println!("新しい PC 名 : {}", new_name);

    if old_name == new_name {
        bail!("新しい PC 名が現在の PC 名と同じです。変更する必要はありません。");
    }

    // 4. コンピュータ名の変更
    if cli.dry_run {
        println!("\n[dry-run] コンピュータ名の変更をスキップします。");
    } else {
        println!("\nコンピュータ名を変更しています...");
        pc_rename::set_computer_name(&new_name)
            .with_context(|| format!("コンピュータ名を '{}' に変更できませんでした", new_name))?;
        println!("✔ コンピュータ名の変更に成功しました");

        // 5. ログに追記 (rename.log)
        // PC 名の変更自体は完了しているため、ログ保存の失敗は警告として扱う。
        if let Err(err) = log_change(&old_name, &new_name) {
            eprintln!("\n⚠ PC 名の変更には成功しましたが、履歴ログを保存できませんでした: {err:#}");
        }
    }

    // 6. 完了サマリー
    println!("\n==============================");
    println!("  PC 名変更 完了サマリー");
    println!("==============================");
    println!("  変更前 : {}", old_name);
    println!("  変更後 : {}", new_name);
    if !cli.dry_run {
        println!("\n⚠  変更を有効にするには PC を再起動してください。");
    }

    Ok(())
}

fn get_new_name(cli: &Cli) -> Result<String> {
    if let Some(n) = &cli.name {
        return Ok(n.trim().to_string());
    }

    // カメラ起動
    println!("インカメラを起動してQRコードをスキャンします (ESCキーでキャンセル)");
    let scanned = qr_scanner::scan_qr_code();

    let mut scanned_name = String::new();
    match scanned {
        Ok(Some(name)) => {
            scanned_name = name;
        }
        Ok(None) => {
            println!("QRスキャンがキャンセルされました。");
        }
        Err(e) => {
            eprintln!("カメラ/スキャンエラー: {:?}", e);
        }
    }

    loop {
        if !scanned_name.is_empty() {
            let prompt_text = format!("読み取ったPC名: [{}] で変更しますか？", scanned_name);
            let selections = &["Yes", "No", "手入力"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&prompt_text)
                .default(0)
                .items(&selections[..])
                .interact()?;

            match selection {
                0 => return Ok(scanned_name), // Yes
                1 => {
                    // No -> 破棄して再ループなど
                    scanned_name.clear();
                }
                2 => {
                    // 手入力
                    return manual_input();
                }
                _ => {}
            }
        } else {
            // スキャン失敗時や手入力を選んだ時など
            println!("手入力に切り替えます。");
            return manual_input();
        }
    }
}

fn manual_input() -> Result<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("新しいPC名を入力してください")
        .interact_text()?;
    Ok(input.trim().to_string())
}

fn log_change(old_name: &str, new_name: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rename.log")?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(file, "[{}] {} -> {}", now, old_name, new_name)?;
    Ok(())
}
