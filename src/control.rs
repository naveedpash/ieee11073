mod control {
    #[macro_export]
    macro_rules! nomenclature {
        //class names
        (MDC_MOC_CNTRL_SCO                  ) => {  43_u16};
        (MDC_MOC_CNTRL_OP                   ) => {  44_u16};
        (MDC_MOC_CNTRL_OP_SEL_IT            ) => {  45_u16};
        (MDC_MOC_CNTRL_OP_SEL_VAL           ) => {  47_u16};
        (MDC_MOC_CNTRL_OP_SET_STRING        ) => {  73_u16};
        (MDC_MOC_CNTRL_OP_TOG               ) => {  49_u16};
        (MDC_MOC_CNTRL_OP_ACTIV             ) => {  50_u16};
        (MDC_MOC_CNTRL_OP_LIM               ) => {  51_u16};
        (MDC_MOC_CNTRL_OP_SET_RANGE         ) => {  80_u16};
        //attribute names
        (MDC_ATTR_SCO_CAPAB                 ) => {2422_u16};
        (MDC_ATTR_SCO_HELP_TEXT_STRING      ) => {2549_u16};
        (MDC_ATTR_VMO_REF                   ) => {2468_u16};
        (MDC_ATTR_INDIC_ACTIV               ) => {2355_u16};
        (MDC_ATTR_STAT_LOCK                 ) => {2432_u16};
        (MDC_ATTR_ID_INVOK_COOKIE           ) => {2339_u16};
        (MDC_ATTR_ID_INSTNO                 ) => {2338_u16};
        (MDC_ATTR_OP_SPEC                   ) => {2386_u16};
        (MDC_ATTR_OP_TEXT_STRING            ) => {2514_u16};
        (MDC_ATTR_OP_TEXT_STRING_DYN        ) => {2602_u16};
        (MDC_ATTR_OP_STAT                   ) => {2387_u16};
        (MDC_ATTR_INDEX_SEL                 ) => {2354_u16};
        (MDC_ATTR_ID_NOM_PARTITION          ) => {2345_u16};
        (MDC_ATTR_LIST_SEL                  ) => {2358_u16};
        (MDC_ATTR_VAL_CURR                  ) => {2461_u16};
        (MDC_ATTR_VAL_RANGE                 ) => {2464_u16};
        (MDC_ATTR_VAL_STEP_WIDTH            ) => {2465_u16};
        (MDC_ATTR_UNIT_CODE                 ) => {2454_u16};
        (MDC_ATTR_STRING_CURR               ) => {2565_u16};
        (MDC_ATTR_SET_STRING_SPEC           ) => {2567_u16};
        (MDC_ATTR_STAT_OP_TOG               ) => {2433_u16};
        (MDC_ATTR_TOG_LABELS_STRING         ) => {2540_u16};
        (MDC_ATTR_AL_OP_CAPAB               ) => {2309_u16};
        (MDC_ATTR_AL_OP_STAT                ) => {2310_u16};
        (MDC_ATTR_LIMIT_CURR                ) => {2356_u16};
        (MDC_ATTR_AL_OP_TEXT                ) => {2311_u16};
        (MDC_ATTR_ID_PHYSIO                 ) => {2347_u16};
        (MDC_ATTR_RANGE_CURR                ) => {2624_u16};
        (MDC_ATTR_RANGE_OP_TEXTSTRING       ) => {2625_u16};
        // attribute groups
        (MDC_ATTR_GRP_VMO_STATIC            ) => {2065_u16};
        (MDC_ATTR_GRP_VMO_DYN               ) => {2064_u16};
        (MDC_ATTR_GRP_OP_STATIC_CTXT        ) => {2053_u16};
        (MDC_ATTR_GRP_OP_DYN_CTXT           ) => {2052_u16};
        (MDC_ATTR_GRP_SCO_TRANSACTION       ) => {2057_u16};
        //behaviours
        (MDC_ACT_SCO_OP_INVOK               ) => {3083_u16};
        (MDC_ACT_GET_CTXT_HELP              ) => {3077_u16};
        //notifications
        (MDC_NOTI_SCO_OP_REQ                ) => {3347_u16};
        (MDC_NOTI_SCO_OP_INVOK_ERR          ) => {3346_u16};
    }
}
