use crate::cpu::arm;
use crate::cpu::arm_op_select::ArmOrderType::{ADC___, ADD___, AND___, B________, BIC___, BL_______, BX_______, CDP______, CMNS_____, CMPS_____, EOR___, LDC______, LDRB_____, LDRBT____, LDRH_____, LDRSB____, LDRSH____, LDRT_____, MCR______, MOV___, MRC______, MRS______, MSR______, MUL___, MVN___, ORR___, RSB___, RSC___, SBC___, STC______, STR______, STRB_____, STRBT____, STRH_____, STRT_____, SUB___, SWI______, TEQS_____, TODO_____, TSTS_____, Undefined};

enum ArmOrderType {
    AND___(u8),
    MUL___(u8),
    EOR___(u8),
    SUB___(u8),
    RSB___(u8),
    ADD___(u8),
    ADC___(u8),
    SBC___(u8),
    RSC___(u8),
    MRS______,
    TSTS_____,
    MSR______,
    BX_______,
    TEQS_____,
    CMPS_____,
    CMNS_____,
    ORR___(u8),
    MOV___(u8),
    BIC___(u8),
    MVN___(u8),

    STR______,
    STRH_____,
    STRT_____,
    STRB_____,
    STRBT____,

    LDRH_____,
    LDRSB____,
    LDRSH____,
    LDRT_____,
    LDRB_____,
    LDRBT____,

    SWI______,
    B________,
    BL_______,
    STC______,
    LDC______,
    MCR______,
    MRC______,
    CDP______,
    TODO_____,
    Undefined,
}

const TABLE: [[ArmOrderType; 16]; 256] = [
    [AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), MUL___(0), AND___(0), STRH_____, AND___(0), TODO_____, AND___(0), TODO_____], // 0
    [AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), MUL___(1), AND___(1), LDRH_____, AND___(1), LDRSB____, AND___(1), LDRSH____], // 1
    [EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), TODO_____, EOR___(0), STRH_____, EOR___(0), TODO_____, EOR___(0), TODO_____], // 2
    [EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), TODO_____, EOR___(1), LDRH_____, EOR___(1), LDRSB____, EOR___(1), LDRSH____], // 3
    [SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), Undefined, SUB___(0), STRH_____, SUB___(0), TODO_____, SUB___(0), TODO_____], // 4
    [SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), Undefined, SUB___(1), LDRH_____, SUB___(1), LDRSB____, SUB___(1), LDRSH____], // 5
    [RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), Undefined, RSB___(0), STRH_____, RSB___(0), TODO_____, RSB___(0), TODO_____], // 6
    [RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), Undefined, RSB___(1), LDRH_____, RSB___(1), LDRSB____, RSB___(1), LDRSH____], // 7
    [ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), TODO_____, ADD___(0), STRH_____, ADD___(0), TODO_____, ADD___(0), TODO_____], // 8
    [ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), TODO_____, ADD___(1), LDRH_____, ADD___(1), LDRSB____, ADD___(1), LDRSH____], // 9
    [ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), TODO_____, ADC___(0), STRH_____, ADC___(0), TODO_____, ADC___(0), TODO_____], // 10
    [ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), TODO_____, ADC___(1), LDRH_____, ADC___(1), LDRSB____, ADC___(1), LDRSH____], // 11
    [SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), TODO_____, SBC___(0), STRH_____, SBC___(0), TODO_____, SBC___(0), TODO_____], // 12
    [SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), TODO_____, SBC___(1), LDRH_____, SBC___(1), LDRSB____, SBC___(1), LDRSH____], // 13
    [RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), TODO_____, RSC___(0), STRH_____, RSC___(0), TODO_____, RSC___(0), TODO_____], // 14
    [RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), TODO_____, RSC___(1), LDRH_____, RSC___(1), LDRSB____, RSC___(1), LDRSH____], // 15

    [MRS______, Undefined, Undefined, Undefined, Undefined, TODO_____, Undefined, Undefined, TODO_____, TODO_____, TODO_____, STRH_____, TODO_____, TODO_____, TODO_____, TODO_____], // 16
    [TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, Undefined, TSTS_____, LDRH_____, TSTS_____, LDRSB____, TSTS_____, LDRSH____], // 17
    [MSR______, BX_______, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, TODO_____, Undefined, TODO_____, STRH_____, TODO_____, TODO_____, TODO_____, TODO_____], // 18
    [TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, Undefined, TEQS_____, LDRH_____, TEQS_____, LDRSB____, TEQS_____, LDRSH____], // 19
    [MRS______, Undefined, Undefined, Undefined, Undefined, TODO_____, Undefined, Undefined, TODO_____, TODO_____, TODO_____, STRH_____, TODO_____, TODO_____, TODO_____, TODO_____], // 20
    [CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, Undefined, CMPS_____, LDRH_____, CMPS_____, LDRSB____, CMPS_____, LDRSH____], // 21
    [MSR______, TODO_____, Undefined, Undefined, Undefined, TODO_____, Undefined, Undefined, TODO_____, Undefined, TODO_____, STRH_____, TODO_____, TODO_____, TODO_____, TODO_____], // 22
    [CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, TODO_____, CMNS_____, LDRH_____, CMNS_____, LDRSB____, CMNS_____, LDRSH____], // 23

    [ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), Undefined, ORR___(0), STRH_____, ORR___(0), TODO_____, ORR___(0), TODO_____], // 24
    [ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), Undefined, ORR___(1), LDRH_____, ORR___(1), LDRSB____, ORR___(1), LDRSH____], // 25
    [MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), TODO_____, MOV___(0), STRH_____, MOV___(0), TODO_____, MOV___(0), TODO_____], // 26
    [MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), TODO_____, MOV___(1), LDRH_____, MOV___(1), LDRSB____, MOV___(1), LDRSH____], // 27
    [BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), TODO_____, BIC___(0), STRH_____, BIC___(0), TODO_____, BIC___(0), TODO_____], // 28
    [BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), TODO_____, BIC___(1), LDRH_____, BIC___(1), LDRSB____, BIC___(1), LDRSH____], // 29
    [MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), TODO_____, MVN___(0), STRH_____, MVN___(0), TODO_____, MVN___(0), TODO_____], // 30
    [MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), TODO_____, MVN___(1), LDRH_____, MVN___(1), LDRSB____, MVN___(1), LDRSH____], // 31


    [AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0), AND___(0)], // 32
    [AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1), AND___(1)], // 33
    [EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0), EOR___(0)], // 34
    [EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1), EOR___(1)], // 35
    [SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0), SUB___(0)], // 36
    [SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1), SUB___(1)], // 37
    [RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0), RSB___(0)], // 38
    [RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1), RSB___(1)], // 39
    [ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0), ADD___(0)], // 40
    [ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1), ADD___(1)], // 41
    [ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0), ADC___(0)], // 42
    [ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1), ADC___(1)], // 43
    [SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0), SBC___(0)], // 44
    [SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1), SBC___(1)], // 45
    [RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0), RSC___(0)], // 46
    [RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1), RSC___(1)], // 47

    [Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined], // 48

    [TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____, TSTS_____], // 49
    [MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______], // 50
    [TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____, TEQS_____], // 51

    [Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined], // 52

    [CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____, CMPS_____], // 53
    [MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______, MSR______], // 54
    [CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____, CMNS_____], // 55
    [ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0), ORR___(0)], // 56
    [ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1), ORR___(1)], // 57
    [MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0), MOV___(0)], // 58
    [MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1), MOV___(1)], // 59
    [BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0), BIC___(0)], // 60
    [BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1), BIC___(1)], // 61
    [MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0), MVN___(0)], // 62
    [MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1), MVN___(1)], // 63


    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______], // 64
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 65
    [STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____], // 66
    [LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____, LDRT_____], // 67
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____], // 68
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____], // 69
    [STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____], // 70
    [LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____], // 71
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______], // 72
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 73
    [STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____, STRT_____], // 74
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 75
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____], // 76
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____], // 77
    [STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____, STRBT____], // 78
    [LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____, LDRBT____], // 79
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______], // 80
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 81
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______], // 82
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 83
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____], // 84
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____], // 85
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____], // 86
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____], // 87
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______], // 88
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 89
    [STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______, STR______], // 90
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 91
    [STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____, STRB_____], // 92
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____], // 93
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 94
    [LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____, LDRB_____], // 95


    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined], // 96
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 97
    [STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined, STRT_____, Undefined], // 98
    [LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined, LDRT_____, Undefined], // 99
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined], // 100
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined], // 101
    [STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined], // 102
    [LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined], // 103
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined], // 104
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 105
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 106
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 107
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 108
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined], // 109
    [STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined, STRBT____, Undefined], // 110
    [LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined, LDRBT____, Undefined], // 111
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined], // 112
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 113
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined], // 114
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 115
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 116
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined], // 117
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 118
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined], // 119
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined], // 120
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 121
    [STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined, STR______, Undefined], // 122
    [TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined, TODO_____, Undefined], // 123
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined], // 124
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined], // 125
    [STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined, STRB_____, Undefined], // 126
    [LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined, LDRB_____, Undefined], // 127


    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 128
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 129
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 130
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 131
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 132
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 133
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 134
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 135
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 136
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 137
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 138
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 139
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 140
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 141
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 142
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 143
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 144
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 145
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 146
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 147
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 148
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 149
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 150
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 151
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 152
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 153
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 154
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 155
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 156
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 157
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 158
    [TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____, TODO_____], // 159


    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 160
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 161
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 162
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 163
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 164
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 165
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 166
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 167
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 168
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 169
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 170
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 171
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 172
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 173
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 174
    [B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________, B________], // 175


    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 176
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 177
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 178
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 179
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 180
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 181
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 182
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 183
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 184
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 185
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 186
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 187
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 188
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 189
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 190
    [BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______, BL_______], // 191


    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 192
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 193
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 194
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 195
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 196
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 197
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 198
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 199
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 200
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 201
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 202
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 203
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 204
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 205
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 206
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 207
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 208
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 209
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 210
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 211
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 212
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 213
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 214
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 215
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 216
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 217
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 218
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 219
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 220
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 221
    [STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______, STC______], // 222
    [LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______, LDC______], // 223


    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______], // 224
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______], // 225
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______], // 226
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______], // 227
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______], // 228
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______], // 229
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______], // 230
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______], // 231
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______], // 232
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______], // 233
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______], // 234
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______], // 235
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______], // 236
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______], // 237
    [CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______, CDP______, MCR______], // 238
    [CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______, CDP______, MRC______], // 239

    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 240
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 241
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 242
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 243
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 244
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 245
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 246
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 247
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 248
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 249
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 250
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 251
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 252
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 253
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 254
    [SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______, SWI______], // 255
];

