use anyhow::{bail, Result};

/// 現在のコンピュータ名（NetBIOS ホスト名）を返す
pub fn get_current_name() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        use windows::core::PWSTR;
        use windows::Win32::System::SystemInformation::{GetComputerNameExW, COMPUTER_NAME_FORMAT};

        // ComputerNameDnsHostname = 1
        const NAME_TYPE: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(1i32);

        // まずバッファサイズを問い合わせる（size=0 で呼ぶと必要サイズが返る）
        let mut size: u32 = 0;
        unsafe {
            let _ = GetComputerNameExW(NAME_TYPE, PWSTR::null(), &mut size);
        }

        let mut buf = vec![0u16; size as usize];
        unsafe {
            GetComputerNameExW(NAME_TYPE, PWSTR(buf.as_mut_ptr()), &mut size)?;
        }
        Ok(String::from_utf16_lossy(&buf[..size as usize]))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok("NON-WINDOWS-HOST".to_string())
    }
}

/// Windows のコンピュータ名を変更する（次回再起動後に有効）
///
/// 管理者権限が必要です。
/// `ComputerNamePhysicalDnsHostname = 5` を使用し、DNS + NetBIOS 両方に反映します。
pub fn set_computer_name(new_name: &str) -> Result<()> {
    validate_name(new_name)?;

    #[cfg(target_os = "windows")]
    {
        use windows::core::PCWSTR;
        use windows::Win32::System::SystemInformation::{SetComputerNameExW, COMPUTER_NAME_FORMAT};

        // ComputerNamePhysicalDnsHostname = 5
        const NAME_TYPE: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(5i32);

        let wide: Vec<u16> = new_name.encode_utf16().chain(std::iter::once(0)).collect();
        unsafe {
            SetComputerNameExW(NAME_TYPE, PCWSTR(wide.as_ptr()))?;
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        eprintln!(
            "[dry-run / 非 Windows] PC 名を '{}' に変更しようとしました（実際には未変更）",
            new_name
        );
        Ok(())
    }
}

/// NetBIOS 名として許可される文字か検証する（最大 15 文字、英数字とハイフン）
fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("PC 名が空です");
    }
    if name.len() > 15 {
        bail!(
            "PC 名は 15 文字以内にしてください（現在: {} 文字）",
            name.len()
        );
    }
    for ch in name.chars() {
        if !ch.is_ascii_alphanumeric() && ch != '-' {
            bail!(
                "PC 名に使用できない文字が含まれています: '{}'\n（英数字とハイフンのみ使用可）",
                ch
            );
        }
    }
    if !name.chars().any(|ch| ch.is_ascii_alphabetic()) {
        bail!("PC 名には英字を 1 文字以上含めてください");
    }
    if name.starts_with('-') || name.ends_with('-') {
        bail!("PC 名の先頭・末尾にハイフンは使用できません");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_name;

    #[test]
    fn accepts_valid_pc_names() {
        for name in ["PC-001", "ROOM-A12", "A", "PC1234567890123"] {
            assert!(validate_name(name).is_ok(), "{name} should be valid");
        }
    }

    #[test]
    fn rejects_invalid_pc_names() {
        for name in [
            "",
            "12345",
            "-PC001",
            "PC001-",
            "PC_001",
            "PC NAME",
            "PC12345678901234",
        ] {
            assert!(validate_name(name).is_err(), "{name} should be invalid");
        }
    }
}
