mod patient {
    #[macro_export]
    macro_rules! nomenclature {
        //class names
        (MDC_MOC_PT_DEMOG                    ) => {  42_u16};
        //attributes
        (MDC_ATTR_ID_HANDLE                  ) => {2337_u16};
        (MDC_ATTR_PT_DEMOG_ST                ) => {2391_u16};
        (MDC_ATTR_PT_ID                      ) => {2394_u16};
        (MDC_ATTR_PT_NAME                    ) => {2395_u16};
        (MDC_ATTR_PT_NAME_GIVEN              ) => {2397_u16};
        (MDC_ATTR_PT_NAME_FAMILY             ) => {2396_u16};
        (MDC_ATTR_PT_NAME_MIDDLE             ) => {2399_u16};
        (MDC_ATTR_PT_NAME_BIRTH              ) => {2398_u16};
        (MDC_ATTR_PT_NAME_TITLE              ) => {2400_u16};
        (MDC_ATTR_PT_SEX                     ) => {2401_u16};
        (MDC_ATTR_PT_RACE                    ) => {2526_u16};
        (MDC_ATTR_PT_TYPE                    ) => {2402_u16};
        (MDC_ATTR_PT_DOB                     ) => {2392_u16};
        (MDC_ATTR_PT_GEN_INFO                ) => {2393_u16};
        (MDC_ATTR_Pt_Age                     ) => {2520_u16};
        (MDC_ATTR_Pt_Age_Gest                ) => {2521_u16};
        (MDC_ATTR_Pt_Height                  ) => {2524_u16};
        (MDC_ATTR_Pt_Weight                  ) => {2527_u16};
        (MDC_ATTR_Pt_Birth_Length            ) => {2522_u16};
        (MDC_ATTR_Pt_Birth_Weight            ) => {2523_u16};
        (MDC_ATTR_ID_PT_Mother               ) => {2504_u16};
        (MDC_ATTR_Pt_Name_Mother             ) => {2525_u16};
        (MDC_ATTR_CIRCUM_HEAD                ) => {2490_u16};
        (MDC_ATTR_PT_BSA                     ) => {2390_u16};
        (MDC_ATTR_ID_BED                     ) => {2501_u16};
        (MDC_ATTR_Diagnostic_Info            ) => {2496_u16};
        (MDC_ATTR_CODE_DIAGNOSTIC            ) => {2492_u16};
        (MDC_ATTR_Physician_Admit            ) => {2515_u16};
        (MDC_ATTR_Physician_ATTEND           ) => {2516_u16};
        (MDC_ATTR_Procedure_DATE             ) => {2518_u16};
        (MDC_ATTR_Code_Procedure             ) => {2493_u16};
        (MDC_ATTR_DESC_Procedure             ) => {2495_u16};
        (MDC_ATTR_Anaesthetist               ) => {2479_u16};
        (MDC_ATTR_Surgeon                    ) => {2532_u16};
        (MDC_ATTR_PT_LBM                     ) => {2601_u16};
        //attribute groups
        (MDC_ATTR_GRP_PT_DEMOG               ) => {2055_u16};
        //behaviours
        (MDC_ACT_DISCH_PT                    ) => {3076_u16};
        (MDC_ACT_ADMIT_PT                    ) => {3074_u16};
        (MDC_ACT_PRE_ADMIT_PT                ) => {3078_u16};
        //notifications
        (MDC_NOTI_PT_DEMOG_MOD               ) => {3342_u16};
        (MDC_NOTI_PT_DEMOG_ST_MOD            ) => {3343_u16};
    }
}
