use anyhow::Result;

/// 現在の物理コンピュータ名（DNSホスト名）を返す。
pub fn get_current_name() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        use windows::core::PWSTR;
        use windows::Win32::System::SystemInformation::{GetComputerNameExW, COMPUTER_NAME_FORMAT};

        // ComputerNamePhysicalDnsHostname = 5
        const NAME_TYPE: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(5i32);

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

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JoinStatus {
    Workgroup,
    Domain,
}

/// Windowsの参加状態を取得し、ドメイン参加PCでは変更しないために使用する。
#[cfg(target_os = "windows")]
fn get_join_status() -> Result<JoinStatus> {
    use std::ffi::c_void;
    use windows::core::{PCWSTR, PWSTR};
    use windows::Win32::NetworkManagement::NetManagement::{
        NetApiBufferFree, NetGetJoinInformation, NetSetupDomainName, NetSetupUnjoined,
        NetSetupWorkgroupName, NETSETUP_JOIN_STATUS,
    };

    let mut name_buffer = PWSTR::null();
    let mut join_status = NETSETUP_JOIN_STATUS(0);
    let result =
        unsafe { NetGetJoinInformation(PCWSTR::null(), &mut name_buffer, &mut join_status) };

    if !name_buffer.0.is_null() {
        unsafe {
            NetApiBufferFree(Some(name_buffer.0 as *const c_void));
        }
    }

    if result != 0 {
        anyhow::bail!("Windowsのドメイン参加状態を取得できませんでした（エラーコード: {result}）");
    }

    if join_status == NetSetupDomainName {
        Ok(JoinStatus::Domain)
    } else if join_status == NetSetupUnjoined || join_status == NetSetupWorkgroupName {
        Ok(JoinStatus::Workgroup)
    } else {
        anyhow::bail!(
            "Windowsのドメイン参加状態が不明です（状態コード: {}）",
            join_status.0
        );
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
        if get_join_status()? == JoinStatus::Domain {
            anyhow::bail!(
                "このPCはActive Directoryドメインに参加しているため、このツールではPC名を変更できません。"
            );
        }

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
