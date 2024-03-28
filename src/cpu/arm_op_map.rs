use crate::cpu::arm_op_map::ArmOpType::{
    Undef__, ADCS___, ADC____, ADDS___, ADD____, ANDS___, AND____, BICS___, BIC____, BL_____, BX_____, B______, CDP____, CMNS___,
    CMPS___, EORS___, EOR____, LDC____, LDMDA__, LDMDB__, LDMIA__, LDMIB__, LDRBT__, LDRB___, LDRH___, LDRSB__, LDRSH__, LDRT___,
    LDR____, MCR____, MLAS___, MLA____, MOVS___, MOV____, MRC____, MRS____, MSR____, MULS___, MUL____, MVNS___, MVN____, ORRS___,
    ORR____, RSBS___, RSB____, RSCS___, RSC____, SBCS___, SBC____, SMLALS_, SMLAL__, SMULLS_, SMULL__, STC____, STMDA__, STMDB__,
    STMIA__, STMIB__, STRBT__, STRB___, STRH___, STRT___, STR____, SUBS___, SUB____, SWI____, SWPB___, SWP____, TEQS___, TSTS___,
    UMLALS_, UMLAL__, UMULLS_, UMULL__,
};

#[derive(Copy, Clone)]
pub enum ArmOpType {
    STMDA__,
    STC____,
    MCR____,
    UMLAL__,
    SMULLS_,
    RSC____,
    ADD____,
    MRS____,
    STMDB__,

    STMIB__,
    ADC____,
    B______,

    STR____,
    STRT___,
    STRB___,
    STRBT__,
    STRH___,

    LDR____,
    LDRSB__,
    LDRB___,
    LDRBT__,
    LDRSH__,
    LDRH___,
    LDRT___,

    ADCS___,
    BICS___,
    LDMIA__,
    MRC____,
    RSBS___,
    MULS___,
    AND____,
    MLAS___,
    MUL____,
    SUBS___,
    SMULL__,
    ANDS___,
    SBC____,
    SUB____,
    SMLALS_,
    BX_____,
    MOVS___,
    MLA____,
    EORS___,
    STMIA__,
    LDMDA__,
    RSB____,
    TSTS___,
    MVN____,
    RSCS___,
    UMLALS_,
    EOR____,
    SMLAL__,
    SBCS___,
    MVNS___,
    CDP____,
    MSR____,
    LDMDB__,
    ADDS___,
    ORRS___,
    SWI____,
    BIC____,
    MOV____,
    CMPS___,
    UMULLS_,
    LDC____,
    SWP____,
    SWPB___,
    UMULL__,
    CMNS___,
    ORR____,
    BL_____,
    LDMIB__,
    TEQS___,
    Undef__,
}

#[rustfmt::skip]
pub const ARM_MAP: [[ArmOpType; 16]; 256] = [
    [AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, MUL____, AND____, STRH___, AND____, Undef__, AND____, Undef__],  // 0
    [ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, MULS___, ANDS___, LDRH___, ANDS___, LDRSB__, ANDS___, LDRSH__],  // 1
    [EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, MLA____, EOR____, STRH___, EOR____, Undef__, EOR____, Undef__],  // 2
    [EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, MLAS___, EORS___, LDRH___, EORS___, LDRSB__, EORS___, LDRSH__],  // 3
    [SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, Undef__, SUB____, STRH___, SUB____, Undef__, SUB____, Undef__],  // 4
    [SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, Undef__, SUBS___, LDRH___, SUBS___, LDRSB__, SUBS___, LDRSH__],  // 5
    [RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, Undef__, RSB____, STRH___, RSB____, Undef__, RSB____, Undef__],  // 6
    [RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, Undef__, RSBS___, LDRH___, RSBS___, LDRSB__, RSBS___, LDRSH__],  // 7
    [ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, UMULL__, ADD____, STRH___, ADD____, Undef__, ADD____, Undef__],  // 8
    [ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, UMULLS_, ADDS___, LDRH___, ADDS___, LDRSB__, ADDS___, LDRSH__],  // 9
    [ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, UMLAL__, ADC____, STRH___, ADC____, Undef__, ADC____, Undef__],  // 10
    [ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, UMLALS_, ADCS___, LDRH___, ADCS___, LDRSB__, ADCS___, LDRSH__],  // 11
    [SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SMULL__, SBC____, STRH___, SBC____, Undef__, SBC____, Undef__],  // 12
    [SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SMULLS_, SBCS___, LDRH___, SBCS___, LDRSB__, SBCS___, LDRSH__],  // 13
    [RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, SMLAL__, RSC____, STRH___, RSC____, Undef__, RSC____, Undef__],  // 14
    [RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, SMLALS_, RSCS___, LDRH___, RSCS___, LDRSB__, RSCS___, LDRSH__],  // 15
    [MRS____, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, SWP____, Undef__, STRH___, Undef__, Undef__, Undef__, Undef__],  // 16
    [TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, Undef__, TSTS___, LDRH___, TSTS___, LDRSB__, TSTS___, LDRSH__],  // 17
    [MSR____, BX_____, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, STRH___, Undef__, Undef__, Undef__, Undef__],  // 18
    [TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, Undef__, TEQS___, LDRH___, TEQS___, LDRSB__, TEQS___, LDRSH__],  // 19
    [MRS____, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, SWPB___, Undef__, STRH___, Undef__, Undef__, Undef__, Undef__],  // 20
    [CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, Undef__, CMPS___, LDRH___, CMPS___, LDRSB__, CMPS___, LDRSH__],  // 21
    [MSR____, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, STRH___, Undef__, Undef__, Undef__, Undef__],  // 22
    [CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, Undef__, CMNS___, LDRH___, CMNS___, LDRSB__, CMNS___, LDRSH__],  // 23
    [ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, Undef__, ORR____, STRH___, ORR____, Undef__, ORR____, Undef__],  // 24
    [ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, Undef__, ORRS___, LDRH___, ORRS___, LDRSB__, ORRS___, LDRSH__],  // 25
    [MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, Undef__, MOV____, STRH___, MOV____, Undef__, MOV____, Undef__],  // 26
    [MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, Undef__, MOVS___, LDRH___, MOVS___, LDRSB__, MOVS___, LDRSH__],  // 27
    [BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, Undef__, BIC____, STRH___, BIC____, Undef__, BIC____, Undef__],  // 28
    [BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, Undef__, BICS___, LDRH___, BICS___, LDRSB__, BICS___, LDRSH__],  // 29
    [MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, Undef__, MVN____, STRH___, MVN____, Undef__, MVN____, Undef__],  // 30
    [MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, Undef__, MVNS___, LDRH___, MVNS___, LDRSB__, MVNS___, LDRSH__],  // 31

    [AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____, AND____],  // 32
    [ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___, ANDS___],  // 33
    [EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____, EOR____],  // 34
    [EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___, EORS___],  // 35
    [SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____, SUB____],  // 36
    [SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___, SUBS___],  // 37
    [RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____, RSB____],  // 38
    [RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___, RSBS___],  // 39
    [ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____, ADD____],  // 40
    [ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___, ADDS___],  // 41
    [ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____, ADC____],  // 42
    [ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___, ADCS___],  // 43
    [SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____, SBC____],  // 44
    [SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___, SBCS___],  // 45
    [RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____, RSC____],  // 46
    [RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___, RSCS___],  // 47
    [Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__],  // 48
    [TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___, TSTS___],  // 49
    [MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____],  // 50
    [TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___, TEQS___],  // 51
    [Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__, Undef__],  // 52
    [CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___, CMPS___],  // 53
    [MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____, MSR____],  // 54
    [CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___, CMNS___],  // 55
    [ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____, ORR____],  // 56
    [ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___, ORRS___],  // 57
    [MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____, MOV____],  // 58
    [MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___, MOVS___],  // 59
    [BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____, BIC____],  // 60
    [BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___, BICS___],  // 61
    [MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____, MVN____],  // 62
    [MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___, MVNS___],  // 63
    [STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____],  // 64
    [LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____],  // 65
    [STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___],  // 66
    [LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___],  // 67
    [STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___],  // 68
    [LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___],  // 69
    [STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__],  // 70
    [LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__],  // 71
    [STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____],  // 72
    [LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____],  // 73
    [STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___, STRT___],  // 74
    [LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___, LDRT___],  // 75
    [STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___],  // 76
    [LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___],  // 77
    [STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__, STRBT__],  // 78
    [LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__, LDRBT__],  // 79
    [STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____],  // 80
    [LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____],  // 81
    [STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____],  // 82
    [LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____],  // 83
    [STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___],  // 84
    [LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___],  // 85
    [STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___],  // 86
    [LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___],  // 87
    [STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____],  // 88
    [LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____],  // 89
    [STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____, STR____],  // 90
    [LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____, LDR____],  // 91
    [STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___],  // 92
    [LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___],  // 93
    [STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___, STRB___],  // 94
    [LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___, LDRB___],  // 95

    [STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__],  // 96
    [LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__],  // 97
    [STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__],  // 98
    [LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__],  // 99
    [STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__],  // 100
    [LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__],  // 101
    [STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__],  // 102
    [LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__],  // 103
    [STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__],  // 104
    [LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__],  // 105
    [STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__, STRT___, Undef__],  // 106
    [LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__, LDRT___, Undef__],  // 107
    [STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__],  // 108
    [LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__],  // 109
    [STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__, STRBT__, Undef__],  // 110
    [LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__, LDRBT__, Undef__],  // 111
    [STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__],  // 112
    [LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__],  // 113
    [STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__],  // 114
    [LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__],  // 115
    [STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__],  // 116
    [LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__],  // 117
    [STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__],  // 118
    [LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__],  // 119
    [STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__],  // 120
    [LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__],  // 121
    [STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__, STR____, Undef__],  // 122
    [LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__, LDR____, Undef__],  // 123
    [STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__],  // 124
    [LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__],  // 125
    [STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__, STRB___, Undef__],  // 126
    [LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__, LDRB___, Undef__],  // 127

    [STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__],  // 128
    [LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__],  // 129
    [STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__],  // 130
    [LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__],  // 131
    [STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__],  // 132
    [LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__],  // 133
    [STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__, STMDA__],  // 134
    [LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__, LDMDA__],  // 135
    [STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__],  // 136
    [LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__],  // 137
    [STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__],  // 138
    [LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__],  // 139
    [STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__],  // 140
    [LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__],  // 141
    [STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__, STMIA__],  // 142
    [LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__, LDMIA__],  // 143
    [STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__],  // 144
    [LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__],  // 145
    [STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__],  // 146
    [LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__],  // 147
    [STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__],  // 148
    [LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__],  // 149
    [STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__, STMDB__],  // 150
    [LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__, LDMDB__],  // 151
    [STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__],  // 152
    [LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__],  // 153
    [STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__],  // 154
    [LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__],  // 155
    [STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__],  // 156
    [LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__],  // 157
    [STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__, STMIB__],  // 158
    [LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__, LDMIB__],  // 159

    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 160
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 161
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 162
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 163
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 164
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 165
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 166
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 167
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 168
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 169
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 170
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 171
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 172
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 173
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 174
    [B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______, B______],  // 175

    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 176
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 177
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 178
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 179
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 180
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 181
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 182
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 183
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 184
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 185
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 186
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 187
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 188
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 189
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 190
    [BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____, BL_____],  // 191

    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 192
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 193
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 194
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 195
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 196
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 197
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 198
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 199
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 200
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 201
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 202
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 203
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 204
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 205
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 206
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 207
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 208
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 209
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 210
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 211
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 212
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 213
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 214
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 215
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 216
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 217
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 218
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 219
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 220
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 221
    [STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____, STC____],  // 222
    [LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____, LDC____],  // 223

    [CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____],  // 224
    [CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____],  // 225
    [CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____],  // 226
    [CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____],  // 227
    [CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____],  // 228
    [CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____],  // 229
    [CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____],  // 230
    [CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____],  // 231
    [CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____],  // 232
    [CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____],  // 233
    [CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____],  // 234
    [CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____],  // 235
    [CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____],  // 236
    [CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____],  // 237
    [CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____, CDP____, MCR____],  // 238
    [CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____, CDP____, MRC____],  // 239

    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 240
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 241
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 242
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 243
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 244
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 245
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 246
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 247
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 248
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 249
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 250
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 251
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 252
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 253
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 254
    [SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____, SWI____],  // 255
];
