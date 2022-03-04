use crate::cpu::arm;
use crate::cpu::arm_op_select::ArmOrderType::{ADC, ADD, AND, B, BL, CDP, EOR, LDC, MCR, MRC, MUL, RSB, RSC, SBC, STC, SUB, SWI, TODO, Undefined};

enum ArmOrderType {
    // bool：set flag
    AND(bool),
    // bool：set flag
    MUL(bool),
    // bool：set flag
    EOR(bool),
    // bool：set flag
    SUB(bool),
    // bool：set flag
    RSB(bool),
    // bool：set flag
    ADD(bool),
    // bool：set flag
    ADC(bool),
    SBC(bool),
    RSC(bool),
    SWI,
    B,
    BL,
    STC,
    LDC,
    MCR,
    MRC,
    CDP,
    TODO,
    Undefined,
}

const TABLE: [[ArmOrderType; 16]; 256] = [
    [AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), MUL(false), AND(false), TODO, AND(false), TODO, AND(false), TODO], // 0
    [AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), MUL(true), AND(true), TODO, AND(true), TODO, AND(true), TODO], // 1
    [EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), TODO, EOR(false), TODO, EOR(false), TODO, EOR(false), TODO], // 2
    [EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), TODO, EOR(true), TODO, EOR(true), TODO, EOR(true), TODO], // 3
    [SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), Undefined, SUB(false), TODO, SUB(false), TODO, SUB(false), TODO], // 4
    [SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), Undefined, SUB(true), TODO, SUB(true), TODO, SUB(true), TODO], // 5
    [RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), Undefined, RSB(false), TODO, RSB(false), TODO, RSB(false), TODO], // 6
    [RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), Undefined, RSB(true), TODO, RSB(true), TODO, RSB(true), TODO], // 7
    [ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), TODO, ADD(false), TODO, ADD(false), TODO, ADD(false), TODO], // 8
    [ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), TODO, ADD(true), TODO, ADD(true), TODO, ADD(true), TODO], // 9
    [ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), TODO, ADC(false), TODO, ADC(false), TODO, ADC(false), TODO], // 10
    [ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), TODO, ADC(true), TODO, ADC(true), TODO, ADC(true), TODO], // 11
    [SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), TODO, SBC(false), TODO, SBC(false), TODO, SBC(false), TODO], // 12
    [SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), TODO, SBC(true), TODO, SBC(true), TODO, SBC(true), TODO], // 13
    [RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), TODO, RSC(false), TODO, RSC(false), TODO, RSC(false), TODO], // 14
    [RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), TODO, RSC(true), TODO, RSC(true), TODO, RSC(true), TODO], // 15
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 16
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 17
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 18
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 19
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 20
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 21
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 22

    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 23
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 24
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 25
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 26
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 27
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 28
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 29
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 30
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 31


    [AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false), AND(false)], // 32
    [AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true), AND(true)], // 33
    [EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false), EOR(false)], // 34
    [EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true), EOR(true)], // 35
    [SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false), SUB(false)], // 36
    [SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true), SUB(true)], // 37
    [RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false), RSB(false)], // 38
    [RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true), RSB(true)], // 39
    [ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false), ADD(false)], // 40
    [ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true), ADD(true)], // 41
    [ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false), ADC(false)], // 42
    [ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true), ADC(true)], // 43
    [SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false), SBC(false)], // 44
    [SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true), SBC(true)], // 45
    [RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false), RSC(false)], // 46
    [RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true), RSC(true)], // 47

    [Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined, Undefined], // 48

    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 49
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 50
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 51
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 52
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 53
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 54
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 55
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 56
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 57
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 58
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 59
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 60
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 61
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 62
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 63
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 64
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 65
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 66
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 67
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 68
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 69
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 70
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 71
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 72
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 73
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 74
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 75
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 76
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 77
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 78
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 79
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 80
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 81
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 82
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 83
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 84
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 85
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 86
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 87
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 88
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 89
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 90
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 91
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 92
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 93
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 94
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 95


    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 96
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 97
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 98
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 99
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 100
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 101
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 102
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 103
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 104
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 105
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 106
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 107
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 108
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 109
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 110
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 111
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 112
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 113
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 114
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 115
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 116
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 117
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 118
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 119
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 120
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 121
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 122
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 123
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 124
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 125
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 126
    [TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined, TODO, Undefined], // 127


    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 128
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 129
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 130
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 131
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 132
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 133
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 134
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 135
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 136
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 137
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 138
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 139
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 140
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 141
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 142
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 143
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 144
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 145
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 146
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 147
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 148
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 149
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 150
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 151
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 152
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 153
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 154
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 155
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 156
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 157
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 158
    [TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO, TODO], // 159


    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 160
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 161
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 162
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 163
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 164
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 165
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 166
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 167
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 168
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 169
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 170
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 171
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 172
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 173
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 174
    [B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B], // 175


    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 176
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 177
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 178
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 179
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 180
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 181
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 182
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 183
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 184
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 185
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 186
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 187
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 188
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 189
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 190
    [BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL], // 191


    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 192
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 193
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 194
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 195
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 196
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 197
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 198
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 199
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 200
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 201
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 202
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 203
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 204
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 205
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 206
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 207
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 208
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 209
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 210
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 211
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 212
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 213
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 214
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 215
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 216
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 217
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 218
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 219
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 220
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 221
    [STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC, STC], // 222
    [LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC, LDC], // 223


    [CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR], // 224
    [CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC], // 225
    [CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR], // 226
    [CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC], // 227
    [CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR], // 228
    [CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC], // 229
    [CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR], // 230
    [CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC], // 231
    [CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR], // 232
    [CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC], // 233
    [CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR], // 234
    [CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC], // 235
    [CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR], // 236
    [CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC], // 237
    [CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR, CDP, MCR], // 238
    [CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC, CDP, MRC], // 239

    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 240
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 241
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 242
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 243
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 244
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 245
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 246
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 247
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 248
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 249
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 250
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 251
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 252
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 253
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 254
    [SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI, SWI], // 255
];

