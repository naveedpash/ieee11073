mod units {
    #[macro_export]
    macro_rules! decimal_factors_offsets {
        (yotta     ) => {10_u16};
        (zetta     ) => { 9_u16};
        (exa       ) => { 8_u16};
        (peta      ) => { 7_u16};
        (tera      ) => { 6_u16};
        (giga      ) => { 5_u16};
        (mega      ) => { 4_u16};
        (kilo      ) => { 3_u16};
        (hecto     ) => { 2_u16};
        (deca      ) => { 1_u16};
        (deci      ) => {16_u16};
        (centi     ) => {17_u16};
        (milli     ) => {18_u16};
        (micro     ) => {19_u16};
        (nano      ) => {20_u16};
        (pico      ) => {21_u16};
        (femto     ) => {22_u16};
        (atto      ) => {23_u16};
        (zepto     ) => {24_u16};
        (yocto     ) => {25_u16};
    }
}
