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
        () => {
            module_init($funct(3));
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
        () => {
            module_init($funct(3));
        }
    };
}