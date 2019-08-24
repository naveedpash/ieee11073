mod archival {
    #[macro_export]
    macro_rules! nomenclature {
        //class names
        (MDC_MOC_ARCHIVE_MULTI_PT                ) => {  63_u16};
        (MDC_MOC_ARCHIVE_PT                      ) => {  64_u16};
        (MDC_MOC_ARCHIVE_SESSION                 ) => {  65_u16};
        (MDC_MOC_PHYSICIAN                       ) => {  67_u16};
        (MDC_MOC_SESSION_TEST                    ) => {  69_u16};
        (MDC_MOC_SESSION_NOTES                   ) => {  68_u16};
        //attributes
        (MDC_ATTR_ID_HANDLE                      ) => {2337_u16};
        (MDC_ATTR_SYS_ID                         ) => {2436_u16};
        (MDC_ATTR_LOCATION                       ) => {2509_u16};
        (MDC_ATTR_NAME_STUDY                     ) => {2531_u16};
        (MDC_ATTR_ARCHIVE_VERS                   ) => {2480_u16};
        (MDC_ATTR_NAME_SYS                       ) => {2543_u16};
        (MDC_ATTR_PROC_HIST                      ) => {2517_u16};
        (MDC_ATTR_PROTECTION                     ) => {2519_u16};
        (MDC_ATTR_ID_SESS_ARCHIVE                ) => {2507_u16};
        (MDC_ATTR_NAME_SESS_ARCHIVE              ) => {2513_u16};
        (MDC_ATTR_SESS_ARCHIVE_COMMENTS          ) => {2530_u16};
        (MDC_ATTR_TIME_START                     ) => {2538_u16};
        (MDC_ATTR_TIME_STOP                      ) => {2539_u16};
        (MDC_ATTR_ID_Physician                   ) => {2503_u16};
        (MDC_ATTR_Auth_Level                     ) => {2481_u16};
        (MDC_ATTR_PHYSICIAN_NAME                 ) => {2544_u16};
        (MDC_ATTR_PHYSICIAN_NAME_GIVEN           ) => {2546_u16};
        (MDC_ATTR_PHYSICIAN_NAME_FAMILY          ) => {2545_u16};
        (MDC_ATTR_PHYSICIAN_NAME_MIDDLE          ) => {2547_u16};
        (MDC_ATTR_PHYSICIAN_NAME_TITLE           ) => {2548_u16};
        (MDC_ATTR_ID_SESS_TEST_ARCHIVE           ) => {2506_u16};
        (MDC_ATTR_NAME_SESS_TEST_ARCHIVE         ) => {2512_u16};
        (MDC_ATTR_SESS_TEST_ARCHIVE_COMMENTS     ) => {2529_u16};
        (MDC_ATTR_ID_SESS_NOTES_ARCHIVE          ) => {2505_u16};
        (MDC_ATTR_NAME_SESS_NOTES_ARCHIVE        ) => {2511_u16};
        (MDC_ATTR_SESS_NOTES_ARCHIVE_COMMENTS    ) => {2528_u16};
        (MDC_ATTR_Findings                       ) => {2500_u16};
        (MDC_ATTR_CODE_DIAGNOSTIC                ) => {2492_u16};
        (MDC_ATTR_DESC_Diagnostic                ) => {2494_u16};
        (MDC_ATTR_Code_Procedure                 ) => {2493_u16};
        (MDC_ATTR_DESC_PROCEDURE                 ) => {2495_u16};
        //attribute groups
        (MDC_ATTR_GRP_ARCHIVE                    ) => {2068_u16};
        (MDC_ATTR_GRP_PHYSICIAN                  ) => {2071_u16};
        //no behaviours
        //no notifications
    }
}
