#[macro_export]
macro_rules! declare_entry {
    ($funct:ident) => {
        #[cfg(target_os = "windows")]
        #[no_mangle]
        pub extern "C" fn DriverEntry(DriverObject: usize, RegistryPath: usize)
        {
            $funct(DriverObject);
        }

        #[cfg(target_os = "linux")]
        loop {}
    };
}

#[macro_export]
macro_rules! declare_panic {
    () => {
        #[panic_handler]
        fn panic(panicInfo: &oxidation::PanicInfo) -> ! {
            loop {}
        }
    };
}

#[macro_export]
macro_rules! declare_leave {
    ($funct:ident) => {
        #[cfg(target_os = "windows")]
        #[no_mangle]
        pub extern "C" fn DriverEntry(DriverObject: usize, RegistryPath: usize)
        {
            $funct(DriverObject);
        }

        #[cfg(target_os = "linux")]
        fn panic(panicInfo: &oxidation::PanicInfo) -> ! {
            loop {}
        }
    };
}