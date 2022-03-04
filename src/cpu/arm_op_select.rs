use crate::cpu::arm;
use crate::cpu::arm_op_select::ArmOrderType::{Add, B, Bl, Cdp, Ldc, Mcr, Mrc, Stc, Swi, Todo};

enum ArmOrderType {
    // bool：set flag
    Add(bool),
    Swi,
    B,
    Bl,
    Stc,
    Ldc,
    Mcr,
    Mrc,
    Cdp,
    Todo,
}

fn get() {
    let x = [
        [Add(false), Add(false), Add(false), Add(false), Add(false), Add(false), Add(false), Add(false), Add(false), Todo, Add(false), Todo, Add(false), Todo, Add(false), Todo], // 0
        [Add(true), Add(true), Add(true), Add(true), Add(true), Add(true), Add(true), Add(true), Add(true), Todo, Add(true), Todo, Add(true), Todo, Add(true), Todo], // 1
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 2
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 3
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 4
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 5
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 6
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 7
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 8
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 9
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 10
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 11
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 12
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 13
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 14
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 15
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 16
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 17
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 18
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 19
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 20
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 21
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 22
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 23
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 24
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 25
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 26
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 27
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 28
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 29
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 30
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 31
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 32
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 33
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 34
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 35
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 36
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 37
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 38
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 39
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 30
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 31
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 32
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 33
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 34
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 35
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 36
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 37
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 38
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 39
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 40
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 41
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 42
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 43
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 44
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 45
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 46
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 47
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 48
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 49
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 50
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 51
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 52
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 53
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 54
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 55
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 56
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 57
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 58
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 59
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 60
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 61
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 62
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 63
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 64
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 65
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 66
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 67
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 68
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 69
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 70
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 71
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 72
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 73
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 74
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 75
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 76
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 77
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 78
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 79
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 80
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 81
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 82
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 83
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 84
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 85
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 86
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 87
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 88
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 89
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 90
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 91
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 92
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 93
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 94
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 95
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 96
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 97
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 98
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 99

        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 100
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 101
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 102
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 103
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 104
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 105
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 106
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 107
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 108
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 109
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 110
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 111
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 112
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 113
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 114
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 115
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 116
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 117
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 118
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 119
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 120
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 121
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 122
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 123
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 124
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 125
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 126
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 127
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 128
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 129
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 130
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 131
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 132
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 133
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 134
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 135
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 136
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 137
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 138
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 139
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 130
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 131
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 132
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 133
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 134
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 135
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 136
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 137
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 138
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 139
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 140
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 141
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 142
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 143
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 144
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 145
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 146
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 147
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 148
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 149
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 150
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 151
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 152
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 153
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 154
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 155
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 156
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 157
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 158
        [Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo, Todo], // 159


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


        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 176
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 177
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 178
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 179
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 180
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 181
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 182
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 183
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 184
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 185
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 186
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 187
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 188
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 189
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 190
        [Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl], // 191


        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 192
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 193
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 194
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 195
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 196
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 197
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 198
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 199
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 200
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 201
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 202
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 203
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 204
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 205
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 206
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 207
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 208
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 209
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 210
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 211
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 212
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 213
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 214
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 215
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 216
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 217
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 218
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 219
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 220
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 221
        [Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc, Stc], // 222
        [Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc, Ldc], // 223


        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 224
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 225
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 226
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 227
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 228
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 229
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 230
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 231
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 232
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 233
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 234
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 235
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 236
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 237
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 238
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 239
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 230
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 231
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 232
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 233
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 234
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 235
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 236
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 237
        [Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr, Cdp, Mcr], // 238
        [Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc, Cdp, Mrc], // 239

        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 240
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 241
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 242
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 243
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 244
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 245
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 246
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 247
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 248
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 249
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 250
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 251
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 252
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 253
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 254
        [Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi, Swi], // 255
    ];
}
