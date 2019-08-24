mod system {
    #[macro_export]
    macro_rules! nomenclature {
        //class names
        (MDC_MOC_VMS                         ) => {  32u16};
        (MDC_MOC_VMS_MDS                     ) => {  33u16};
        (MDC_MOC_VMS_MDS_SIMP                ) => {  37u16};
        (MDC_MOC_VMS_MDS_HYD                 ) => {  36u16};
        (MDC_MOC_VMS_MDS_COMPOS_SINGLE_BED   ) => {  35u16};
        (MDC_MOC_VMS_MDS_COMPOS_MULTI_BED    ) => {  34u16};
        (MDC_MOC_LOG                         ) => {  38u16};
        (MDC_MOC_LOG_EVENT                   ) => {  72u16};
        (MDC_MOC_BATT                        ) => {  41u16};
        (MDC_MOC_CLOCK                       ) => {  78u16};
        //attributes
        (MDC_ATTR_SYS_TYPE                   ) => {2438u16};
        (MDC_ATTR_ID_MODEL                   ) => {2344u16};
        (MDC_ATTR_SYS_ID                     ) => {2436u16};
        (MDC_ATTR_ID_COMPAT                  ) => {2336u16};
        (MDC_ATTR_NOM_VERS                   ) => {2376u16};
        (MDC_ATTR_SYS_CAPAB                  ) => {2435u16};
        (MDC_ATTR_SYS_SPECN                  ) => {2437u16};
        (MDC_ATTR_ID_PROD_SPECN              ) => {2349u16};
        (MDC_ATTR_EXT_OBJ_RELATION           ) => {2499u16};
        (MDC_ATTR_VMS_MDS_STAT               ) => {2471u16};
        (MDC_ATTR_ID_BED_LABEL               ) => {2334u16};
        (MDC_ATTR_ID_SOFT                    ) => {2350u16};
        (MDC_ATTR_MODE_OP                    ) => {2374u16};
        (MDC_ATTR_AREA_APPL                  ) => {2317u16};
        (MDC_ATTR_PT_TYPE                    ) => {2402u16};
        (MDC_ATTR_TIME_ABS                   ) => {2439u16};
        (MDC_ATTR_TIME_REL                   ) => {2447u16};
        (MDC_ATTR_TIME_REL_HI_RES            ) => {2536u16};
        (MDC_ATTR_TIME_SUPPORT               ) => {2607u16};
        (MDC_ATTR_DATE_TIME_STATUS           ) => {2608u16};
        (MDC_ATTR_TIME_ABS_ISO               ) => {2609u16};
        (MDC_ATTR_TIME_STAMP_LIST_EXT        ) => {2610u16};
        (MDC_ATTR_TIME_ABS_REL_SYNC          ) => {2611u16};
        (MDC_ATTR_TIME_ZONE                  ) => {2612u16};
        (MDC_ATTR_TIME_DAYLIGHT_SAVINGS_TRANS) => {2613u16};
        (MDC_ATTR_CUM_LEAP_SECONDS           ) => {2614u16};
        (MDC_ATTR_NEXT_LEAP_SECOND           ) => {2615u16};
        (MDC_ATTR_POWER_STAT                 ) => {2389u16};
        (MDC_ATTR_ALTITUDE                   ) => {2316u16};
        (MDC_ATTR_VAL_BATT_CHARGE            ) => {2460u16};
        (MDC_ATTR_TIME_BATT_REMAIN           ) => {2440u16};
        (MDC_ATTR_LINE_FREQ                  ) => {2357u16};
        (MDC_ATTR_ID_ASSOC_NO                ) => {2333u16};
        (MDC_ATTR_LOG_ENTRIES_CURR           ) => {2360u16};
        (MDC_ATTR_LOG_ENTRIES_MAX            ) => {2361u16};
        (MDC_ATTR_EVENT_LOG_ENTRY_LIST       ) => {2564u16};
        (MDC_ATTR_EVENT_LOG_INFO             ) => {2591u16};
        (MDC_ATTR_LOG_CHANGE_COUNT           ) => {2592u16};
        (MDC_ATTR_BATT_STAT                  ) => {2484u16};
        (MDC_ATTR_CAPAC_BATT_REMAIN          ) => {2488u16};
        (MDC_ATTR_CAPAC_BATT_FULL            ) => {2487u16};
        (MDC_ATTR_CAPAC_BATT_SPECN           ) => {2489u16};
        (MDC_ATTR_BATT_VOLTAGE               ) => {2485u16};
        (MDC_ATTR_BATT_VOLTAGE_SPECN         ) => {2486u16};
        (MDC_ATTR_BATT_CURR                  ) => {2483u16};
        (MDC_ATTR_TEMP_BATT                  ) => {2534u16};
        (MDC_ATTR_BATT_CHARGE_CYCLES         ) => {2482u16};
        //attribute groups
        (MDC_ATTR_GRP_SYS_ID                 ) => {2059u16};
        (MDC_ATTR_GRP_SYS_APPL               ) => {2058u16};
        (MDC_ATTR_GRP_SYS_PROD               ) => {2060u16};
        (MDC_ATTR_GRP_RELATION               ) => {2072u16};
        (MDC_ATTR_GRP_CLOCK                  ) => {2078u16};
        (MDC_ATTR_GRP_BATT                   ) => {2069u16};
        //behaviours
        (MDC_ACT_SET_MDS_STATE               ) => {3087u16};
        (MDC_ACT_CLR_LOG                     ) => {3075u16};
        (MDC_ACT_GET_EVENT_LOG_ENTRIES       ) => {3092u16};
        (MDC_ACT_SET_TIME                    ) => {3095u16};
        (MDC_ACT_SET_TIME_ZONE               ) => {3096u16};
        (MDC_ACT_SET_LEAP_SECONDS            ) => {3097u16};
        (MDC_ACT_SET_TIME_ISO                ) => {3098u16};
        //notifications
        (MDC_NOTI_SYS_ERR                    ) => {3349u16};
        (MDC_NOTI_MDS_CREAT                  ) => {3334u16};
        (MDC_NOTI_MDS_ATTR_UPDT              ) => {3333u16};
        (MDC_NOTE_DATE_TIME_CHANGED          ) => {3355u16};




    }
}
