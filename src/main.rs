extern crate winapi;

use std::ffi::CStr;
use winapi::shared::winerror::NO_ERROR;
use winapi::um::iphlpapi::GetAdaptersInfo;
use winapi::um::iptypes::IP_ADAPTER_INFO;

fn main() {
    unsafe {
        let mut adapter_info: [IP_ADAPTER_INFO; 16] = std::mem::zeroed();
        let mut buffer_size: u32 = std::mem::size_of_val(&adapter_info) as u32;

        // Panggil GetAdaptersInfo untuk mendapatkan informasi adapter jaringan
        let ret = GetAdaptersInfo(adapter_info.as_mut_ptr(), &mut buffer_size);
        if ret != NO_ERROR {
            eprintln!("GetAdaptersInfo failed with error: {}", ret);
            return;
        }

        // Iterasi melalui adapter dan cetak informasi
        for adapter in adapter_info.iter() {
            if adapter.AdapterName[0] == 0 {
                break;
            }

            let adapter_name = CStr::from_ptr(adapter.Description.as_ptr()).to_string_lossy();
            println!("Adapter: {}", adapter_name);

            let ip_address =
                CStr::from_ptr(adapter.IpAddressList.IpAddress.String.as_ptr()).to_string_lossy();
            let subnet_mask =
                CStr::from_ptr(adapter.IpAddressList.IpMask.String.as_ptr()).to_string_lossy();
            let gateway =
                CStr::from_ptr(adapter.GatewayList.IpAddress.String.as_ptr()).to_string_lossy();

            println!("  IP Address: {}", ip_address);
            println!("  Subnet Mask: {}", subnet_mask);
            println!("  Gateway: {}", gateway);
        }
    }
}
