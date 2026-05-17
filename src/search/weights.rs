macro_rules! params {
    ($($name:ident | $tunable:ident : $ty:ty => $default:literal | $min:literal..=$max:literal;)*) => {
        pub struct W;

        $(
            #[cfg(feature = "tune")]
            pub static $tunable: SyncUnsafeCell<$ty> = SyncUnsafeCell::new($default);
        )*

        impl W {
            $(
                #[cfg(not(feature = "tune"))]
                pub const fn $name() -> $ty { $default }

                #[cfg(feature = "tune")]
                pub const fn $name() -> $ty {
                    unsafe { *$tunable.get() }
                }
            )*

            #[cfg(feature = "tune")]
            pub fn set_weight(name: &str, value: String) {
                match name {
                    $(
                        stringify!($tunable) => {
                            let value = match value.parse::<$ty>() {
                                Ok(value) => value,
                                Err(e) => {
                                    println!("info string {:?}", UciParseError::InvalidInteger(e));
                                    return;
                                }
                            };

                            if value > $max || value < $min {
                                println!("info string Invalid {} value: `{value}`", stringify!($tunable));
                                return;
                            }

                            unsafe {
                                *$tunable.get() = value;
                            }
                        },
                    )*
                    _ => println!("info string Unknown Option: `{name}`"),
                }
            }

            #[cfg(feature = "tune")]
            pub fn is_weight(name: &str) -> bool {
                match name {
                    $(stringify!($tunable) => true,)*
                    _ => false
                }
            }

            #[cfg(feature = "tune")]
            pub fn print_spsa() {
                $(
                    println!("{}, int, {:.1}, {:.1}, {:.1}, {:.2}, 0.002", stringify!($tunable), $default as f32, $min as f32, $max as f32, ($max as f32 - $min as f32).abs() / 25.0);
                )*
            }

            #[cfg(feature = "tune")]
            pub fn print_uci() {
                $(
                    println!("option name {} type spin default {} min {} max {}", stringify!($tunable), $default, $min, $max);
                )*
            }
        }
    }
}

params! {
    main_bonus_base   | MAIN_BONUS_BASE:   i64 => 128  | 64..=256;
    main_bonus_scale1 | MAIN_BONUS_SCALE1: i64 => 128  | 64..=256;
    main_bonus_max    | MAIN_BONUS_MAX:    i64 => 2048 | 1024..=4096;
    main_malus_base   | MAIN_MALUS_BASE:   i64 => 128  | 64..=256;
    main_malus_scale1 | MAIN_MALUS_SCALE1: i64 => 128  | 64..=256;
    main_malus_max    | MAIN_MALUS_MAX:    i64 => 2048 | 1024..=4096;
    
    soft_time_div | SOFT_TIME_DIV: u64 => 250648 | 196608..=294912;
    soft_time_inc | SOFT_TIME_INC: u64 => 4027   | 2048..=4096;
    hard_time_div | HARD_TIME_DIV: u64 => 111864 | 8192..=16384;
    hard_time_inc | HARD_TIME_INC: u64 => 3877   | 2048..=4096;
}
