use anyhow::Result;

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
    pc_name::validate(new_name)?;

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
