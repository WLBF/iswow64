//! Determines whether the current process is running under WOW64.
//!
//! # Example:
//!
//! ```
//! extern crate iswow64;
//!
//! let result = iswow64::iswow64();
//!
//! println!("{:?}", result);
//!
//! #[cfg(target_arch = "x86")]
//! assert_eq!(result.unwrap(), true);
//!
//! #[cfg(target_arch = "x86_64")]
//! assert_eq!(result.unwrap(), false);
//!
//! ```

#[macro_use]
extern crate cpp;

cpp!{{
    #include <windows.h>
    #include <tchar.h>
}}

pub fn iswow64() -> Result<bool, &'static str>{
    unsafe {
        let result: i32 = cpp!([]  -> i32 as "int32_t" {
            typedef BOOL (WINAPI *LPFN_ISWOW64PROCESS) (HANDLE, PBOOL);

            LPFN_ISWOW64PROCESS fnIsWow64Process;

            BOOL bIsWow64 = FALSE;

            //IsWow64Process is not available on all supported versions of Windows.
            //Use GetModuleHandle to get a handle to the DLL that contains the function
            //and GetProcAddress to get a pointer to the function if available.

            fnIsWow64Process = (LPFN_ISWOW64PROCESS) GetProcAddress(
                GetModuleHandle(TEXT("kernel32")),"IsWow64Process");

            if(NULL == fnIsWow64Process)
                return -1;
            if (!fnIsWow64Process(GetCurrentProcess(),&bIsWow64))
                return -2;
            return bIsWow64;
        });

        match result {
            0 => Ok(false),
            1 => Ok(true),
            -1 => Err("get IsWow64Process function failed!"),
            -2 => Err("calling IsWow64Process function failed!"),
            _ => unreachable!(),
        }
    }
}
