//class names
pub const MDC_MOC_SCAN                       :u16  =   16;
pub const MDC_MOC_SCAN_CFG                   :u16  =   17;
pub const MDC_MOC_SCAN_CFG_EPI               :u16  =   18;
pub const MDC_MOC_SCAN_CFG_PERI              :u16  =   19;
pub const MDC_MOC_SCAN_CFG_PERI_FAST         :u16  =   20;
pub const MDC_MOC_SCAN_UCFG                  :u16  =   21;
pub const MDC_MOC_SCAN_UCFG_CTXT             :u16  =   23;
pub const MDC_MOC_SCAN_UCFG_ALSTAT           :u16  =   22;
pub const MDC_MOC_SCAN_UCFG_OP               :u16  =   24;
pub const MDC_MOC_DISCRIM                    :u16  =   66;
//attributes
pub const MDC_ATTR_ID_HANDLE                 :u16  = 2337;
pub const MDC_ATTR_ID_INSTNO                 :u16  = 2338;
pub const MDC_ATTR_OP_STAT                   :u16  = 2387;
pub const MDC_ATTR_SCAN_LIST                 :u16  = 2420;
pub const MDC_ATTR_CONFIRM_MODE              :u16  = 2323;
pub const MDC_ATTR_CONFIRM_TIMEOUT           :u16  = 2324;
pub const MDC_ATTR_TX_WIND                   :u16  = 2453;
pub const MDC_ATTR_SCAN_CFG_LIMIT            :u16  = 2558;
pub const MDC_ATTR_SCAN_EXTEND               :u16  = 2419;
pub const MDC_ATTR_SCAN_REP_PD               :u16  = 2421;
pub const MDC_ATTR_SCAN_CTXT_MODE            :u16  = 2418;
pub const MDC_ATTR_DISCRIM_CONSTRUCT         :u16  = 2497;
//attribute groups
pub const MDC_ATTR_GRP_SCAN                  :u16  = 2056;
pub const MDC_ATTR_GRP_DISCRIM               :u16  = 2070;
//behaviours
pub const MDC_ACT_REFR_EPI_DATA              :u16  = 3080;
pub const MDC_ACT_REFR_CTXT                  :u16  = 3079;
pub const MDC_ACT_REFR_OP_CTXT               :u16  = 3082;
pub const MDC_ACT_REFR_OP_ATTR               :u16  = 3081;
//notifications
pub const MDC_NOTI_UNBUF_SCAN_RPT            :u16  = 3350;
pub const MDC_NOTI_BUF_SCAN_RPT              :u16  = 3331;
pub const MDC_NOTI_FAST_BUF_SCAN_RPT         :u16  = 3332;
pub const MDC_NOTI_OBJ_CREAT                 :u16  = 3336;
pub const MDC_NOTI_OBJ_DEL                   :u16  = 3338;
pub const MDC_NOTI_AL_STAT_SCAN_RPT          :u16  = 3329;
pub const MDC_NOTI_OP_CREAT                  :u16  = 3340;
pub const MDC_NOTI_OP_DEL                    :u16  = 3341;
pub const MDC_NOTI_OP_ATTR_UPDT              :u16  = 3339;
