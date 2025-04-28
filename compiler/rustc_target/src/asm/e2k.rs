use std::fmt;

use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    E2k E2kInlineAsmRegClass {
        reg,
    }
}

impl E2kInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::reg => types! {
            _: I8, I16, I32, I64, F32, F64,
              VecI8(8), VecI16(4), VecI32(2), VecF32(2); },
        }
    }
}

def_regs! {
    E2k E2kInlineAsmReg E2kInlineAsmRegClass {
        r0: reg = ["r0", "dr0"],
        r1: reg = ["r1", "dr1"],
        r2: reg = ["r2", "dr2"],
        r3: reg = ["r3", "dr3"],
        r4: reg = ["r4", "dr4"],
        r5: reg = ["r5", "dr5"],
        r6: reg = ["r6", "dr6"],
        r7: reg = ["r7", "dr7"],
        r8: reg = ["r8", "dr8"],
        r9: reg = ["r9", "dr9"],
        r10: reg = ["r10", "dr10"],
        r11: reg = ["r11", "dr11"],
        r12: reg = ["r12", "dr12"],
        r13: reg = ["r13", "dr13"],
        r14: reg = ["r14", "dr14"],
        r15: reg = ["r15", "dr15"],
        r16: reg = ["r16", "dr16"],
        r17: reg = ["r17", "dr17"],
        r18: reg = ["r18", "dr18"],
        r19: reg = ["r19", "dr19"],
        r20: reg = ["r20", "dr20"],
        r21: reg = ["r21", "dr21"],
        r22: reg = ["r22", "dr22"],
        r23: reg = ["r23", "dr23"],
        r24: reg = ["r24", "dr24"],
        r25: reg = ["r25", "dr25"],
        r26: reg = ["r26", "dr26"],
        r27: reg = ["r27", "dr27"],
        r28: reg = ["r28", "dr28"],
        r29: reg = ["r29", "dr29"],
        r30: reg = ["r30", "dr30"],
        r31: reg = ["r31", "dr31"],
        r32: reg = ["r32", "dr32"],
        r33: reg = ["r33", "dr33"],
        r34: reg = ["r34", "dr34"],
        r35: reg = ["r35", "dr35"],
        r36: reg = ["r36", "dr36"],
        r37: reg = ["r37", "dr37"],
        r38: reg = ["r38", "dr38"],
        r39: reg = ["r39", "dr39"],
        r40: reg = ["r40", "dr40"],
        r41: reg = ["r41", "dr41"],
        r42: reg = ["r42", "dr42"],
        r43: reg = ["r43", "dr43"],
        r44: reg = ["r44", "dr44"],
        r45: reg = ["r45", "dr45"],
        r46: reg = ["r46", "dr46"],
        r47: reg = ["r47", "dr47"],
        r48: reg = ["r48", "dr48"],
        r49: reg = ["r49", "dr49"],
        r50: reg = ["r50", "dr50"],
        r51: reg = ["r51", "dr51"],
        r52: reg = ["r52", "dr52"],
        r53: reg = ["r53", "dr53"],
        r54: reg = ["r54", "dr54"],
        r55: reg = ["r55", "dr55"],
        r56: reg = ["r56", "dr56"],
        r57: reg = ["r57", "dr57"],
        r58: reg = ["r58", "dr58"],
        r59: reg = ["r59", "dr59"],
        r60: reg = ["r60", "dr60"],
        r61: reg = ["r61", "dr61"],
        r62: reg = ["r62", "dr62"],
        r63: reg = ["r63", "dr63"],

        b0: reg = ["b[0]", "db[0]"],
        b1: reg = ["b[1]", "db[1]"],
        b2: reg = ["b[2]", "db[2]"],
        b3: reg = ["b[3]", "db[3]"],
        b4: reg = ["b[4]", "db[4]"],
        b5: reg = ["b[5]", "db[5]"],
        b6: reg = ["b[6]", "db[6]"],
        b7: reg = ["b[7]", "db[7]"],
        b8: reg = ["b[8]", "db[8]"],
        b9: reg = ["b[9]", "db[9]"],
        b10: reg = ["b[10]", "db[10]"],
        b11: reg = ["b[11]", "db[11]"],
        b12: reg = ["b[12]", "db[12]"],
        b13: reg = ["b[13]", "db[13]"],
        b14: reg = ["b[14]", "db[14]"],
        b15: reg = ["b[15]", "db[15]"],
        b16: reg = ["b[16]", "db[16]"],
        b17: reg = ["b[17]", "db[17]"],
        b18: reg = ["b[18]", "db[18]"],
        b19: reg = ["b[19]", "db[19]"],
        b20: reg = ["b[20]", "db[20]"],
        b21: reg = ["b[21]", "db[21]"],
        b22: reg = ["b[22]", "db[22]"],
        b23: reg = ["b[23]", "db[23]"],
        b24: reg = ["b[24]", "db[24]"],
        b25: reg = ["b[25]", "db[25]"],
        b26: reg = ["b[26]", "db[26]"],
        b27: reg = ["b[27]", "db[27]"],
        b28: reg = ["b[28]", "db[28]"],
        b29: reg = ["b[29]", "db[29]"],
        b30: reg = ["b[30]", "db[30]"],
        b31: reg = ["b[31]", "db[31]"],
        b32: reg = ["b[32]", "db[32]"],
        b33: reg = ["b[33]", "db[33]"],
        b34: reg = ["b[34]", "db[34]"],
        b35: reg = ["b[35]", "db[35]"],
        b36: reg = ["b[36]", "db[36]"],
        b37: reg = ["b[37]", "db[37]"],
        b38: reg = ["b[38]", "db[38]"],
        b39: reg = ["b[39]", "db[39]"],
        b40: reg = ["b[40]", "db[40]"],
        b41: reg = ["b[41]", "db[41]"],
        b42: reg = ["b[42]", "db[42]"],
        b43: reg = ["b[43]", "db[43]"],
        b44: reg = ["b[44]", "db[44]"],
        b45: reg = ["b[45]", "db[45]"],
        b46: reg = ["b[46]", "db[46]"],
        b47: reg = ["b[47]", "db[47]"],
        b48: reg = ["b[48]", "db[48]"],
        b49: reg = ["b[49]", "db[49]"],
        b50: reg = ["b[50]", "db[50]"],
        b51: reg = ["b[51]", "db[51]"],
        b52: reg = ["b[52]", "db[52]"],
        b53: reg = ["b[53]", "db[53]"],
        b54: reg = ["b[54]", "db[54]"],
        b55: reg = ["b[55]", "db[55]"],
        b56: reg = ["b[56]", "db[56]"],
        b57: reg = ["b[57]", "db[57]"],
        b58: reg = ["b[58]", "db[58]"],
        b59: reg = ["b[59]", "db[59]"],
        b60: reg = ["b[60]", "db[60]"],
        b61: reg = ["b[61]", "db[61]"],
        b62: reg = ["b[62]", "db[62]"],
        b63: reg = ["b[63]", "db[63]"],

        g0: reg = ["g0", "dg0"],
        g1: reg = ["g1", "dg1"],
        g2: reg = ["g2", "dg2"],
        g3: reg = ["g3", "dg3"],
        g4: reg = ["g4", "dg4"],
        g5: reg = ["g5", "dg5"],
        g6: reg = ["g6", "dg6"],
        g7: reg = ["g7", "dg7"],
        g8: reg = ["g8", "dg8"],
        g9: reg = ["g9", "dg9"],
        g10: reg = ["g10", "dg10"],
        g11: reg = ["g11", "dg11"],
        g12: reg = ["g12", "dg12"],
        g13: reg = ["g13", "dg13"],
        g14: reg = ["g14", "dg14"],
        g15: reg = ["g15", "dg15"],
        g16: reg = ["g16", "dg16"],
        g17: reg = ["g17", "dg17"],
        g18: reg = ["g18", "dg18"],
        g19: reg = ["g19", "dg19"],
        g20: reg = ["g20", "dg20"],
        g21: reg = ["g21", "dg21"],
        g22: reg = ["g22", "dg22"],
        g23: reg = ["g23", "dg23"],
        g24: reg = ["g24", "dg24"],
        g25: reg = ["g25", "dg25"],
        g26: reg = ["g26", "dg26"],
        g27: reg = ["g27", "dg27"],
        g28: reg = ["g28", "dg28"],
        g29: reg = ["g29", "dg29"],
        g30: reg = ["g30", "dg30"],
        g31: reg = ["g31", "dg31"],

        pred0: reg = ["pred0"],
        pred1: reg = ["pred1"],
        pred2: reg = ["pred2"],
        pred3: reg = ["pred3"],
        pred4: reg = ["pred4"],
        pred5: reg = ["pred5"],
        pred6: reg = ["pred6"],
        pred7: reg = ["pred7"],
        pred8: reg = ["pred8"],
        pred9: reg = ["pred9"],
        pred10: reg = ["pred10"],
        pred11: reg = ["pred11"],
        pred12: reg = ["pred12"],
        pred13: reg = ["pred13"],
        pred14: reg = ["pred14"],
        pred15: reg = ["pred15"],
        pred16: reg = ["pred16"],
        pred17: reg = ["pred17"],
        pred18: reg = ["pred18"],
        pred19: reg = ["pred19"],
        pred20: reg = ["pred20"],
        pred21: reg = ["pred21"],
        pred22: reg = ["pred22"],
        pred23: reg = ["pred23"],
        pred24: reg = ["pred24"],
        pred25: reg = ["pred25"],
        pred26: reg = ["pred26"],
        pred27: reg = ["pred27"],
        pred28: reg = ["pred28"],
        pred29: reg = ["pred29"],
        pred30: reg = ["pred30"],
        pred31: reg = ["pred31"],

        ctpr1: reg = ["ctpr1"],
        ctpr2: reg = ["ctpr2"],
        ctpr3: reg = ["ctpr3"],
    }
}

impl E2kInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
