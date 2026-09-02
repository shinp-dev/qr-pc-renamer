use anyhow::{bail, Context, Result};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use qrcode::{EcLevel, QrCode};
use rusttype::{Font, Scale};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_FILE: &str = "input.txt";
const OUTPUT_DIR: &str = "output";
const SAMPLE_INPUT: &str = "PC-ROOM-001\nPC-ROOM-002\nPC-ROOM-003\n";

fn main() {
    if let Err(err) = run() {
        eprintln!("\n❌ QRコードの生成に失敗しました: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("=== PC名QRコード一括作成ツール ===");

    let file = match fs::File::open(INPUT_FILE) {
        Ok(file) => file,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            fs::write(INPUT_FILE, SAMPLE_INPUT)
                .with_context(|| format!("{INPUT_FILE} を作成できませんでした"))?;
            println!(
                "🚨 {INPUT_FILE} を新規作成しました。PC名を1行ずつ入力して再度実行してください。"
            );
            return Ok(());
        }
        Err(err) => return Err(err).with_context(|| format!("{INPUT_FILE} を開けませんでした")),
    };

    let texts = read_pc_names(BufReader::new(file))?;
    if texts.is_empty() {
        bail!("{INPUT_FILE} にPC名が入力されていません");
    }

    // 既存出力を消す前に、生成に必要なフォントを確認する。
    let font = load_font()?;
    prepare_output_dir(Path::new(OUTPUT_DIR))?;

    println!("生成を開始します...（合計 {} 件）\n", texts.len());

    let scale = Scale { x: 40.0, y: 40.0 };
    let mut failed_count = 0usize;

    for (i, text) in texts.iter().enumerate() {
        let code = match QrCode::with_error_correction_level(text.as_bytes(), EcLevel::H) {
            Ok(code) => code,
            Err(err) => {
                eprintln!(
                    "❌ [{}/{}] QRコード生成失敗: {} ({err})",
                    i + 1,
                    texts.len(),
                    text
                );
                failed_count += 1;
                continue;
            }
        };

        let mut img: RgbImage = code
            .render::<Rgb<u8>>()
            .min_dimensions(800, 800)
            .dark_color(Rgb([20, 30, 40]))
            .light_color(Rgb([255, 255, 255]))
            .build();

        let (img_width, img_height) = img.dimensions();
        let display_text = text.clone();

        let v_metrics = font.v_metrics(scale);
        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let width = font
            .layout(&display_text, scale, rusttype::point(0.0, v_metrics.ascent))
            .last()
            .map(|glyph| glyph.position().x + glyph.unpositioned().h_metrics().advance_width)
            .unwrap_or(0.0)
            .ceil() as u32;

        let padding_x: u32 = 24;
        let padding_y: u32 = 10;
        let bg_w = width + padding_x * 2;
        let bg_h = height + padding_y * 2;
        let bg_x = (img_width.saturating_sub(bg_w)) / 2;
        let bg_y = (img_height.saturating_sub(bg_h)) / 2;

        draw_filled_rect_mut(
            &mut img,
            Rect::at(bg_x as i32, bg_y as i32).of_size(bg_w, bg_h),
            Rgb([255, 255, 255]),
        );

        draw_text_mut(
            &mut img,
            Rgb([20, 30, 40]),
            bg_x + padding_x,
            bg_y + padding_y / 2,
            scale,
            &font,
            &display_text,
        );

        let output_path = format!("{OUTPUT_DIR}/qr_{:03}.png", i + 1);
        match img.save(&output_path) {
            Ok(()) => println!(
                "✅ [{}/{}] 保存完了: {} (PC名: {})",
                i + 1,
                texts.len(),
                output_path,
                display_text
            ),
            Err(err) => {
                eprintln!(
                    "❌ [{}/{}] 保存失敗: {} ({err})",
                    i + 1,
                    texts.len(),
                    output_path
                );
                failed_count += 1;
            }
        }
    }

    if failed_count > 0 {
        bail!("{failed_count} 件のQRコードを生成できませんでした");
    }

    println!("\n🎉 すべて完了しました！ {OUTPUT_DIR} フォルダをご確認ください！");
    Ok(())
}

fn read_pc_names<R: BufRead>(reader: R) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut validation_errors = Vec::new();
    let mut first_occurrences = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line =
            line.with_context(|| format!("{INPUT_FILE} の {line_number} 行目を読めませんでした"))?;
        let name = line.trim();
        if name.is_empty() {
            continue;
        }

        match pc_name::validate(name) {
            Ok(()) => {
                let normalized_name = name.to_ascii_uppercase();
                if let Some(first_line_number) = first_occurrences.get(&normalized_name) {
                    validation_errors.push(format!(
                        "{line_number}行目 [{name}] は{first_line_number}行目と重複しています"
                    ));
                } else {
                    first_occurrences.insert(normalized_name, line_number);
                    names.push(name.to_string());
                }
            }
            Err(err) => validation_errors.push(format!("{line_number} 行目 [{name}]: {err}")),
        }
    }

    if !validation_errors.is_empty() {
        bail!(
            "{INPUT_FILE} に無効なPC名があります:\n{}",
            validation_errors.join("\n")
        );
    }

    Ok(names)
}

fn load_font() -> Result<Font<'static>> {
    let windir = std::env::var("windir").unwrap_or_else(|_| "C:\\Windows".to_string());
    let meiryo_path = format!("{windir}\\Fonts\\meiryo.ttc");
    let gothic_path = format!("{windir}\\Fonts\\msgothic.ttc");

    let font_data = fs::read(&meiryo_path)
        .or_else(|_| fs::read(&gothic_path))
        .with_context(|| {
            format!("システムフォントが見つかりません: {meiryo_path}, {gothic_path}")
        })?;

    Font::try_from_vec(font_data).context("システムフォントを読み込めませんでした")
}

fn prepare_output_dir(output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("{} フォルダを作成できませんでした", output_dir.display()))?;

    for entry in fs::read_dir(output_dir)
        .with_context(|| format!("{} フォルダを確認できませんでした", output_dir.display()))?
    {
        let entry = entry.with_context(|| {
            format!(
                "{} 内のファイルを確認できませんでした",
                output_dir.display()
            )
        })?;
        if entry
            .file_type()
            .with_context(|| format!("{} の種類を確認できませんでした", entry.path().display()))?
            .is_file()
            && is_generated_qr_file(&entry.file_name())
        {
            fs::remove_file(entry.path()).with_context(|| {
                format!(
                    "古いQR画像 {} を削除できませんでした",
                    entry.path().display()
                )
            })?;
        }
    }

    Ok(())
}

fn is_generated_qr_file(name: &OsStr) -> bool {
    let Some(name) = name.to_str() else {
        return false;
    };
    let Some(number) = name
        .strip_prefix("qr_")
        .and_then(|name| name.strip_suffix(".png"))
    else {
        return false;
    };
    !number.is_empty() && number.chars().all(|character| character.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::{is_generated_qr_file, prepare_output_dir, read_pc_names};
    use std::ffi::OsStr;
    use std::fs;
    use std::io::Cursor;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn reports_invalid_input_line() {
        let err = read_pc_names(Cursor::new("PC-001\n12345\nPC-003\n"))
            .expect_err("numeric-only PC name should fail");
        assert!(err.to_string().contains("2 行目 [12345]"));
    }

    #[test]
    fn rejects_exact_duplicate_names() {
        let err = read_pc_names(Cursor::new("PC-001\nROOM-002\nPC-001\n"))
            .expect_err("exact duplicate PC name should fail");
        assert!(err
            .to_string()
            .contains("3行目 [PC-001] は1行目と重複しています"));
    }

    #[test]
    fn rejects_case_insensitive_duplicate_names() {
        let err = read_pc_names(Cursor::new("PC-001\nroom-002\npc-001\n"))
            .expect_err("case-insensitive duplicate PC name should fail");
        assert!(err
            .to_string()
            .contains("3行目 [pc-001] は1行目と重複しています"));
    }

    #[test]
    fn accepts_non_duplicate_names() {
        let names = read_pc_names(Cursor::new("PC-001\nroom-002\nPC-003\n"))
            .expect("non-duplicate PC names should be accepted");
        assert_eq!(names, ["PC-001", "room-002", "PC-003"]);
    }

    #[test]
    fn duplicate_input_does_not_touch_existing_output() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be valid")
            .as_nanos();
        let test_dir = std::env::temp_dir().join(format!(
            "qr-batch-generator-duplicate-test-{}-{unique}",
            std::process::id()
        ));

        fs::create_dir_all(&test_dir).expect("test directory should be created");
        let old_qr = test_dir.join("qr_001.png");
        fs::write(&old_qr, b"old").expect("old QR should be written");

        let result = read_pc_names(Cursor::new("PC-001\npc-001\n"));
        assert!(result.is_err());
        assert!(
            old_qr.exists(),
            "validation must finish before output cleanup"
        );

        fs::remove_dir_all(&test_dir).expect("test directory should be removed");
    }

    #[test]
    fn generated_file_pattern_is_strict() {
        assert!(is_generated_qr_file(OsStr::new("qr_001.png")));
        assert!(is_generated_qr_file(OsStr::new("qr_12.png")));
        assert!(!is_generated_qr_file(OsStr::new("qr_backup.png")));
        assert!(!is_generated_qr_file(OsStr::new("keep.png")));
    }

    #[test]
    fn removes_only_previous_generated_images() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be valid")
            .as_nanos();
        let test_dir = std::env::temp_dir().join(format!(
            "qr-batch-generator-test-{}-{unique}",
            std::process::id()
        ));

        fs::create_dir_all(&test_dir).expect("test directory should be created");
        fs::write(test_dir.join("qr_001.png"), b"old").expect("old QR should be written");
        fs::write(test_dir.join("keep.png"), b"keep").expect("keep file should be written");

        prepare_output_dir(&test_dir).expect("output directory should be prepared");

        assert!(!test_dir.join("qr_001.png").exists());
        assert!(test_dir.join("keep.png").exists());
        fs::remove_dir_all(&test_dir).expect("test directory should be removed");
    }
}
