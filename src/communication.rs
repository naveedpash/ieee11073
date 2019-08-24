mod communication {
    #[macro_export]
    macro_rules! nomenclature {
        //class names
        (MDC_MOC_CC                ) => {  28_u16};
        (MDC_MOC_DCC               ) => {  76_u16};
        (MDC_MOC_BCC               ) => {  77_u16};
        //attributes
        (MDC_ATTR_ID_HANDLE        ) => {2337_u16};
        (MDC_ATTR_CC_CAPAB         ) => {2593_u16};
        (MDC_ATTR_CC_TYPE          ) => {2594_u16};
        (MDC_ATTR_CC_NUM_DIFS      ) => {2595_u16};
        (MDC_ATTR_CC_THIS_DIF_INDEX) => {2596_u16};
        (MDC_ATTR_CC_EXT_MNG_PROT  ) => {2597_u16};
        //attribute groups
        (MDC_ATTR_GRP_CC           ) => {2077_u16}
        //behaviours
        (MDC_ACT_GET_MIB_DATA      ) => {3093_u16};
        (MDC_ACT_POLL_MDIB_DATA    ) => {3094_u16};
        //no notifications
    }
}
