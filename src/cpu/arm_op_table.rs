use crate::cpu::arm_op_table::ArmOpType::{ADC______, ADCS_____, ADD______, ADDS_____, AND______, ANDS_____, B________, BIC______, BICS_____, BKPT_____, BL_______, BLX______, BX_______, CDP______, CLZ______, CMNS_____, CMPS_____, EOR______, EORS_____, LDC______, LDMDA____, LDMDB____, LDMIA____, LDMIB____, LDR______, LDRB_____, LDRBT____, LDRD_____, LDRH_____, LDRSB____, LDRSH____, LDRT_____, MCR______, MLA______, MLAS_____, MOV______, MOVS_____, MRC______, MRS______, MSR______, MUL______, MULS_____, MVN______, MVNS_____, ORR______, ORRS_____, QADD_____, QDADD____, QDSUB____, QSUB_____, RSB______, RSBS_____, RSC______, RSCS_____, SBC______, SBCS_____, SMLABB___, SMLABT___, SMLAL____, SMLALBB__, SMLALBT__, SMLALS___, SMLALTB__, SMLALTT__, SMLATB___, SMLATT___, SMLAWB___, SMLAWT___, SMULBB___, SMULBT___, SMULL____, SMULLS___, SMULTB___, SMULTT___, SMULWB___, SMULWT___, STC______, STMDA____, STMDB____, STMIA____, STMIB____, STR______, STRB_____, STRBT____, STRD_____, STRH_____, STRT_____, SUB______, SUBS_____, SWI______, SWP______, SWPB_____, TEQS_____, TSTS_____, UMLAL____, UMLALS___, UMULL____, UMULLS___, Undefined};

pub fn select_instruct() {}

#[derive(Copy, Clone)]
pub enum ArmOpType {
    BLX______,
    SMLALBT__,
    STMDA____,
    STC______,
    MCR______,
    QDSUB____,
    UMLAL____,
    SMULLS___,
    RSC______,
    ADD______,
    MRS______,
    SMLATB___,
    SMULTT___,
    STMDB____,

    STMIB____,
    ADC______,
    B________,
    SMLAWT___,

    STR______,
    STRT_____,
    STRB_____,
    STRBT____,
    STRH_____,
    STRD_____,

    LDR______,
    LDRSB____,
    LDRB_____,
    LDRD_____,
    LDRBT____,
    LDRSH____,
    LDRH_____,
    LDRT_____,

    ADCS_____,
    BICS_____,
    LDMIA____,
    MRC______,
    RSBS_____,
    MULS_____,
    AND______,
    MLAS_____,
    MUL______,
    SUBS_____,
    QDADD____,
    SMULL____,
    ANDS_____,
    SBC______,
    SUB______,
    SMLALS___,
    BX_______,
    MOVS_____,
    MLA______,
    EORS_____,
    STMIA____,
    SMLABB___,
    LDMDA____,
    RSB______,
    TSTS_____,
    MVN______,
    QSUB_____,
    QADD_____,
    RSCS_____,
    UMLALS___,
    EOR______,
    SMLAL____,
    SMLATT___,
    SBCS_____,
    SMLAWB___,
    MVNS_____,
    CDP______,
    MSR______,
    SMLALTB__,
    SMULWB___,
    CLZ______,
    LDMDB____,
    SMULWT___,
    ADDS_____,
    ORRS_____,
    SWI______,
    BIC______,
    MOV______,
    CMPS_____,
    BKPT_____,
    SMLABT___,
    UMULLS___,
    LDC______,
    SMULTB___,
    SWP______,
    SMULBB___,
    SWPB_____,
    UMULL____,
    SMLALTT__,
    SMULBT___,
    CMNS_____,
    SMLALBB__,
    ORR______,
    BL_______,
    LDMIB____,
    TEQS_____,
    Undefined,
}

pub const TABLE: [[ArmOpType; 16]; 256] = [
    [AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, MUL______, AND______, STRH_____, AND______, LDRD_____, AND______, STRD_____],  // 0
    [ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, MULS_____, ANDS_____, LDRH_____, ANDS_____, LDRSB____, ANDS_____, LDRSH____],  // 1
    [EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, MLA______, EOR______, STRH_____, EOR______, LDRD_____, EOR______, STRD_____],  // 2
    [EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, MLAS_____, EORS_____, LDRH_____, EORS_____, LDRSB____, EORS_____, LDRSH____],  // 3
    [SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, Undefined, SUB______, STRH_____, SUB______, LDRD_____, SUB______, STRD_____],  // 4
    [SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, Undefined, SUBS_____, LDRH_____, SUBS_____, LDRSB____, SUBS_____, LDRSH____],  // 5
    [RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, Undefined, RSB______, STRH_____, RSB______, LDRD_____, RSB______, STRD_____],  // 6
    [RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, Undefined, RSBS_____, LDRH_____, RSBS_____, LDRSB____, RSBS_____, LDRSH____],  // 7
    [ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, UMULL____, ADD______, STRH_____, ADD______, LDRD_____, ADD______, STRD_____],  // 8
    [ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, UMULLS___, ADDS_____, LDRH_____, ADDS_____, LDRSB____, ADDS_____, LDRSH____],  // 9
    [ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, UMLAL____, ADC______, STRH_____, ADC______, LDRD_____, ADC______, STRD_____],  // 10
    [ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, UMLALS___, ADCS_____, LDRH_____, ADCS_____, LDRSB____, ADCS_____, LDRSH____],  // 11
    [SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SMULL____, SBC______, STRH_____, SBC______, LDRD_____, SBC______, STRD_____],  // 12
    [SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SMULLS___, SBCS_____, LDRH_____, SBCS_____, LDRSB____, SBCS_____, LDRSH____],  // 13
    [RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, SMLAL____, RSC______, STRH_____, RSC______, LDRD_____, RSC______, STRD_____],  // 14
    [RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, SMLALS___, RSCS_____, LDRH_____, RSCS_____, LDRSB____, RSCS_____, LDRSH____],  // 15
    [MRS______, Undefined, Undefined, Undefined, Undefined, QADD_____, Undefined, Undefined, SMLABB___, SWP______, SMLATB___, STRH_____, SMLABT___, LDRD_____, SMLATT___, STRD_____],  // 16
    [TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, Undefined, TSTS_____, LDRH_____, TSTS_____, LDRSB____, TSTS_____, LDRSH____],  // 17
    [MSR______, BX_______, Undefined, BLX______, Undefined, QSUB_____, Undefined, BKPT_____, SMLAWB___, Undefined, SMULWB___, STRH_____, SMLAWT___, LDRD_____, SMULWT___, STRD_____],  // 18
    [TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, Undefined, TEQS_____, LDRH_____, TEQS_____, LDRSB____, TEQS_____, LDRSH____],  // 19
    [MRS______, Undefined, Undefined, Undefined, Undefined, QDADD____, Undefined, Undefined, SMLALBB__, SWPB_____, SMLALTB__, STRH_____, SMLALBT__, LDRD_____, SMLALTT__, STRD_____],  // 20
    [CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, Undefined, CMPS_____, LDRH_____, CMPS_____, LDRSB____, CMPS_____, LDRSH____],  // 21
    [MSR______, CLZ______, Undefined, Undefined, Undefined, QDSUB____, Undefined, Undefined, SMULBB___, Undefined, SMULTB___, STRH_____, SMULBT___, LDRD_____, SMULTT___, STRD_____],  // 22
    [CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, Undefined, CMNS_____, LDRH_____, CMNS_____, LDRSB____, CMNS_____, LDRSH____],  // 23
    [ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, Undefined, ORR______, STRH_____, ORR______, LDRD_____, ORR______, STRD_____],  // 24
    [ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, Undefined, ORRS_____, LDRH_____, ORRS_____, LDRSB____, ORRS_____, LDRSH____],  // 25
    [MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, Undefined, MOV______, STRH_____, MOV______, LDRD_____, MOV______, STRD_____],  // 26
    [MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, Undefined, MOVS_____, LDRH_____, MOVS_____, LDRSB____, MOVS_____, LDRSH____],  // 27
    [BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, Undefined, BIC______, STRH_____, BIC______, LDRD_____, BIC______, STRD_____],  // 28
    [BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, Undefined, BICS_____, LDRH_____, BICS_____, LDRSB____, BICS_____, LDRSH____],  // 29
    [MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, Undefined, MVN______, STRH_____, MVN______, LDRD_____, MVN______, STRD_____],  // 30
    [MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, Undefined, MVNS_____, LDRH_____, MVNS_____, LDRSB____, MVNS_____, LDRSH____],  // 31

    [AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______, AND______],  // 32
    [ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____, ANDS_____],  // 33
    [EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______, EOR______],  // 34
    [EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____, EORS_____],  // 35
    [SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______, SUB______],  // 36
    [SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____, SUBS_____],  // 37
    [RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______, RSB______],  // 38
    [RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____, RSBS_____],  // 39
    [ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______, ADD______],  // 40
    [ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____, ADDS_____],  // 41
    [ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______, ADC______],  // 42
    [ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____, ADCS_____],  // 43
    [SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______, SBC______],  // 44
    [SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____, SBCS_____],  // 45
    [RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______, RSC______],  // 46
    [RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____, RSCS_____],  // 47
    [Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined],  // 48
    [TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____],  // 49
    [MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______],  // 50
    [TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____],  // 51
    [Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined],  // 52
    [CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____],  // 53
    [MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______],  // 54
    [CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____],  // 55
    [ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______, ORR______],  // 56
    [ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____, ORRS_____],  // 57
    [MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______, MOV______],  // 58
    [MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____, MOVS_____],  // 59
    [BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______, BIC______],  // 60
    [BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____, BICS_____],  // 61
    [MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______, MVN______],  // 62
    [MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____, MVNS_____],  // 63
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______],  // 64
    [LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______],  // 65
    [STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____],  // 66
    [LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____],  // 67
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____],  // 68
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____],  // 69
    [STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____],  // 70
    [LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____],  // 71
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______],  // 72
    [LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______],  // 73
    [STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____],  // 74
    [LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____],  // 75
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____],  // 76
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____],  // 77
    [STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____],  // 78
    [LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____],  // 79
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______],  // 80
    [LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______],  // 81
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______],  // 82
    [LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______],  // 83
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____],  // 84
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____],  // 85
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____],  // 86
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____],  // 87
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______],  // 88
    [LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______],  // 89
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______],  // 90
    [LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______, LDR______],  // 91
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____],  // 92
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____],  // 93
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____],  // 94
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____],  // 95

    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined],  // 96
    [LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined],  // 97
    [STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined],  // 98
    [LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined],  // 99
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined],  // 100
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined],  // 101
    [STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined],  // 102
    [LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined],  // 103
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined],  // 104
    [LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined],  // 105
    [STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined],  // 106
    [LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined],  // 107
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined],  // 108
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined],  // 109
    [STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined],  // 110
    [LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined],  // 111
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined],  // 112
    [LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined],  // 113
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined],  // 114
    [LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined],  // 115
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined],  // 116
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined],  // 117
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined],  // 118
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined],  // 119
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined],  // 120
    [LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined],  // 121
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined],  // 122
    [LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined, LDR______, Undefined],  // 123
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined],  // 124
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined],  // 125
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined],  // 126
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined],  // 127

    [STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____],  // 128
    [LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____],  // 129
    [STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____],  // 130
    [LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____],  // 131
    [STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____],  // 132
    [LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____],  // 133
    [STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____, STMDA____],  // 134
    [LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____, LDMDA____],  // 135
    [STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____],  // 136
    [LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____],  // 137
    [STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____],  // 138
    [LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____],  // 139
    [STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____],  // 140
    [LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____],  // 141
    [STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____, STMIA____],  // 142
    [LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____, LDMIA____],  // 143
    [STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____],  // 144
    [LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____],  // 145
    [STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____],  // 146
    [LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____],  // 147
    [STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____],  // 148
    [LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____],  // 149
    [STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____, STMDB____],  // 150
    [LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____, LDMDB____],  // 151
    [STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____],  // 152
    [LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____],  // 153
    [STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____],  // 154
    [LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____],  // 155
    [STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____],  // 156
    [LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____],  // 157
    [STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____, STMIB____],  // 158
    [LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____, LDMIB____],  // 159

    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 160
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 161
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 162
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 163
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 164
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 165
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 166
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 167
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 168
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 169
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 170
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 171
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 172
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 173
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 174
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________],  // 175

    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 176
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 177
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 178
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 179
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 180
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 181
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 182
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 183
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 184
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 185
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 186
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 187
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 188
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 189
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 190
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______],  // 191

    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 192
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 193
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 194
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 195
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 196
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 197
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 198
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 199
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 200
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 201
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 202
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 203
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 204
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 205
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 206
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 207
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 208
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 209
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 210
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 211
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 212
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 213
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 214
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 215
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 216
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 217
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 218
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 219
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 220
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 221
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______],  // 222
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______],  // 223

    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______],  // 224
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______],  // 225
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______],  // 226
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______],  // 227
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______],  // 228
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______],  // 229
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______],  // 230
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______],  // 231
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______],  // 232
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______],  // 233
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______],  // 234
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______],  // 235
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______],  // 236
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______],  // 237
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______],  // 238
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______],  // 239

    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 240
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 241
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 242
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 243
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 244
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 245
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 246
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 247
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 248
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 249
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 250
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 251
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 252
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 253
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 254
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______],  // 255
];