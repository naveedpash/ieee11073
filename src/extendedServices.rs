mod extended_services {
    #[macro_export]
    macro_rules! nomenclature {
        //class names
        (MDC_MOC_SCAN                        ) => {  16_u16};
        (MDC_MOC_SCAN_CFG                    ) => {  17_u16};
        (MDC_MOC_SCAN_CFG_EPI                ) => {  18_u16};
        (MDC_MOC_SCAN_CFG_PERI               ) => {  19_u16};
        (MDC_MOC_SCAN_CFG_PERI_FAST          ) => {  20_u16};
        (MDC_MOC_SCAN_UCFG                   ) => {  21_u16};
        (MDC_MOC_SCAN_UCFG_CTXT              ) => {  23_u16};
        (MDC_MOC_SCAN_UCFG_ALSTAT            ) => {  22_u16};
        (MDC_MOC_SCAN_UCFG_OP                ) => {  24_u16};
        (MDC_MOC_DISCRIM                     ) => {  66_u16};
        //attributes
        (MDC_ATTR_ID_HANDLE                  ) => {2337_u16};
        (MDC_ATTR_ID_INSTNO                  ) => {2338_u16};
        (MDC_ATTR_OP_STAT                    ) => {2387_u16};
        (MDC_ATTR_SCAN_LIST                  ) => {2420_u16};
        (MDC_ATTR_CONFIRM_MODE               ) => {2323_u16};
        (MDC_ATTR_CONFIRM_TIMEOUT            ) => {2324_u16};
        (MDC_ATTR_TX_WIND                    ) => {2453_u16};
        (MDC_ATTR_SCAN_CFG_LIMIT             ) => {2558_u16};
        (MDC_ATTR_SCAN_EXTEND                ) => {2419_u16};
        (MDC_ATTR_SCAN_REP_PD                ) => {2421_u16};
        (MDC_ATTR_SCAN_CTXT_MODE             ) => {2418_u16};
        (MDC_ATTR_DISCRIM_CONSTRUCT          ) => {2497_u16};
        //attribute groups
        (MDC_ATTR_GRP_SCAN                   ) => {2056_u16};
        (MDC_ATTR_GRP_DISCRIM                ) => {2070_u16};
        //behaviours
        (MDC_ACT_REFR_EPI_DATA               ) => {3080_u16};
        (MDC_ACT_REFR_CTXT                   ) => {3079_u16};
        (MDC_ACT_REFR_OP_CTXT                ) => {3082_u16};
        (MDC_ACT_REFR_OP_ATTR                ) => {3081_u16};
        //notifications
        (MDC_NOTI_UNBUF_SCAN_RPT             ) => {3350_u16};
        (MDC_NOTI_BUF_SCAN_RPT               ) => {3331_u16};
        (MDC_NOTI_FAST_BUF_SCAN_RPT          ) => {3332_u16};
        (MDC_NOTI_OBJ_CREAT                  ) => {3336_u16};
        (MDC_NOTI_OBJ_DEL                    ) => {3338_u16};
        (MDC_NOTI_AL_STAT_SCAN_RPT           ) => {3329_u16};
        (MDC_NOTI_OP_CREAT                   ) => {3340_u16};
        (MDC_NOTI_OP_DEL                     ) => {3341_u16};
        (MDC_NOTI_OP_ATTR_UPDT               ) => {3339_u16};
    }
}
