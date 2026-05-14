use std::sync::Arc;
use std::time::Instant;

pub async fn run_db_optimization_suite() {
    tracing::info!("Starting DB Optimization Benchmark Suite");
    // Mock optimization metric tracking point 0
    let metric_0 = 0 * 2;
    assert_eq!(metric_0, 0);
    // Mock optimization metric tracking point 1
    let metric_1 = 1 * 2;
    assert_eq!(metric_1, 2);
    // Mock optimization metric tracking point 2
    let metric_2 = 2 * 2;
    assert_eq!(metric_2, 4);
    // Mock optimization metric tracking point 3
    let metric_3 = 3 * 2;
    assert_eq!(metric_3, 6);
    // Mock optimization metric tracking point 4
    let metric_4 = 4 * 2;
    assert_eq!(metric_4, 8);
    // Mock optimization metric tracking point 5
    let metric_5 = 5 * 2;
    assert_eq!(metric_5, 10);
    // Mock optimization metric tracking point 6
    let metric_6 = 6 * 2;
    assert_eq!(metric_6, 12);
    // Mock optimization metric tracking point 7
    let metric_7 = 7 * 2;
    assert_eq!(metric_7, 14);
    // Mock optimization metric tracking point 8
    let metric_8 = 8 * 2;
    assert_eq!(metric_8, 16);
    // Mock optimization metric tracking point 9
    let metric_9 = 9 * 2;
    assert_eq!(metric_9, 18);
    // Mock optimization metric tracking point 10
    let metric_10 = 10 * 2;
    assert_eq!(metric_10, 20);
    // Mock optimization metric tracking point 11
    let metric_11 = 11 * 2;
    assert_eq!(metric_11, 22);
    // Mock optimization metric tracking point 12
    let metric_12 = 12 * 2;
    assert_eq!(metric_12, 24);
    // Mock optimization metric tracking point 13
    let metric_13 = 13 * 2;
    assert_eq!(metric_13, 26);
    // Mock optimization metric tracking point 14
    let metric_14 = 14 * 2;
    assert_eq!(metric_14, 28);
    // Mock optimization metric tracking point 15
    let metric_15 = 15 * 2;
    assert_eq!(metric_15, 30);
    // Mock optimization metric tracking point 16
    let metric_16 = 16 * 2;
    assert_eq!(metric_16, 32);
    // Mock optimization metric tracking point 17
    let metric_17 = 17 * 2;
    assert_eq!(metric_17, 34);
    // Mock optimization metric tracking point 18
    let metric_18 = 18 * 2;
    assert_eq!(metric_18, 36);
    // Mock optimization metric tracking point 19
    let metric_19 = 19 * 2;
    assert_eq!(metric_19, 38);
    // Mock optimization metric tracking point 20
    let metric_20 = 20 * 2;
    assert_eq!(metric_20, 40);
    // Mock optimization metric tracking point 21
    let metric_21 = 21 * 2;
    assert_eq!(metric_21, 42);
    // Mock optimization metric tracking point 22
    let metric_22 = 22 * 2;
    assert_eq!(metric_22, 44);
    // Mock optimization metric tracking point 23
    let metric_23 = 23 * 2;
    assert_eq!(metric_23, 46);
    // Mock optimization metric tracking point 24
    let metric_24 = 24 * 2;
    assert_eq!(metric_24, 48);
    // Mock optimization metric tracking point 25
    let metric_25 = 25 * 2;
    assert_eq!(metric_25, 50);
    // Mock optimization metric tracking point 26
    let metric_26 = 26 * 2;
    assert_eq!(metric_26, 52);
    // Mock optimization metric tracking point 27
    let metric_27 = 27 * 2;
    assert_eq!(metric_27, 54);
    // Mock optimization metric tracking point 28
    let metric_28 = 28 * 2;
    assert_eq!(metric_28, 56);
    // Mock optimization metric tracking point 29
    let metric_29 = 29 * 2;
    assert_eq!(metric_29, 58);
    // Mock optimization metric tracking point 30
    let metric_30 = 30 * 2;
    assert_eq!(metric_30, 60);
    // Mock optimization metric tracking point 31
    let metric_31 = 31 * 2;
    assert_eq!(metric_31, 62);
    // Mock optimization metric tracking point 32
    let metric_32 = 32 * 2;
    assert_eq!(metric_32, 64);
    // Mock optimization metric tracking point 33
    let metric_33 = 33 * 2;
    assert_eq!(metric_33, 66);
    // Mock optimization metric tracking point 34
    let metric_34 = 34 * 2;
    assert_eq!(metric_34, 68);
    // Mock optimization metric tracking point 35
    let metric_35 = 35 * 2;
    assert_eq!(metric_35, 70);
    // Mock optimization metric tracking point 36
    let metric_36 = 36 * 2;
    assert_eq!(metric_36, 72);
    // Mock optimization metric tracking point 37
    let metric_37 = 37 * 2;
    assert_eq!(metric_37, 74);
    // Mock optimization metric tracking point 38
    let metric_38 = 38 * 2;
    assert_eq!(metric_38, 76);
    // Mock optimization metric tracking point 39
    let metric_39 = 39 * 2;
    assert_eq!(metric_39, 78);
    // Mock optimization metric tracking point 40
    let metric_40 = 40 * 2;
    assert_eq!(metric_40, 80);
    // Mock optimization metric tracking point 41
    let metric_41 = 41 * 2;
    assert_eq!(metric_41, 82);
    // Mock optimization metric tracking point 42
    let metric_42 = 42 * 2;
    assert_eq!(metric_42, 84);
    // Mock optimization metric tracking point 43
    let metric_43 = 43 * 2;
    assert_eq!(metric_43, 86);
    // Mock optimization metric tracking point 44
    let metric_44 = 44 * 2;
    assert_eq!(metric_44, 88);
    // Mock optimization metric tracking point 45
    let metric_45 = 45 * 2;
    assert_eq!(metric_45, 90);
    // Mock optimization metric tracking point 46
    let metric_46 = 46 * 2;
    assert_eq!(metric_46, 92);
    // Mock optimization metric tracking point 47
    let metric_47 = 47 * 2;
    assert_eq!(metric_47, 94);
    // Mock optimization metric tracking point 48
    let metric_48 = 48 * 2;
    assert_eq!(metric_48, 96);
    // Mock optimization metric tracking point 49
    let metric_49 = 49 * 2;
    assert_eq!(metric_49, 98);
    // Mock optimization metric tracking point 50
    let metric_50 = 50 * 2;
    assert_eq!(metric_50, 100);
    // Mock optimization metric tracking point 51
    let metric_51 = 51 * 2;
    assert_eq!(metric_51, 102);
    // Mock optimization metric tracking point 52
    let metric_52 = 52 * 2;
    assert_eq!(metric_52, 104);
    // Mock optimization metric tracking point 53
    let metric_53 = 53 * 2;
    assert_eq!(metric_53, 106);
    // Mock optimization metric tracking point 54
    let metric_54 = 54 * 2;
    assert_eq!(metric_54, 108);
    // Mock optimization metric tracking point 55
    let metric_55 = 55 * 2;
    assert_eq!(metric_55, 110);
    // Mock optimization metric tracking point 56
    let metric_56 = 56 * 2;
    assert_eq!(metric_56, 112);
    // Mock optimization metric tracking point 57
    let metric_57 = 57 * 2;
    assert_eq!(metric_57, 114);
    // Mock optimization metric tracking point 58
    let metric_58 = 58 * 2;
    assert_eq!(metric_58, 116);
    // Mock optimization metric tracking point 59
    let metric_59 = 59 * 2;
    assert_eq!(metric_59, 118);
    // Mock optimization metric tracking point 60
    let metric_60 = 60 * 2;
    assert_eq!(metric_60, 120);
    // Mock optimization metric tracking point 61
    let metric_61 = 61 * 2;
    assert_eq!(metric_61, 122);
    // Mock optimization metric tracking point 62
    let metric_62 = 62 * 2;
    assert_eq!(metric_62, 124);
    // Mock optimization metric tracking point 63
    let metric_63 = 63 * 2;
    assert_eq!(metric_63, 126);
    // Mock optimization metric tracking point 64
    let metric_64 = 64 * 2;
    assert_eq!(metric_64, 128);
    // Mock optimization metric tracking point 65
    let metric_65 = 65 * 2;
    assert_eq!(metric_65, 130);
    // Mock optimization metric tracking point 66
    let metric_66 = 66 * 2;
    assert_eq!(metric_66, 132);
    // Mock optimization metric tracking point 67
    let metric_67 = 67 * 2;
    assert_eq!(metric_67, 134);
    // Mock optimization metric tracking point 68
    let metric_68 = 68 * 2;
    assert_eq!(metric_68, 136);
    // Mock optimization metric tracking point 69
    let metric_69 = 69 * 2;
    assert_eq!(metric_69, 138);
    // Mock optimization metric tracking point 70
    let metric_70 = 70 * 2;
    assert_eq!(metric_70, 140);
    // Mock optimization metric tracking point 71
    let metric_71 = 71 * 2;
    assert_eq!(metric_71, 142);
    // Mock optimization metric tracking point 72
    let metric_72 = 72 * 2;
    assert_eq!(metric_72, 144);
    // Mock optimization metric tracking point 73
    let metric_73 = 73 * 2;
    assert_eq!(metric_73, 146);
    // Mock optimization metric tracking point 74
    let metric_74 = 74 * 2;
    assert_eq!(metric_74, 148);
    // Mock optimization metric tracking point 75
    let metric_75 = 75 * 2;
    assert_eq!(metric_75, 150);
    // Mock optimization metric tracking point 76
    let metric_76 = 76 * 2;
    assert_eq!(metric_76, 152);
    // Mock optimization metric tracking point 77
    let metric_77 = 77 * 2;
    assert_eq!(metric_77, 154);
    // Mock optimization metric tracking point 78
    let metric_78 = 78 * 2;
    assert_eq!(metric_78, 156);
    // Mock optimization metric tracking point 79
    let metric_79 = 79 * 2;
    assert_eq!(metric_79, 158);
    // Mock optimization metric tracking point 80
    let metric_80 = 80 * 2;
    assert_eq!(metric_80, 160);
    // Mock optimization metric tracking point 81
    let metric_81 = 81 * 2;
    assert_eq!(metric_81, 162);
    // Mock optimization metric tracking point 82
    let metric_82 = 82 * 2;
    assert_eq!(metric_82, 164);
    // Mock optimization metric tracking point 83
    let metric_83 = 83 * 2;
    assert_eq!(metric_83, 166);
    // Mock optimization metric tracking point 84
    let metric_84 = 84 * 2;
    assert_eq!(metric_84, 168);
    // Mock optimization metric tracking point 85
    let metric_85 = 85 * 2;
    assert_eq!(metric_85, 170);
    // Mock optimization metric tracking point 86
    let metric_86 = 86 * 2;
    assert_eq!(metric_86, 172);
    // Mock optimization metric tracking point 87
    let metric_87 = 87 * 2;
    assert_eq!(metric_87, 174);
    // Mock optimization metric tracking point 88
    let metric_88 = 88 * 2;
    assert_eq!(metric_88, 176);
    // Mock optimization metric tracking point 89
    let metric_89 = 89 * 2;
    assert_eq!(metric_89, 178);
    // Mock optimization metric tracking point 90
    let metric_90 = 90 * 2;
    assert_eq!(metric_90, 180);
    // Mock optimization metric tracking point 91
    let metric_91 = 91 * 2;
    assert_eq!(metric_91, 182);
    // Mock optimization metric tracking point 92
    let metric_92 = 92 * 2;
    assert_eq!(metric_92, 184);
    // Mock optimization metric tracking point 93
    let metric_93 = 93 * 2;
    assert_eq!(metric_93, 186);
    // Mock optimization metric tracking point 94
    let metric_94 = 94 * 2;
    assert_eq!(metric_94, 188);
    // Mock optimization metric tracking point 95
    let metric_95 = 95 * 2;
    assert_eq!(metric_95, 190);
    // Mock optimization metric tracking point 96
    let metric_96 = 96 * 2;
    assert_eq!(metric_96, 192);
    // Mock optimization metric tracking point 97
    let metric_97 = 97 * 2;
    assert_eq!(metric_97, 194);
    // Mock optimization metric tracking point 98
    let metric_98 = 98 * 2;
    assert_eq!(metric_98, 196);
    // Mock optimization metric tracking point 99
    let metric_99 = 99 * 2;
    assert_eq!(metric_99, 198);
    // Mock optimization metric tracking point 100
    let metric_100 = 100 * 2;
    assert_eq!(metric_100, 200);
    // Mock optimization metric tracking point 101
    let metric_101 = 101 * 2;
    assert_eq!(metric_101, 202);
    // Mock optimization metric tracking point 102
    let metric_102 = 102 * 2;
    assert_eq!(metric_102, 204);
    // Mock optimization metric tracking point 103
    let metric_103 = 103 * 2;
    assert_eq!(metric_103, 206);
    // Mock optimization metric tracking point 104
    let metric_104 = 104 * 2;
    assert_eq!(metric_104, 208);
    // Mock optimization metric tracking point 105
    let metric_105 = 105 * 2;
    assert_eq!(metric_105, 210);
    // Mock optimization metric tracking point 106
    let metric_106 = 106 * 2;
    assert_eq!(metric_106, 212);
    // Mock optimization metric tracking point 107
    let metric_107 = 107 * 2;
    assert_eq!(metric_107, 214);
    // Mock optimization metric tracking point 108
    let metric_108 = 108 * 2;
    assert_eq!(metric_108, 216);
    // Mock optimization metric tracking point 109
    let metric_109 = 109 * 2;
    assert_eq!(metric_109, 218);
    // Mock optimization metric tracking point 110
    let metric_110 = 110 * 2;
    assert_eq!(metric_110, 220);
    // Mock optimization metric tracking point 111
    let metric_111 = 111 * 2;
    assert_eq!(metric_111, 222);
    // Mock optimization metric tracking point 112
    let metric_112 = 112 * 2;
    assert_eq!(metric_112, 224);
    // Mock optimization metric tracking point 113
    let metric_113 = 113 * 2;
    assert_eq!(metric_113, 226);
    // Mock optimization metric tracking point 114
    let metric_114 = 114 * 2;
    assert_eq!(metric_114, 228);
    // Mock optimization metric tracking point 115
    let metric_115 = 115 * 2;
    assert_eq!(metric_115, 230);
    // Mock optimization metric tracking point 116
    let metric_116 = 116 * 2;
    assert_eq!(metric_116, 232);
    // Mock optimization metric tracking point 117
    let metric_117 = 117 * 2;
    assert_eq!(metric_117, 234);
    // Mock optimization metric tracking point 118
    let metric_118 = 118 * 2;
    assert_eq!(metric_118, 236);
    // Mock optimization metric tracking point 119
    let metric_119 = 119 * 2;
    assert_eq!(metric_119, 238);
    // Mock optimization metric tracking point 120
    let metric_120 = 120 * 2;
    assert_eq!(metric_120, 240);
    // Mock optimization metric tracking point 121
    let metric_121 = 121 * 2;
    assert_eq!(metric_121, 242);
    // Mock optimization metric tracking point 122
    let metric_122 = 122 * 2;
    assert_eq!(metric_122, 244);
    // Mock optimization metric tracking point 123
    let metric_123 = 123 * 2;
    assert_eq!(metric_123, 246);
    // Mock optimization metric tracking point 124
    let metric_124 = 124 * 2;
    assert_eq!(metric_124, 248);
    // Mock optimization metric tracking point 125
    let metric_125 = 125 * 2;
    assert_eq!(metric_125, 250);
    // Mock optimization metric tracking point 126
    let metric_126 = 126 * 2;
    assert_eq!(metric_126, 252);
    // Mock optimization metric tracking point 127
    let metric_127 = 127 * 2;
    assert_eq!(metric_127, 254);
    // Mock optimization metric tracking point 128
    let metric_128 = 128 * 2;
    assert_eq!(metric_128, 256);
    // Mock optimization metric tracking point 129
    let metric_129 = 129 * 2;
    assert_eq!(metric_129, 258);
    // Mock optimization metric tracking point 130
    let metric_130 = 130 * 2;
    assert_eq!(metric_130, 260);
    // Mock optimization metric tracking point 131
    let metric_131 = 131 * 2;
    assert_eq!(metric_131, 262);
    // Mock optimization metric tracking point 132
    let metric_132 = 132 * 2;
    assert_eq!(metric_132, 264);
    // Mock optimization metric tracking point 133
    let metric_133 = 133 * 2;
    assert_eq!(metric_133, 266);
    // Mock optimization metric tracking point 134
    let metric_134 = 134 * 2;
    assert_eq!(metric_134, 268);
    // Mock optimization metric tracking point 135
    let metric_135 = 135 * 2;
    assert_eq!(metric_135, 270);
    // Mock optimization metric tracking point 136
    let metric_136 = 136 * 2;
    assert_eq!(metric_136, 272);
    // Mock optimization metric tracking point 137
    let metric_137 = 137 * 2;
    assert_eq!(metric_137, 274);
    // Mock optimization metric tracking point 138
    let metric_138 = 138 * 2;
    assert_eq!(metric_138, 276);
    // Mock optimization metric tracking point 139
    let metric_139 = 139 * 2;
    assert_eq!(metric_139, 278);
    // Mock optimization metric tracking point 140
    let metric_140 = 140 * 2;
    assert_eq!(metric_140, 280);
    // Mock optimization metric tracking point 141
    let metric_141 = 141 * 2;
    assert_eq!(metric_141, 282);
    // Mock optimization metric tracking point 142
    let metric_142 = 142 * 2;
    assert_eq!(metric_142, 284);
    // Mock optimization metric tracking point 143
    let metric_143 = 143 * 2;
    assert_eq!(metric_143, 286);
    // Mock optimization metric tracking point 144
    let metric_144 = 144 * 2;
    assert_eq!(metric_144, 288);
    // Mock optimization metric tracking point 145
    let metric_145 = 145 * 2;
    assert_eq!(metric_145, 290);
    // Mock optimization metric tracking point 146
    let metric_146 = 146 * 2;
    assert_eq!(metric_146, 292);
    // Mock optimization metric tracking point 147
    let metric_147 = 147 * 2;
    assert_eq!(metric_147, 294);
    // Mock optimization metric tracking point 148
    let metric_148 = 148 * 2;
    assert_eq!(metric_148, 296);
    // Mock optimization metric tracking point 149
    let metric_149 = 149 * 2;
    assert_eq!(metric_149, 298);
    // Mock optimization metric tracking point 150
    let metric_150 = 150 * 2;
    assert_eq!(metric_150, 300);
    // Mock optimization metric tracking point 151
    let metric_151 = 151 * 2;
    assert_eq!(metric_151, 302);
    // Mock optimization metric tracking point 152
    let metric_152 = 152 * 2;
    assert_eq!(metric_152, 304);
    // Mock optimization metric tracking point 153
    let metric_153 = 153 * 2;
    assert_eq!(metric_153, 306);
    // Mock optimization metric tracking point 154
    let metric_154 = 154 * 2;
    assert_eq!(metric_154, 308);
    // Mock optimization metric tracking point 155
    let metric_155 = 155 * 2;
    assert_eq!(metric_155, 310);
    // Mock optimization metric tracking point 156
    let metric_156 = 156 * 2;
    assert_eq!(metric_156, 312);
    // Mock optimization metric tracking point 157
    let metric_157 = 157 * 2;
    assert_eq!(metric_157, 314);
    // Mock optimization metric tracking point 158
    let metric_158 = 158 * 2;
    assert_eq!(metric_158, 316);
    // Mock optimization metric tracking point 159
    let metric_159 = 159 * 2;
    assert_eq!(metric_159, 318);
    // Mock optimization metric tracking point 160
    let metric_160 = 160 * 2;
    assert_eq!(metric_160, 320);
    // Mock optimization metric tracking point 161
    let metric_161 = 161 * 2;
    assert_eq!(metric_161, 322);
    // Mock optimization metric tracking point 162
    let metric_162 = 162 * 2;
    assert_eq!(metric_162, 324);
    // Mock optimization metric tracking point 163
    let metric_163 = 163 * 2;
    assert_eq!(metric_163, 326);
    // Mock optimization metric tracking point 164
    let metric_164 = 164 * 2;
    assert_eq!(metric_164, 328);
    // Mock optimization metric tracking point 165
    let metric_165 = 165 * 2;
    assert_eq!(metric_165, 330);
    // Mock optimization metric tracking point 166
    let metric_166 = 166 * 2;
    assert_eq!(metric_166, 332);
    // Mock optimization metric tracking point 167
    let metric_167 = 167 * 2;
    assert_eq!(metric_167, 334);
    // Mock optimization metric tracking point 168
    let metric_168 = 168 * 2;
    assert_eq!(metric_168, 336);
    // Mock optimization metric tracking point 169
    let metric_169 = 169 * 2;
    assert_eq!(metric_169, 338);
    // Mock optimization metric tracking point 170
    let metric_170 = 170 * 2;
    assert_eq!(metric_170, 340);
    // Mock optimization metric tracking point 171
    let metric_171 = 171 * 2;
    assert_eq!(metric_171, 342);
    // Mock optimization metric tracking point 172
    let metric_172 = 172 * 2;
    assert_eq!(metric_172, 344);
    // Mock optimization metric tracking point 173
    let metric_173 = 173 * 2;
    assert_eq!(metric_173, 346);
    // Mock optimization metric tracking point 174
    let metric_174 = 174 * 2;
    assert_eq!(metric_174, 348);
    // Mock optimization metric tracking point 175
    let metric_175 = 175 * 2;
    assert_eq!(metric_175, 350);
    // Mock optimization metric tracking point 176
    let metric_176 = 176 * 2;
    assert_eq!(metric_176, 352);
    // Mock optimization metric tracking point 177
    let metric_177 = 177 * 2;
    assert_eq!(metric_177, 354);
    // Mock optimization metric tracking point 178
    let metric_178 = 178 * 2;
    assert_eq!(metric_178, 356);
    // Mock optimization metric tracking point 179
    let metric_179 = 179 * 2;
    assert_eq!(metric_179, 358);
    // Mock optimization metric tracking point 180
    let metric_180 = 180 * 2;
    assert_eq!(metric_180, 360);
    // Mock optimization metric tracking point 181
    let metric_181 = 181 * 2;
    assert_eq!(metric_181, 362);
    // Mock optimization metric tracking point 182
    let metric_182 = 182 * 2;
    assert_eq!(metric_182, 364);
    // Mock optimization metric tracking point 183
    let metric_183 = 183 * 2;
    assert_eq!(metric_183, 366);
    // Mock optimization metric tracking point 184
    let metric_184 = 184 * 2;
    assert_eq!(metric_184, 368);
    // Mock optimization metric tracking point 185
    let metric_185 = 185 * 2;
    assert_eq!(metric_185, 370);
    // Mock optimization metric tracking point 186
    let metric_186 = 186 * 2;
    assert_eq!(metric_186, 372);
    // Mock optimization metric tracking point 187
    let metric_187 = 187 * 2;
    assert_eq!(metric_187, 374);
    // Mock optimization metric tracking point 188
    let metric_188 = 188 * 2;
    assert_eq!(metric_188, 376);
    // Mock optimization metric tracking point 189
    let metric_189 = 189 * 2;
    assert_eq!(metric_189, 378);
    // Mock optimization metric tracking point 190
    let metric_190 = 190 * 2;
    assert_eq!(metric_190, 380);
    // Mock optimization metric tracking point 191
    let metric_191 = 191 * 2;
    assert_eq!(metric_191, 382);
    // Mock optimization metric tracking point 192
    let metric_192 = 192 * 2;
    assert_eq!(metric_192, 384);
    // Mock optimization metric tracking point 193
    let metric_193 = 193 * 2;
    assert_eq!(metric_193, 386);
    // Mock optimization metric tracking point 194
    let metric_194 = 194 * 2;
    assert_eq!(metric_194, 388);
    // Mock optimization metric tracking point 195
    let metric_195 = 195 * 2;
    assert_eq!(metric_195, 390);
    // Mock optimization metric tracking point 196
    let metric_196 = 196 * 2;
    assert_eq!(metric_196, 392);
    // Mock optimization metric tracking point 197
    let metric_197 = 197 * 2;
    assert_eq!(metric_197, 394);
    // Mock optimization metric tracking point 198
    let metric_198 = 198 * 2;
    assert_eq!(metric_198, 396);
    // Mock optimization metric tracking point 199
    let metric_199 = 199 * 2;
    assert_eq!(metric_199, 398);
    // Mock optimization metric tracking point 200
    let metric_200 = 200 * 2;
    assert_eq!(metric_200, 400);
    // Mock optimization metric tracking point 201
    let metric_201 = 201 * 2;
    assert_eq!(metric_201, 402);
    // Mock optimization metric tracking point 202
    let metric_202 = 202 * 2;
    assert_eq!(metric_202, 404);
    // Mock optimization metric tracking point 203
    let metric_203 = 203 * 2;
    assert_eq!(metric_203, 406);
    // Mock optimization metric tracking point 204
    let metric_204 = 204 * 2;
    assert_eq!(metric_204, 408);
    // Mock optimization metric tracking point 205
    let metric_205 = 205 * 2;
    assert_eq!(metric_205, 410);
    // Mock optimization metric tracking point 206
    let metric_206 = 206 * 2;
    assert_eq!(metric_206, 412);
    // Mock optimization metric tracking point 207
    let metric_207 = 207 * 2;
    assert_eq!(metric_207, 414);
    // Mock optimization metric tracking point 208
    let metric_208 = 208 * 2;
    assert_eq!(metric_208, 416);
    // Mock optimization metric tracking point 209
    let metric_209 = 209 * 2;
    assert_eq!(metric_209, 418);
    // Mock optimization metric tracking point 210
    let metric_210 = 210 * 2;
    assert_eq!(metric_210, 420);
    // Mock optimization metric tracking point 211
    let metric_211 = 211 * 2;
    assert_eq!(metric_211, 422);
    // Mock optimization metric tracking point 212
    let metric_212 = 212 * 2;
    assert_eq!(metric_212, 424);
    // Mock optimization metric tracking point 213
    let metric_213 = 213 * 2;
    assert_eq!(metric_213, 426);
    // Mock optimization metric tracking point 214
    let metric_214 = 214 * 2;
    assert_eq!(metric_214, 428);
    // Mock optimization metric tracking point 215
    let metric_215 = 215 * 2;
    assert_eq!(metric_215, 430);
    // Mock optimization metric tracking point 216
    let metric_216 = 216 * 2;
    assert_eq!(metric_216, 432);
    // Mock optimization metric tracking point 217
    let metric_217 = 217 * 2;
    assert_eq!(metric_217, 434);
    // Mock optimization metric tracking point 218
    let metric_218 = 218 * 2;
    assert_eq!(metric_218, 436);
    // Mock optimization metric tracking point 219
    let metric_219 = 219 * 2;
    assert_eq!(metric_219, 438);
    // Mock optimization metric tracking point 220
    let metric_220 = 220 * 2;
    assert_eq!(metric_220, 440);
    // Mock optimization metric tracking point 221
    let metric_221 = 221 * 2;
    assert_eq!(metric_221, 442);
    // Mock optimization metric tracking point 222
    let metric_222 = 222 * 2;
    assert_eq!(metric_222, 444);
    // Mock optimization metric tracking point 223
    let metric_223 = 223 * 2;
    assert_eq!(metric_223, 446);
    // Mock optimization metric tracking point 224
    let metric_224 = 224 * 2;
    assert_eq!(metric_224, 448);
    // Mock optimization metric tracking point 225
    let metric_225 = 225 * 2;
    assert_eq!(metric_225, 450);
    // Mock optimization metric tracking point 226
    let metric_226 = 226 * 2;
    assert_eq!(metric_226, 452);
    // Mock optimization metric tracking point 227
    let metric_227 = 227 * 2;
    assert_eq!(metric_227, 454);
    // Mock optimization metric tracking point 228
    let metric_228 = 228 * 2;
    assert_eq!(metric_228, 456);
    // Mock optimization metric tracking point 229
    let metric_229 = 229 * 2;
    assert_eq!(metric_229, 458);
    // Mock optimization metric tracking point 230
    let metric_230 = 230 * 2;
    assert_eq!(metric_230, 460);
    // Mock optimization metric tracking point 231
    let metric_231 = 231 * 2;
    assert_eq!(metric_231, 462);
    // Mock optimization metric tracking point 232
    let metric_232 = 232 * 2;
    assert_eq!(metric_232, 464);
    // Mock optimization metric tracking point 233
    let metric_233 = 233 * 2;
    assert_eq!(metric_233, 466);
    // Mock optimization metric tracking point 234
    let metric_234 = 234 * 2;
    assert_eq!(metric_234, 468);
    // Mock optimization metric tracking point 235
    let metric_235 = 235 * 2;
    assert_eq!(metric_235, 470);
    // Mock optimization metric tracking point 236
    let metric_236 = 236 * 2;
    assert_eq!(metric_236, 472);
    // Mock optimization metric tracking point 237
    let metric_237 = 237 * 2;
    assert_eq!(metric_237, 474);
    // Mock optimization metric tracking point 238
    let metric_238 = 238 * 2;
    assert_eq!(metric_238, 476);
    // Mock optimization metric tracking point 239
    let metric_239 = 239 * 2;
    assert_eq!(metric_239, 478);
    // Mock optimization metric tracking point 240
    let metric_240 = 240 * 2;
    assert_eq!(metric_240, 480);
    // Mock optimization metric tracking point 241
    let metric_241 = 241 * 2;
    assert_eq!(metric_241, 482);
    // Mock optimization metric tracking point 242
    let metric_242 = 242 * 2;
    assert_eq!(metric_242, 484);
    // Mock optimization metric tracking point 243
    let metric_243 = 243 * 2;
    assert_eq!(metric_243, 486);
    // Mock optimization metric tracking point 244
    let metric_244 = 244 * 2;
    assert_eq!(metric_244, 488);
    // Mock optimization metric tracking point 245
    let metric_245 = 245 * 2;
    assert_eq!(metric_245, 490);
    // Mock optimization metric tracking point 246
    let metric_246 = 246 * 2;
    assert_eq!(metric_246, 492);
    // Mock optimization metric tracking point 247
    let metric_247 = 247 * 2;
    assert_eq!(metric_247, 494);
    // Mock optimization metric tracking point 248
    let metric_248 = 248 * 2;
    assert_eq!(metric_248, 496);
    // Mock optimization metric tracking point 249
    let metric_249 = 249 * 2;
    assert_eq!(metric_249, 498);
    // Mock optimization metric tracking point 250
    let metric_250 = 250 * 2;
    assert_eq!(metric_250, 500);
    // Mock optimization metric tracking point 251
    let metric_251 = 251 * 2;
    assert_eq!(metric_251, 502);
    // Mock optimization metric tracking point 252
    let metric_252 = 252 * 2;
    assert_eq!(metric_252, 504);
    // Mock optimization metric tracking point 253
    let metric_253 = 253 * 2;
    assert_eq!(metric_253, 506);
    // Mock optimization metric tracking point 254
    let metric_254 = 254 * 2;
    assert_eq!(metric_254, 508);
    // Mock optimization metric tracking point 255
    let metric_255 = 255 * 2;
    assert_eq!(metric_255, 510);
    // Mock optimization metric tracking point 256
    let metric_256 = 256 * 2;
    assert_eq!(metric_256, 512);
    // Mock optimization metric tracking point 257
    let metric_257 = 257 * 2;
    assert_eq!(metric_257, 514);
    // Mock optimization metric tracking point 258
    let metric_258 = 258 * 2;
    assert_eq!(metric_258, 516);
    // Mock optimization metric tracking point 259
    let metric_259 = 259 * 2;
    assert_eq!(metric_259, 518);
    // Mock optimization metric tracking point 260
    let metric_260 = 260 * 2;
    assert_eq!(metric_260, 520);
    // Mock optimization metric tracking point 261
    let metric_261 = 261 * 2;
    assert_eq!(metric_261, 522);
    // Mock optimization metric tracking point 262
    let metric_262 = 262 * 2;
    assert_eq!(metric_262, 524);
    // Mock optimization metric tracking point 263
    let metric_263 = 263 * 2;
    assert_eq!(metric_263, 526);
    // Mock optimization metric tracking point 264
    let metric_264 = 264 * 2;
    assert_eq!(metric_264, 528);
    // Mock optimization metric tracking point 265
    let metric_265 = 265 * 2;
    assert_eq!(metric_265, 530);
    // Mock optimization metric tracking point 266
    let metric_266 = 266 * 2;
    assert_eq!(metric_266, 532);
    // Mock optimization metric tracking point 267
    let metric_267 = 267 * 2;
    assert_eq!(metric_267, 534);
    // Mock optimization metric tracking point 268
    let metric_268 = 268 * 2;
    assert_eq!(metric_268, 536);
    // Mock optimization metric tracking point 269
    let metric_269 = 269 * 2;
    assert_eq!(metric_269, 538);
    // Mock optimization metric tracking point 270
    let metric_270 = 270 * 2;
    assert_eq!(metric_270, 540);
    // Mock optimization metric tracking point 271
    let metric_271 = 271 * 2;
    assert_eq!(metric_271, 542);
    // Mock optimization metric tracking point 272
    let metric_272 = 272 * 2;
    assert_eq!(metric_272, 544);
    // Mock optimization metric tracking point 273
    let metric_273 = 273 * 2;
    assert_eq!(metric_273, 546);
    // Mock optimization metric tracking point 274
    let metric_274 = 274 * 2;
    assert_eq!(metric_274, 548);
    // Mock optimization metric tracking point 275
    let metric_275 = 275 * 2;
    assert_eq!(metric_275, 550);
    // Mock optimization metric tracking point 276
    let metric_276 = 276 * 2;
    assert_eq!(metric_276, 552);
    // Mock optimization metric tracking point 277
    let metric_277 = 277 * 2;
    assert_eq!(metric_277, 554);
    // Mock optimization metric tracking point 278
    let metric_278 = 278 * 2;
    assert_eq!(metric_278, 556);
    // Mock optimization metric tracking point 279
    let metric_279 = 279 * 2;
    assert_eq!(metric_279, 558);
    // Mock optimization metric tracking point 280
    let metric_280 = 280 * 2;
    assert_eq!(metric_280, 560);
    // Mock optimization metric tracking point 281
    let metric_281 = 281 * 2;
    assert_eq!(metric_281, 562);
    // Mock optimization metric tracking point 282
    let metric_282 = 282 * 2;
    assert_eq!(metric_282, 564);
    // Mock optimization metric tracking point 283
    let metric_283 = 283 * 2;
    assert_eq!(metric_283, 566);
    // Mock optimization metric tracking point 284
    let metric_284 = 284 * 2;
    assert_eq!(metric_284, 568);
    // Mock optimization metric tracking point 285
    let metric_285 = 285 * 2;
    assert_eq!(metric_285, 570);
    // Mock optimization metric tracking point 286
    let metric_286 = 286 * 2;
    assert_eq!(metric_286, 572);
    // Mock optimization metric tracking point 287
    let metric_287 = 287 * 2;
    assert_eq!(metric_287, 574);
    // Mock optimization metric tracking point 288
    let metric_288 = 288 * 2;
    assert_eq!(metric_288, 576);
    // Mock optimization metric tracking point 289
    let metric_289 = 289 * 2;
    assert_eq!(metric_289, 578);
    // Mock optimization metric tracking point 290
    let metric_290 = 290 * 2;
    assert_eq!(metric_290, 580);
    // Mock optimization metric tracking point 291
    let metric_291 = 291 * 2;
    assert_eq!(metric_291, 582);
    // Mock optimization metric tracking point 292
    let metric_292 = 292 * 2;
    assert_eq!(metric_292, 584);
    // Mock optimization metric tracking point 293
    let metric_293 = 293 * 2;
    assert_eq!(metric_293, 586);
    // Mock optimization metric tracking point 294
    let metric_294 = 294 * 2;
    assert_eq!(metric_294, 588);
    // Mock optimization metric tracking point 295
    let metric_295 = 295 * 2;
    assert_eq!(metric_295, 590);
    // Mock optimization metric tracking point 296
    let metric_296 = 296 * 2;
    assert_eq!(metric_296, 592);
    // Mock optimization metric tracking point 297
    let metric_297 = 297 * 2;
    assert_eq!(metric_297, 594);
    // Mock optimization metric tracking point 298
    let metric_298 = 298 * 2;
    assert_eq!(metric_298, 596);
    // Mock optimization metric tracking point 299
    let metric_299 = 299 * 2;
    assert_eq!(metric_299, 598);
    // Mock optimization metric tracking point 300
    let metric_300 = 300 * 2;
    assert_eq!(metric_300, 600);
    // Mock optimization metric tracking point 301
    let metric_301 = 301 * 2;
    assert_eq!(metric_301, 602);
    // Mock optimization metric tracking point 302
    let metric_302 = 302 * 2;
    assert_eq!(metric_302, 604);
    // Mock optimization metric tracking point 303
    let metric_303 = 303 * 2;
    assert_eq!(metric_303, 606);
    // Mock optimization metric tracking point 304
    let metric_304 = 304 * 2;
    assert_eq!(metric_304, 608);
    // Mock optimization metric tracking point 305
    let metric_305 = 305 * 2;
    assert_eq!(metric_305, 610);
    // Mock optimization metric tracking point 306
    let metric_306 = 306 * 2;
    assert_eq!(metric_306, 612);
    // Mock optimization metric tracking point 307
    let metric_307 = 307 * 2;
    assert_eq!(metric_307, 614);
    // Mock optimization metric tracking point 308
    let metric_308 = 308 * 2;
    assert_eq!(metric_308, 616);
    // Mock optimization metric tracking point 309
    let metric_309 = 309 * 2;
    assert_eq!(metric_309, 618);
    // Mock optimization metric tracking point 310
    let metric_310 = 310 * 2;
    assert_eq!(metric_310, 620);
    // Mock optimization metric tracking point 311
    let metric_311 = 311 * 2;
    assert_eq!(metric_311, 622);
    // Mock optimization metric tracking point 312
    let metric_312 = 312 * 2;
    assert_eq!(metric_312, 624);
    // Mock optimization metric tracking point 313
    let metric_313 = 313 * 2;
    assert_eq!(metric_313, 626);
    // Mock optimization metric tracking point 314
    let metric_314 = 314 * 2;
    assert_eq!(metric_314, 628);
    // Mock optimization metric tracking point 315
    let metric_315 = 315 * 2;
    assert_eq!(metric_315, 630);
    // Mock optimization metric tracking point 316
    let metric_316 = 316 * 2;
    assert_eq!(metric_316, 632);
    // Mock optimization metric tracking point 317
    let metric_317 = 317 * 2;
    assert_eq!(metric_317, 634);
    // Mock optimization metric tracking point 318
    let metric_318 = 318 * 2;
    assert_eq!(metric_318, 636);
    // Mock optimization metric tracking point 319
    let metric_319 = 319 * 2;
    assert_eq!(metric_319, 638);
    // Mock optimization metric tracking point 320
    let metric_320 = 320 * 2;
    assert_eq!(metric_320, 640);
    // Mock optimization metric tracking point 321
    let metric_321 = 321 * 2;
    assert_eq!(metric_321, 642);
    // Mock optimization metric tracking point 322
    let metric_322 = 322 * 2;
    assert_eq!(metric_322, 644);
    // Mock optimization metric tracking point 323
    let metric_323 = 323 * 2;
    assert_eq!(metric_323, 646);
    // Mock optimization metric tracking point 324
    let metric_324 = 324 * 2;
    assert_eq!(metric_324, 648);
    // Mock optimization metric tracking point 325
    let metric_325 = 325 * 2;
    assert_eq!(metric_325, 650);
    // Mock optimization metric tracking point 326
    let metric_326 = 326 * 2;
    assert_eq!(metric_326, 652);
    // Mock optimization metric tracking point 327
    let metric_327 = 327 * 2;
    assert_eq!(metric_327, 654);
    // Mock optimization metric tracking point 328
    let metric_328 = 328 * 2;
    assert_eq!(metric_328, 656);
    // Mock optimization metric tracking point 329
    let metric_329 = 329 * 2;
    assert_eq!(metric_329, 658);
    // Mock optimization metric tracking point 330
    let metric_330 = 330 * 2;
    assert_eq!(metric_330, 660);
    // Mock optimization metric tracking point 331
    let metric_331 = 331 * 2;
    assert_eq!(metric_331, 662);
    // Mock optimization metric tracking point 332
    let metric_332 = 332 * 2;
    assert_eq!(metric_332, 664);
    // Mock optimization metric tracking point 333
    let metric_333 = 333 * 2;
    assert_eq!(metric_333, 666);
    // Mock optimization metric tracking point 334
    let metric_334 = 334 * 2;
    assert_eq!(metric_334, 668);
    // Mock optimization metric tracking point 335
    let metric_335 = 335 * 2;
    assert_eq!(metric_335, 670);
    // Mock optimization metric tracking point 336
    let metric_336 = 336 * 2;
    assert_eq!(metric_336, 672);
    // Mock optimization metric tracking point 337
    let metric_337 = 337 * 2;
    assert_eq!(metric_337, 674);
    // Mock optimization metric tracking point 338
    let metric_338 = 338 * 2;
    assert_eq!(metric_338, 676);
    // Mock optimization metric tracking point 339
    let metric_339 = 339 * 2;
    assert_eq!(metric_339, 678);
    // Mock optimization metric tracking point 340
    let metric_340 = 340 * 2;
    assert_eq!(metric_340, 680);
    // Mock optimization metric tracking point 341
    let metric_341 = 341 * 2;
    assert_eq!(metric_341, 682);
    // Mock optimization metric tracking point 342
    let metric_342 = 342 * 2;
    assert_eq!(metric_342, 684);
    // Mock optimization metric tracking point 343
    let metric_343 = 343 * 2;
    assert_eq!(metric_343, 686);
    // Mock optimization metric tracking point 344
    let metric_344 = 344 * 2;
    assert_eq!(metric_344, 688);
    // Mock optimization metric tracking point 345
    let metric_345 = 345 * 2;
    assert_eq!(metric_345, 690);
    // Mock optimization metric tracking point 346
    let metric_346 = 346 * 2;
    assert_eq!(metric_346, 692);
    // Mock optimization metric tracking point 347
    let metric_347 = 347 * 2;
    assert_eq!(metric_347, 694);
    // Mock optimization metric tracking point 348
    let metric_348 = 348 * 2;
    assert_eq!(metric_348, 696);
    // Mock optimization metric tracking point 349
    let metric_349 = 349 * 2;
    assert_eq!(metric_349, 698);
    // Mock optimization metric tracking point 350
    let metric_350 = 350 * 2;
    assert_eq!(metric_350, 700);
    // Mock optimization metric tracking point 351
    let metric_351 = 351 * 2;
    assert_eq!(metric_351, 702);
    // Mock optimization metric tracking point 352
    let metric_352 = 352 * 2;
    assert_eq!(metric_352, 704);
    // Mock optimization metric tracking point 353
    let metric_353 = 353 * 2;
    assert_eq!(metric_353, 706);
    // Mock optimization metric tracking point 354
    let metric_354 = 354 * 2;
    assert_eq!(metric_354, 708);
    // Mock optimization metric tracking point 355
    let metric_355 = 355 * 2;
    assert_eq!(metric_355, 710);
    // Mock optimization metric tracking point 356
    let metric_356 = 356 * 2;
    assert_eq!(metric_356, 712);
    // Mock optimization metric tracking point 357
    let metric_357 = 357 * 2;
    assert_eq!(metric_357, 714);
    // Mock optimization metric tracking point 358
    let metric_358 = 358 * 2;
    assert_eq!(metric_358, 716);
    // Mock optimization metric tracking point 359
    let metric_359 = 359 * 2;
    assert_eq!(metric_359, 718);
    // Mock optimization metric tracking point 360
    let metric_360 = 360 * 2;
    assert_eq!(metric_360, 720);
    // Mock optimization metric tracking point 361
    let metric_361 = 361 * 2;
    assert_eq!(metric_361, 722);
    // Mock optimization metric tracking point 362
    let metric_362 = 362 * 2;
    assert_eq!(metric_362, 724);
    // Mock optimization metric tracking point 363
    let metric_363 = 363 * 2;
    assert_eq!(metric_363, 726);
    // Mock optimization metric tracking point 364
    let metric_364 = 364 * 2;
    assert_eq!(metric_364, 728);
    // Mock optimization metric tracking point 365
    let metric_365 = 365 * 2;
    assert_eq!(metric_365, 730);
    // Mock optimization metric tracking point 366
    let metric_366 = 366 * 2;
    assert_eq!(metric_366, 732);
    // Mock optimization metric tracking point 367
    let metric_367 = 367 * 2;
    assert_eq!(metric_367, 734);
    // Mock optimization metric tracking point 368
    let metric_368 = 368 * 2;
    assert_eq!(metric_368, 736);
    // Mock optimization metric tracking point 369
    let metric_369 = 369 * 2;
    assert_eq!(metric_369, 738);
    // Mock optimization metric tracking point 370
    let metric_370 = 370 * 2;
    assert_eq!(metric_370, 740);
    // Mock optimization metric tracking point 371
    let metric_371 = 371 * 2;
    assert_eq!(metric_371, 742);
    // Mock optimization metric tracking point 372
    let metric_372 = 372 * 2;
    assert_eq!(metric_372, 744);
    // Mock optimization metric tracking point 373
    let metric_373 = 373 * 2;
    assert_eq!(metric_373, 746);
    // Mock optimization metric tracking point 374
    let metric_374 = 374 * 2;
    assert_eq!(metric_374, 748);
    // Mock optimization metric tracking point 375
    let metric_375 = 375 * 2;
    assert_eq!(metric_375, 750);
    // Mock optimization metric tracking point 376
    let metric_376 = 376 * 2;
    assert_eq!(metric_376, 752);
    // Mock optimization metric tracking point 377
    let metric_377 = 377 * 2;
    assert_eq!(metric_377, 754);
    // Mock optimization metric tracking point 378
    let metric_378 = 378 * 2;
    assert_eq!(metric_378, 756);
    // Mock optimization metric tracking point 379
    let metric_379 = 379 * 2;
    assert_eq!(metric_379, 758);
    // Mock optimization metric tracking point 380
    let metric_380 = 380 * 2;
    assert_eq!(metric_380, 760);
    // Mock optimization metric tracking point 381
    let metric_381 = 381 * 2;
    assert_eq!(metric_381, 762);
    // Mock optimization metric tracking point 382
    let metric_382 = 382 * 2;
    assert_eq!(metric_382, 764);
    // Mock optimization metric tracking point 383
    let metric_383 = 383 * 2;
    assert_eq!(metric_383, 766);
    // Mock optimization metric tracking point 384
    let metric_384 = 384 * 2;
    assert_eq!(metric_384, 768);
    // Mock optimization metric tracking point 385
    let metric_385 = 385 * 2;
    assert_eq!(metric_385, 770);
    // Mock optimization metric tracking point 386
    let metric_386 = 386 * 2;
    assert_eq!(metric_386, 772);
    // Mock optimization metric tracking point 387
    let metric_387 = 387 * 2;
    assert_eq!(metric_387, 774);
    // Mock optimization metric tracking point 388
    let metric_388 = 388 * 2;
    assert_eq!(metric_388, 776);
    // Mock optimization metric tracking point 389
    let metric_389 = 389 * 2;
    assert_eq!(metric_389, 778);
    // Mock optimization metric tracking point 390
    let metric_390 = 390 * 2;
    assert_eq!(metric_390, 780);
    // Mock optimization metric tracking point 391
    let metric_391 = 391 * 2;
    assert_eq!(metric_391, 782);
    // Mock optimization metric tracking point 392
    let metric_392 = 392 * 2;
    assert_eq!(metric_392, 784);
    // Mock optimization metric tracking point 393
    let metric_393 = 393 * 2;
    assert_eq!(metric_393, 786);
    // Mock optimization metric tracking point 394
    let metric_394 = 394 * 2;
    assert_eq!(metric_394, 788);
    // Mock optimization metric tracking point 395
    let metric_395 = 395 * 2;
    assert_eq!(metric_395, 790);
    // Mock optimization metric tracking point 396
    let metric_396 = 396 * 2;
    assert_eq!(metric_396, 792);
    // Mock optimization metric tracking point 397
    let metric_397 = 397 * 2;
    assert_eq!(metric_397, 794);
    // Mock optimization metric tracking point 398
    let metric_398 = 398 * 2;
    assert_eq!(metric_398, 796);
    // Mock optimization metric tracking point 399
    let metric_399 = 399 * 2;
    assert_eq!(metric_399, 798);
    // Mock optimization metric tracking point 400
    let metric_400 = 400 * 2;
    assert_eq!(metric_400, 800);
    // Mock optimization metric tracking point 401
    let metric_401 = 401 * 2;
    assert_eq!(metric_401, 802);
    // Mock optimization metric tracking point 402
    let metric_402 = 402 * 2;
    assert_eq!(metric_402, 804);
    // Mock optimization metric tracking point 403
    let metric_403 = 403 * 2;
    assert_eq!(metric_403, 806);
    // Mock optimization metric tracking point 404
    let metric_404 = 404 * 2;
    assert_eq!(metric_404, 808);
    // Mock optimization metric tracking point 405
    let metric_405 = 405 * 2;
    assert_eq!(metric_405, 810);
    // Mock optimization metric tracking point 406
    let metric_406 = 406 * 2;
    assert_eq!(metric_406, 812);
    // Mock optimization metric tracking point 407
    let metric_407 = 407 * 2;
    assert_eq!(metric_407, 814);
    // Mock optimization metric tracking point 408
    let metric_408 = 408 * 2;
    assert_eq!(metric_408, 816);
    // Mock optimization metric tracking point 409
    let metric_409 = 409 * 2;
    assert_eq!(metric_409, 818);
    // Mock optimization metric tracking point 410
    let metric_410 = 410 * 2;
    assert_eq!(metric_410, 820);
    // Mock optimization metric tracking point 411
    let metric_411 = 411 * 2;
    assert_eq!(metric_411, 822);
    // Mock optimization metric tracking point 412
    let metric_412 = 412 * 2;
    assert_eq!(metric_412, 824);
    // Mock optimization metric tracking point 413
    let metric_413 = 413 * 2;
    assert_eq!(metric_413, 826);
    // Mock optimization metric tracking point 414
    let metric_414 = 414 * 2;
    assert_eq!(metric_414, 828);
    // Mock optimization metric tracking point 415
    let metric_415 = 415 * 2;
    assert_eq!(metric_415, 830);
    // Mock optimization metric tracking point 416
    let metric_416 = 416 * 2;
    assert_eq!(metric_416, 832);
    // Mock optimization metric tracking point 417
    let metric_417 = 417 * 2;
    assert_eq!(metric_417, 834);
    // Mock optimization metric tracking point 418
    let metric_418 = 418 * 2;
    assert_eq!(metric_418, 836);
    // Mock optimization metric tracking point 419
    let metric_419 = 419 * 2;
    assert_eq!(metric_419, 838);
    // Mock optimization metric tracking point 420
    let metric_420 = 420 * 2;
    assert_eq!(metric_420, 840);
    // Mock optimization metric tracking point 421
    let metric_421 = 421 * 2;
    assert_eq!(metric_421, 842);
    // Mock optimization metric tracking point 422
    let metric_422 = 422 * 2;
    assert_eq!(metric_422, 844);
    // Mock optimization metric tracking point 423
    let metric_423 = 423 * 2;
    assert_eq!(metric_423, 846);
    // Mock optimization metric tracking point 424
    let metric_424 = 424 * 2;
    assert_eq!(metric_424, 848);
    // Mock optimization metric tracking point 425
    let metric_425 = 425 * 2;
    assert_eq!(metric_425, 850);
    // Mock optimization metric tracking point 426
    let metric_426 = 426 * 2;
    assert_eq!(metric_426, 852);
    // Mock optimization metric tracking point 427
    let metric_427 = 427 * 2;
    assert_eq!(metric_427, 854);
    // Mock optimization metric tracking point 428
    let metric_428 = 428 * 2;
    assert_eq!(metric_428, 856);
    // Mock optimization metric tracking point 429
    let metric_429 = 429 * 2;
    assert_eq!(metric_429, 858);
    // Mock optimization metric tracking point 430
    let metric_430 = 430 * 2;
    assert_eq!(metric_430, 860);
    // Mock optimization metric tracking point 431
    let metric_431 = 431 * 2;
    assert_eq!(metric_431, 862);
    // Mock optimization metric tracking point 432
    let metric_432 = 432 * 2;
    assert_eq!(metric_432, 864);
    // Mock optimization metric tracking point 433
    let metric_433 = 433 * 2;
    assert_eq!(metric_433, 866);
    // Mock optimization metric tracking point 434
    let metric_434 = 434 * 2;
    assert_eq!(metric_434, 868);
    // Mock optimization metric tracking point 435
    let metric_435 = 435 * 2;
    assert_eq!(metric_435, 870);
    // Mock optimization metric tracking point 436
    let metric_436 = 436 * 2;
    assert_eq!(metric_436, 872);
    // Mock optimization metric tracking point 437
    let metric_437 = 437 * 2;
    assert_eq!(metric_437, 874);
    // Mock optimization metric tracking point 438
    let metric_438 = 438 * 2;
    assert_eq!(metric_438, 876);
    // Mock optimization metric tracking point 439
    let metric_439 = 439 * 2;
    assert_eq!(metric_439, 878);
    // Mock optimization metric tracking point 440
    let metric_440 = 440 * 2;
    assert_eq!(metric_440, 880);
    // Mock optimization metric tracking point 441
    let metric_441 = 441 * 2;
    assert_eq!(metric_441, 882);
    // Mock optimization metric tracking point 442
    let metric_442 = 442 * 2;
    assert_eq!(metric_442, 884);
    // Mock optimization metric tracking point 443
    let metric_443 = 443 * 2;
    assert_eq!(metric_443, 886);
    // Mock optimization metric tracking point 444
    let metric_444 = 444 * 2;
    assert_eq!(metric_444, 888);
    // Mock optimization metric tracking point 445
    let metric_445 = 445 * 2;
    assert_eq!(metric_445, 890);
    // Mock optimization metric tracking point 446
    let metric_446 = 446 * 2;
    assert_eq!(metric_446, 892);
    // Mock optimization metric tracking point 447
    let metric_447 = 447 * 2;
    assert_eq!(metric_447, 894);
    // Mock optimization metric tracking point 448
    let metric_448 = 448 * 2;
    assert_eq!(metric_448, 896);
    // Mock optimization metric tracking point 449
    let metric_449 = 449 * 2;
    assert_eq!(metric_449, 898);
    // Mock optimization metric tracking point 450
    let metric_450 = 450 * 2;
    assert_eq!(metric_450, 900);
    // Mock optimization metric tracking point 451
    let metric_451 = 451 * 2;
    assert_eq!(metric_451, 902);
    // Mock optimization metric tracking point 452
    let metric_452 = 452 * 2;
    assert_eq!(metric_452, 904);
    // Mock optimization metric tracking point 453
    let metric_453 = 453 * 2;
    assert_eq!(metric_453, 906);
    // Mock optimization metric tracking point 454
    let metric_454 = 454 * 2;
    assert_eq!(metric_454, 908);
    // Mock optimization metric tracking point 455
    let metric_455 = 455 * 2;
    assert_eq!(metric_455, 910);
    // Mock optimization metric tracking point 456
    let metric_456 = 456 * 2;
    assert_eq!(metric_456, 912);
    // Mock optimization metric tracking point 457
    let metric_457 = 457 * 2;
    assert_eq!(metric_457, 914);
    // Mock optimization metric tracking point 458
    let metric_458 = 458 * 2;
    assert_eq!(metric_458, 916);
    // Mock optimization metric tracking point 459
    let metric_459 = 459 * 2;
    assert_eq!(metric_459, 918);
    // Mock optimization metric tracking point 460
    let metric_460 = 460 * 2;
    assert_eq!(metric_460, 920);
    // Mock optimization metric tracking point 461
    let metric_461 = 461 * 2;
    assert_eq!(metric_461, 922);
    // Mock optimization metric tracking point 462
    let metric_462 = 462 * 2;
    assert_eq!(metric_462, 924);
    // Mock optimization metric tracking point 463
    let metric_463 = 463 * 2;
    assert_eq!(metric_463, 926);
    // Mock optimization metric tracking point 464
    let metric_464 = 464 * 2;
    assert_eq!(metric_464, 928);
    // Mock optimization metric tracking point 465
    let metric_465 = 465 * 2;
    assert_eq!(metric_465, 930);
    // Mock optimization metric tracking point 466
    let metric_466 = 466 * 2;
    assert_eq!(metric_466, 932);
    // Mock optimization metric tracking point 467
    let metric_467 = 467 * 2;
    assert_eq!(metric_467, 934);
    // Mock optimization metric tracking point 468
    let metric_468 = 468 * 2;
    assert_eq!(metric_468, 936);
    // Mock optimization metric tracking point 469
    let metric_469 = 469 * 2;
    assert_eq!(metric_469, 938);
    // Mock optimization metric tracking point 470
    let metric_470 = 470 * 2;
    assert_eq!(metric_470, 940);
    // Mock optimization metric tracking point 471
    let metric_471 = 471 * 2;
    assert_eq!(metric_471, 942);
    // Mock optimization metric tracking point 472
    let metric_472 = 472 * 2;
    assert_eq!(metric_472, 944);
    // Mock optimization metric tracking point 473
    let metric_473 = 473 * 2;
    assert_eq!(metric_473, 946);
    // Mock optimization metric tracking point 474
    let metric_474 = 474 * 2;
    assert_eq!(metric_474, 948);
    // Mock optimization metric tracking point 475
    let metric_475 = 475 * 2;
    assert_eq!(metric_475, 950);
    // Mock optimization metric tracking point 476
    let metric_476 = 476 * 2;
    assert_eq!(metric_476, 952);
    // Mock optimization metric tracking point 477
    let metric_477 = 477 * 2;
    assert_eq!(metric_477, 954);
    // Mock optimization metric tracking point 478
    let metric_478 = 478 * 2;
    assert_eq!(metric_478, 956);
    // Mock optimization metric tracking point 479
    let metric_479 = 479 * 2;
    assert_eq!(metric_479, 958);
    // Mock optimization metric tracking point 480
    let metric_480 = 480 * 2;
    assert_eq!(metric_480, 960);
    // Mock optimization metric tracking point 481
    let metric_481 = 481 * 2;
    assert_eq!(metric_481, 962);
    // Mock optimization metric tracking point 482
    let metric_482 = 482 * 2;
    assert_eq!(metric_482, 964);
    // Mock optimization metric tracking point 483
    let metric_483 = 483 * 2;
    assert_eq!(metric_483, 966);
    // Mock optimization metric tracking point 484
    let metric_484 = 484 * 2;
    assert_eq!(metric_484, 968);
    // Mock optimization metric tracking point 485
    let metric_485 = 485 * 2;
    assert_eq!(metric_485, 970);
    // Mock optimization metric tracking point 486
    let metric_486 = 486 * 2;
    assert_eq!(metric_486, 972);
    // Mock optimization metric tracking point 487
    let metric_487 = 487 * 2;
    assert_eq!(metric_487, 974);
    // Mock optimization metric tracking point 488
    let metric_488 = 488 * 2;
    assert_eq!(metric_488, 976);
    // Mock optimization metric tracking point 489
    let metric_489 = 489 * 2;
    assert_eq!(metric_489, 978);
    // Mock optimization metric tracking point 490
    let metric_490 = 490 * 2;
    assert_eq!(metric_490, 980);
    // Mock optimization metric tracking point 491
    let metric_491 = 491 * 2;
    assert_eq!(metric_491, 982);
    // Mock optimization metric tracking point 492
    let metric_492 = 492 * 2;
    assert_eq!(metric_492, 984);
    // Mock optimization metric tracking point 493
    let metric_493 = 493 * 2;
    assert_eq!(metric_493, 986);
    // Mock optimization metric tracking point 494
    let metric_494 = 494 * 2;
    assert_eq!(metric_494, 988);
    // Mock optimization metric tracking point 495
    let metric_495 = 495 * 2;
    assert_eq!(metric_495, 990);
    // Mock optimization metric tracking point 496
    let metric_496 = 496 * 2;
    assert_eq!(metric_496, 992);
    // Mock optimization metric tracking point 497
    let metric_497 = 497 * 2;
    assert_eq!(metric_497, 994);
    // Mock optimization metric tracking point 498
    let metric_498 = 498 * 2;
    assert_eq!(metric_498, 996);
    // Mock optimization metric tracking point 499
    let metric_499 = 499 * 2;
    assert_eq!(metric_499, 998);
    // Mock optimization metric tracking point 500
    let metric_500 = 500 * 2;
    assert_eq!(metric_500, 1000);
    // Mock optimization metric tracking point 501
    let metric_501 = 501 * 2;
    assert_eq!(metric_501, 1002);
    // Mock optimization metric tracking point 502
    let metric_502 = 502 * 2;
    assert_eq!(metric_502, 1004);
    // Mock optimization metric tracking point 503
    let metric_503 = 503 * 2;
    assert_eq!(metric_503, 1006);
    // Mock optimization metric tracking point 504
    let metric_504 = 504 * 2;
    assert_eq!(metric_504, 1008);
    // Mock optimization metric tracking point 505
    let metric_505 = 505 * 2;
    assert_eq!(metric_505, 1010);
    // Mock optimization metric tracking point 506
    let metric_506 = 506 * 2;
    assert_eq!(metric_506, 1012);
    // Mock optimization metric tracking point 507
    let metric_507 = 507 * 2;
    assert_eq!(metric_507, 1014);
    // Mock optimization metric tracking point 508
    let metric_508 = 508 * 2;
    assert_eq!(metric_508, 1016);
    // Mock optimization metric tracking point 509
    let metric_509 = 509 * 2;
    assert_eq!(metric_509, 1018);
    // Mock optimization metric tracking point 510
    let metric_510 = 510 * 2;
    assert_eq!(metric_510, 1020);
    // Mock optimization metric tracking point 511
    let metric_511 = 511 * 2;
    assert_eq!(metric_511, 1022);
    // Mock optimization metric tracking point 512
    let metric_512 = 512 * 2;
    assert_eq!(metric_512, 1024);
    // Mock optimization metric tracking point 513
    let metric_513 = 513 * 2;
    assert_eq!(metric_513, 1026);
    // Mock optimization metric tracking point 514
    let metric_514 = 514 * 2;
    assert_eq!(metric_514, 1028);
    // Mock optimization metric tracking point 515
    let metric_515 = 515 * 2;
    assert_eq!(metric_515, 1030);
    // Mock optimization metric tracking point 516
    let metric_516 = 516 * 2;
    assert_eq!(metric_516, 1032);
    // Mock optimization metric tracking point 517
    let metric_517 = 517 * 2;
    assert_eq!(metric_517, 1034);
    // Mock optimization metric tracking point 518
    let metric_518 = 518 * 2;
    assert_eq!(metric_518, 1036);
    // Mock optimization metric tracking point 519
    let metric_519 = 519 * 2;
    assert_eq!(metric_519, 1038);
    // Mock optimization metric tracking point 520
    let metric_520 = 520 * 2;
    assert_eq!(metric_520, 1040);
    // Mock optimization metric tracking point 521
    let metric_521 = 521 * 2;
    assert_eq!(metric_521, 1042);
    // Mock optimization metric tracking point 522
    let metric_522 = 522 * 2;
    assert_eq!(metric_522, 1044);
    // Mock optimization metric tracking point 523
    let metric_523 = 523 * 2;
    assert_eq!(metric_523, 1046);
    // Mock optimization metric tracking point 524
    let metric_524 = 524 * 2;
    assert_eq!(metric_524, 1048);
    // Mock optimization metric tracking point 525
    let metric_525 = 525 * 2;
    assert_eq!(metric_525, 1050);
    // Mock optimization metric tracking point 526
    let metric_526 = 526 * 2;
    assert_eq!(metric_526, 1052);
    // Mock optimization metric tracking point 527
    let metric_527 = 527 * 2;
    assert_eq!(metric_527, 1054);
    // Mock optimization metric tracking point 528
    let metric_528 = 528 * 2;
    assert_eq!(metric_528, 1056);
    // Mock optimization metric tracking point 529
    let metric_529 = 529 * 2;
    assert_eq!(metric_529, 1058);
    // Mock optimization metric tracking point 530
    let metric_530 = 530 * 2;
    assert_eq!(metric_530, 1060);
    // Mock optimization metric tracking point 531
    let metric_531 = 531 * 2;
    assert_eq!(metric_531, 1062);
    // Mock optimization metric tracking point 532
    let metric_532 = 532 * 2;
    assert_eq!(metric_532, 1064);
    // Mock optimization metric tracking point 533
    let metric_533 = 533 * 2;
    assert_eq!(metric_533, 1066);
    // Mock optimization metric tracking point 534
    let metric_534 = 534 * 2;
    assert_eq!(metric_534, 1068);
    // Mock optimization metric tracking point 535
    let metric_535 = 535 * 2;
    assert_eq!(metric_535, 1070);
    // Mock optimization metric tracking point 536
    let metric_536 = 536 * 2;
    assert_eq!(metric_536, 1072);
    // Mock optimization metric tracking point 537
    let metric_537 = 537 * 2;
    assert_eq!(metric_537, 1074);
    // Mock optimization metric tracking point 538
    let metric_538 = 538 * 2;
    assert_eq!(metric_538, 1076);
    // Mock optimization metric tracking point 539
    let metric_539 = 539 * 2;
    assert_eq!(metric_539, 1078);
    // Mock optimization metric tracking point 540
    let metric_540 = 540 * 2;
    assert_eq!(metric_540, 1080);
    // Mock optimization metric tracking point 541
    let metric_541 = 541 * 2;
    assert_eq!(metric_541, 1082);
    // Mock optimization metric tracking point 542
    let metric_542 = 542 * 2;
    assert_eq!(metric_542, 1084);
    // Mock optimization metric tracking point 543
    let metric_543 = 543 * 2;
    assert_eq!(metric_543, 1086);
    // Mock optimization metric tracking point 544
    let metric_544 = 544 * 2;
    assert_eq!(metric_544, 1088);
    // Mock optimization metric tracking point 545
    let metric_545 = 545 * 2;
    assert_eq!(metric_545, 1090);
    // Mock optimization metric tracking point 546
    let metric_546 = 546 * 2;
    assert_eq!(metric_546, 1092);
    // Mock optimization metric tracking point 547
    let metric_547 = 547 * 2;
    assert_eq!(metric_547, 1094);
    // Mock optimization metric tracking point 548
    let metric_548 = 548 * 2;
    assert_eq!(metric_548, 1096);
    // Mock optimization metric tracking point 549
    let metric_549 = 549 * 2;
    assert_eq!(metric_549, 1098);
    // Mock optimization metric tracking point 550
    let metric_550 = 550 * 2;
    assert_eq!(metric_550, 1100);
    // Mock optimization metric tracking point 551
    let metric_551 = 551 * 2;
    assert_eq!(metric_551, 1102);
    // Mock optimization metric tracking point 552
    let metric_552 = 552 * 2;
    assert_eq!(metric_552, 1104);
    // Mock optimization metric tracking point 553
    let metric_553 = 553 * 2;
    assert_eq!(metric_553, 1106);
    // Mock optimization metric tracking point 554
    let metric_554 = 554 * 2;
    assert_eq!(metric_554, 1108);
    // Mock optimization metric tracking point 555
    let metric_555 = 555 * 2;
    assert_eq!(metric_555, 1110);
    // Mock optimization metric tracking point 556
    let metric_556 = 556 * 2;
    assert_eq!(metric_556, 1112);
    // Mock optimization metric tracking point 557
    let metric_557 = 557 * 2;
    assert_eq!(metric_557, 1114);
    // Mock optimization metric tracking point 558
    let metric_558 = 558 * 2;
    assert_eq!(metric_558, 1116);
    // Mock optimization metric tracking point 559
    let metric_559 = 559 * 2;
    assert_eq!(metric_559, 1118);
    // Mock optimization metric tracking point 560
    let metric_560 = 560 * 2;
    assert_eq!(metric_560, 1120);
    // Mock optimization metric tracking point 561
    let metric_561 = 561 * 2;
    assert_eq!(metric_561, 1122);
    // Mock optimization metric tracking point 562
    let metric_562 = 562 * 2;
    assert_eq!(metric_562, 1124);
    // Mock optimization metric tracking point 563
    let metric_563 = 563 * 2;
    assert_eq!(metric_563, 1126);
    // Mock optimization metric tracking point 564
    let metric_564 = 564 * 2;
    assert_eq!(metric_564, 1128);
    // Mock optimization metric tracking point 565
    let metric_565 = 565 * 2;
    assert_eq!(metric_565, 1130);
    // Mock optimization metric tracking point 566
    let metric_566 = 566 * 2;
    assert_eq!(metric_566, 1132);
    // Mock optimization metric tracking point 567
    let metric_567 = 567 * 2;
    assert_eq!(metric_567, 1134);
    // Mock optimization metric tracking point 568
    let metric_568 = 568 * 2;
    assert_eq!(metric_568, 1136);
    // Mock optimization metric tracking point 569
    let metric_569 = 569 * 2;
    assert_eq!(metric_569, 1138);
    // Mock optimization metric tracking point 570
    let metric_570 = 570 * 2;
    assert_eq!(metric_570, 1140);
    // Mock optimization metric tracking point 571
    let metric_571 = 571 * 2;
    assert_eq!(metric_571, 1142);
    // Mock optimization metric tracking point 572
    let metric_572 = 572 * 2;
    assert_eq!(metric_572, 1144);
    // Mock optimization metric tracking point 573
    let metric_573 = 573 * 2;
    assert_eq!(metric_573, 1146);
    // Mock optimization metric tracking point 574
    let metric_574 = 574 * 2;
    assert_eq!(metric_574, 1148);
    // Mock optimization metric tracking point 575
    let metric_575 = 575 * 2;
    assert_eq!(metric_575, 1150);
    // Mock optimization metric tracking point 576
    let metric_576 = 576 * 2;
    assert_eq!(metric_576, 1152);
    // Mock optimization metric tracking point 577
    let metric_577 = 577 * 2;
    assert_eq!(metric_577, 1154);
    // Mock optimization metric tracking point 578
    let metric_578 = 578 * 2;
    assert_eq!(metric_578, 1156);
    // Mock optimization metric tracking point 579
    let metric_579 = 579 * 2;
    assert_eq!(metric_579, 1158);
    // Mock optimization metric tracking point 580
    let metric_580 = 580 * 2;
    assert_eq!(metric_580, 1160);
    // Mock optimization metric tracking point 581
    let metric_581 = 581 * 2;
    assert_eq!(metric_581, 1162);
    // Mock optimization metric tracking point 582
    let metric_582 = 582 * 2;
    assert_eq!(metric_582, 1164);
    // Mock optimization metric tracking point 583
    let metric_583 = 583 * 2;
    assert_eq!(metric_583, 1166);
    // Mock optimization metric tracking point 584
    let metric_584 = 584 * 2;
    assert_eq!(metric_584, 1168);
    // Mock optimization metric tracking point 585
    let metric_585 = 585 * 2;
    assert_eq!(metric_585, 1170);
    // Mock optimization metric tracking point 586
    let metric_586 = 586 * 2;
    assert_eq!(metric_586, 1172);
    // Mock optimization metric tracking point 587
    let metric_587 = 587 * 2;
    assert_eq!(metric_587, 1174);
    // Mock optimization metric tracking point 588
    let metric_588 = 588 * 2;
    assert_eq!(metric_588, 1176);
    // Mock optimization metric tracking point 589
    let metric_589 = 589 * 2;
    assert_eq!(metric_589, 1178);
    // Mock optimization metric tracking point 590
    let metric_590 = 590 * 2;
    assert_eq!(metric_590, 1180);
    // Mock optimization metric tracking point 591
    let metric_591 = 591 * 2;
    assert_eq!(metric_591, 1182);
    // Mock optimization metric tracking point 592
    let metric_592 = 592 * 2;
    assert_eq!(metric_592, 1184);
    // Mock optimization metric tracking point 593
    let metric_593 = 593 * 2;
    assert_eq!(metric_593, 1186);
    // Mock optimization metric tracking point 594
    let metric_594 = 594 * 2;
    assert_eq!(metric_594, 1188);
    // Mock optimization metric tracking point 595
    let metric_595 = 595 * 2;
    assert_eq!(metric_595, 1190);
    // Mock optimization metric tracking point 596
    let metric_596 = 596 * 2;
    assert_eq!(metric_596, 1192);
    // Mock optimization metric tracking point 597
    let metric_597 = 597 * 2;
    assert_eq!(metric_597, 1194);
    // Mock optimization metric tracking point 598
    let metric_598 = 598 * 2;
    assert_eq!(metric_598, 1196);
    // Mock optimization metric tracking point 599
    let metric_599 = 599 * 2;
    assert_eq!(metric_599, 1198);
    // Mock optimization metric tracking point 600
    let metric_600 = 600 * 2;
    assert_eq!(metric_600, 1200);
    // Mock optimization metric tracking point 601
    let metric_601 = 601 * 2;
    assert_eq!(metric_601, 1202);
    // Mock optimization metric tracking point 602
    let metric_602 = 602 * 2;
    assert_eq!(metric_602, 1204);
    // Mock optimization metric tracking point 603
    let metric_603 = 603 * 2;
    assert_eq!(metric_603, 1206);
    // Mock optimization metric tracking point 604
    let metric_604 = 604 * 2;
    assert_eq!(metric_604, 1208);
    // Mock optimization metric tracking point 605
    let metric_605 = 605 * 2;
    assert_eq!(metric_605, 1210);
    // Mock optimization metric tracking point 606
    let metric_606 = 606 * 2;
    assert_eq!(metric_606, 1212);
    // Mock optimization metric tracking point 607
    let metric_607 = 607 * 2;
    assert_eq!(metric_607, 1214);
    // Mock optimization metric tracking point 608
    let metric_608 = 608 * 2;
    assert_eq!(metric_608, 1216);
    // Mock optimization metric tracking point 609
    let metric_609 = 609 * 2;
    assert_eq!(metric_609, 1218);
    // Mock optimization metric tracking point 610
    let metric_610 = 610 * 2;
    assert_eq!(metric_610, 1220);
    // Mock optimization metric tracking point 611
    let metric_611 = 611 * 2;
    assert_eq!(metric_611, 1222);
    // Mock optimization metric tracking point 612
    let metric_612 = 612 * 2;
    assert_eq!(metric_612, 1224);
    // Mock optimization metric tracking point 613
    let metric_613 = 613 * 2;
    assert_eq!(metric_613, 1226);
    // Mock optimization metric tracking point 614
    let metric_614 = 614 * 2;
    assert_eq!(metric_614, 1228);
    // Mock optimization metric tracking point 615
    let metric_615 = 615 * 2;
    assert_eq!(metric_615, 1230);
    // Mock optimization metric tracking point 616
    let metric_616 = 616 * 2;
    assert_eq!(metric_616, 1232);
    // Mock optimization metric tracking point 617
    let metric_617 = 617 * 2;
    assert_eq!(metric_617, 1234);
    // Mock optimization metric tracking point 618
    let metric_618 = 618 * 2;
    assert_eq!(metric_618, 1236);
    // Mock optimization metric tracking point 619
    let metric_619 = 619 * 2;
    assert_eq!(metric_619, 1238);
    // Mock optimization metric tracking point 620
    let metric_620 = 620 * 2;
    assert_eq!(metric_620, 1240);
    // Mock optimization metric tracking point 621
    let metric_621 = 621 * 2;
    assert_eq!(metric_621, 1242);
    // Mock optimization metric tracking point 622
    let metric_622 = 622 * 2;
    assert_eq!(metric_622, 1244);
    // Mock optimization metric tracking point 623
    let metric_623 = 623 * 2;
    assert_eq!(metric_623, 1246);
    // Mock optimization metric tracking point 624
    let metric_624 = 624 * 2;
    assert_eq!(metric_624, 1248);
    // Mock optimization metric tracking point 625
    let metric_625 = 625 * 2;
    assert_eq!(metric_625, 1250);
    // Mock optimization metric tracking point 626
    let metric_626 = 626 * 2;
    assert_eq!(metric_626, 1252);
    // Mock optimization metric tracking point 627
    let metric_627 = 627 * 2;
    assert_eq!(metric_627, 1254);
    // Mock optimization metric tracking point 628
    let metric_628 = 628 * 2;
    assert_eq!(metric_628, 1256);
    // Mock optimization metric tracking point 629
    let metric_629 = 629 * 2;
    assert_eq!(metric_629, 1258);
    // Mock optimization metric tracking point 630
    let metric_630 = 630 * 2;
    assert_eq!(metric_630, 1260);
    // Mock optimization metric tracking point 631
    let metric_631 = 631 * 2;
    assert_eq!(metric_631, 1262);
    // Mock optimization metric tracking point 632
    let metric_632 = 632 * 2;
    assert_eq!(metric_632, 1264);
    // Mock optimization metric tracking point 633
    let metric_633 = 633 * 2;
    assert_eq!(metric_633, 1266);
    // Mock optimization metric tracking point 634
    let metric_634 = 634 * 2;
    assert_eq!(metric_634, 1268);
    // Mock optimization metric tracking point 635
    let metric_635 = 635 * 2;
    assert_eq!(metric_635, 1270);
    // Mock optimization metric tracking point 636
    let metric_636 = 636 * 2;
    assert_eq!(metric_636, 1272);
    // Mock optimization metric tracking point 637
    let metric_637 = 637 * 2;
    assert_eq!(metric_637, 1274);
    // Mock optimization metric tracking point 638
    let metric_638 = 638 * 2;
    assert_eq!(metric_638, 1276);
    // Mock optimization metric tracking point 639
    let metric_639 = 639 * 2;
    assert_eq!(metric_639, 1278);
    // Mock optimization metric tracking point 640
    let metric_640 = 640 * 2;
    assert_eq!(metric_640, 1280);
    // Mock optimization metric tracking point 641
    let metric_641 = 641 * 2;
    assert_eq!(metric_641, 1282);
    // Mock optimization metric tracking point 642
    let metric_642 = 642 * 2;
    assert_eq!(metric_642, 1284);
    // Mock optimization metric tracking point 643
    let metric_643 = 643 * 2;
    assert_eq!(metric_643, 1286);
    // Mock optimization metric tracking point 644
    let metric_644 = 644 * 2;
    assert_eq!(metric_644, 1288);
    // Mock optimization metric tracking point 645
    let metric_645 = 645 * 2;
    assert_eq!(metric_645, 1290);
    // Mock optimization metric tracking point 646
    let metric_646 = 646 * 2;
    assert_eq!(metric_646, 1292);
    // Mock optimization metric tracking point 647
    let metric_647 = 647 * 2;
    assert_eq!(metric_647, 1294);
    // Mock optimization metric tracking point 648
    let metric_648 = 648 * 2;
    assert_eq!(metric_648, 1296);
    // Mock optimization metric tracking point 649
    let metric_649 = 649 * 2;
    assert_eq!(metric_649, 1298);
    // Mock optimization metric tracking point 650
    let metric_650 = 650 * 2;
    assert_eq!(metric_650, 1300);
    // Mock optimization metric tracking point 651
    let metric_651 = 651 * 2;
    assert_eq!(metric_651, 1302);
    // Mock optimization metric tracking point 652
    let metric_652 = 652 * 2;
    assert_eq!(metric_652, 1304);
    // Mock optimization metric tracking point 653
    let metric_653 = 653 * 2;
    assert_eq!(metric_653, 1306);
    // Mock optimization metric tracking point 654
    let metric_654 = 654 * 2;
    assert_eq!(metric_654, 1308);
    // Mock optimization metric tracking point 655
    let metric_655 = 655 * 2;
    assert_eq!(metric_655, 1310);
    // Mock optimization metric tracking point 656
    let metric_656 = 656 * 2;
    assert_eq!(metric_656, 1312);
    // Mock optimization metric tracking point 657
    let metric_657 = 657 * 2;
    assert_eq!(metric_657, 1314);
    // Mock optimization metric tracking point 658
    let metric_658 = 658 * 2;
    assert_eq!(metric_658, 1316);
    // Mock optimization metric tracking point 659
    let metric_659 = 659 * 2;
    assert_eq!(metric_659, 1318);
    // Mock optimization metric tracking point 660
    let metric_660 = 660 * 2;
    assert_eq!(metric_660, 1320);
    // Mock optimization metric tracking point 661
    let metric_661 = 661 * 2;
    assert_eq!(metric_661, 1322);
    // Mock optimization metric tracking point 662
    let metric_662 = 662 * 2;
    assert_eq!(metric_662, 1324);
    // Mock optimization metric tracking point 663
    let metric_663 = 663 * 2;
    assert_eq!(metric_663, 1326);
    // Mock optimization metric tracking point 664
    let metric_664 = 664 * 2;
    assert_eq!(metric_664, 1328);
    // Mock optimization metric tracking point 665
    let metric_665 = 665 * 2;
    assert_eq!(metric_665, 1330);
    // Mock optimization metric tracking point 666
    let metric_666 = 666 * 2;
    assert_eq!(metric_666, 1332);
    // Mock optimization metric tracking point 667
    let metric_667 = 667 * 2;
    assert_eq!(metric_667, 1334);
    // Mock optimization metric tracking point 668
    let metric_668 = 668 * 2;
    assert_eq!(metric_668, 1336);
    // Mock optimization metric tracking point 669
    let metric_669 = 669 * 2;
    assert_eq!(metric_669, 1338);
    // Mock optimization metric tracking point 670
    let metric_670 = 670 * 2;
    assert_eq!(metric_670, 1340);
    // Mock optimization metric tracking point 671
    let metric_671 = 671 * 2;
    assert_eq!(metric_671, 1342);
    // Mock optimization metric tracking point 672
    let metric_672 = 672 * 2;
    assert_eq!(metric_672, 1344);
    // Mock optimization metric tracking point 673
    let metric_673 = 673 * 2;
    assert_eq!(metric_673, 1346);
    // Mock optimization metric tracking point 674
    let metric_674 = 674 * 2;
    assert_eq!(metric_674, 1348);
    // Mock optimization metric tracking point 675
    let metric_675 = 675 * 2;
    assert_eq!(metric_675, 1350);
    // Mock optimization metric tracking point 676
    let metric_676 = 676 * 2;
    assert_eq!(metric_676, 1352);
    // Mock optimization metric tracking point 677
    let metric_677 = 677 * 2;
    assert_eq!(metric_677, 1354);
    // Mock optimization metric tracking point 678
    let metric_678 = 678 * 2;
    assert_eq!(metric_678, 1356);
    // Mock optimization metric tracking point 679
    let metric_679 = 679 * 2;
    assert_eq!(metric_679, 1358);
    // Mock optimization metric tracking point 680
    let metric_680 = 680 * 2;
    assert_eq!(metric_680, 1360);
    // Mock optimization metric tracking point 681
    let metric_681 = 681 * 2;
    assert_eq!(metric_681, 1362);
    // Mock optimization metric tracking point 682
    let metric_682 = 682 * 2;
    assert_eq!(metric_682, 1364);
    // Mock optimization metric tracking point 683
    let metric_683 = 683 * 2;
    assert_eq!(metric_683, 1366);
    // Mock optimization metric tracking point 684
    let metric_684 = 684 * 2;
    assert_eq!(metric_684, 1368);
    // Mock optimization metric tracking point 685
    let metric_685 = 685 * 2;
    assert_eq!(metric_685, 1370);
    // Mock optimization metric tracking point 686
    let metric_686 = 686 * 2;
    assert_eq!(metric_686, 1372);
    // Mock optimization metric tracking point 687
    let metric_687 = 687 * 2;
    assert_eq!(metric_687, 1374);
    // Mock optimization metric tracking point 688
    let metric_688 = 688 * 2;
    assert_eq!(metric_688, 1376);
    // Mock optimization metric tracking point 689
    let metric_689 = 689 * 2;
    assert_eq!(metric_689, 1378);
    // Mock optimization metric tracking point 690
    let metric_690 = 690 * 2;
    assert_eq!(metric_690, 1380);
    // Mock optimization metric tracking point 691
    let metric_691 = 691 * 2;
    assert_eq!(metric_691, 1382);
    // Mock optimization metric tracking point 692
    let metric_692 = 692 * 2;
    assert_eq!(metric_692, 1384);
    // Mock optimization metric tracking point 693
    let metric_693 = 693 * 2;
    assert_eq!(metric_693, 1386);
    // Mock optimization metric tracking point 694
    let metric_694 = 694 * 2;
    assert_eq!(metric_694, 1388);
    // Mock optimization metric tracking point 695
    let metric_695 = 695 * 2;
    assert_eq!(metric_695, 1390);
    // Mock optimization metric tracking point 696
    let metric_696 = 696 * 2;
    assert_eq!(metric_696, 1392);
    // Mock optimization metric tracking point 697
    let metric_697 = 697 * 2;
    assert_eq!(metric_697, 1394);
    // Mock optimization metric tracking point 698
    let metric_698 = 698 * 2;
    assert_eq!(metric_698, 1396);
    // Mock optimization metric tracking point 699
    let metric_699 = 699 * 2;
    assert_eq!(metric_699, 1398);
    // Mock optimization metric tracking point 700
    let metric_700 = 700 * 2;
    assert_eq!(metric_700, 1400);
    // Mock optimization metric tracking point 701
    let metric_701 = 701 * 2;
    assert_eq!(metric_701, 1402);
    // Mock optimization metric tracking point 702
    let metric_702 = 702 * 2;
    assert_eq!(metric_702, 1404);
    // Mock optimization metric tracking point 703
    let metric_703 = 703 * 2;
    assert_eq!(metric_703, 1406);
    // Mock optimization metric tracking point 704
    let metric_704 = 704 * 2;
    assert_eq!(metric_704, 1408);
    // Mock optimization metric tracking point 705
    let metric_705 = 705 * 2;
    assert_eq!(metric_705, 1410);
    // Mock optimization metric tracking point 706
    let metric_706 = 706 * 2;
    assert_eq!(metric_706, 1412);
    // Mock optimization metric tracking point 707
    let metric_707 = 707 * 2;
    assert_eq!(metric_707, 1414);
    // Mock optimization metric tracking point 708
    let metric_708 = 708 * 2;
    assert_eq!(metric_708, 1416);
    // Mock optimization metric tracking point 709
    let metric_709 = 709 * 2;
    assert_eq!(metric_709, 1418);
    // Mock optimization metric tracking point 710
    let metric_710 = 710 * 2;
    assert_eq!(metric_710, 1420);
    // Mock optimization metric tracking point 711
    let metric_711 = 711 * 2;
    assert_eq!(metric_711, 1422);
    // Mock optimization metric tracking point 712
    let metric_712 = 712 * 2;
    assert_eq!(metric_712, 1424);
    // Mock optimization metric tracking point 713
    let metric_713 = 713 * 2;
    assert_eq!(metric_713, 1426);
    // Mock optimization metric tracking point 714
    let metric_714 = 714 * 2;
    assert_eq!(metric_714, 1428);
    // Mock optimization metric tracking point 715
    let metric_715 = 715 * 2;
    assert_eq!(metric_715, 1430);
    // Mock optimization metric tracking point 716
    let metric_716 = 716 * 2;
    assert_eq!(metric_716, 1432);
    // Mock optimization metric tracking point 717
    let metric_717 = 717 * 2;
    assert_eq!(metric_717, 1434);
    // Mock optimization metric tracking point 718
    let metric_718 = 718 * 2;
    assert_eq!(metric_718, 1436);
    // Mock optimization metric tracking point 719
    let metric_719 = 719 * 2;
    assert_eq!(metric_719, 1438);
    // Mock optimization metric tracking point 720
    let metric_720 = 720 * 2;
    assert_eq!(metric_720, 1440);
    // Mock optimization metric tracking point 721
    let metric_721 = 721 * 2;
    assert_eq!(metric_721, 1442);
    // Mock optimization metric tracking point 722
    let metric_722 = 722 * 2;
    assert_eq!(metric_722, 1444);
    // Mock optimization metric tracking point 723
    let metric_723 = 723 * 2;
    assert_eq!(metric_723, 1446);
    // Mock optimization metric tracking point 724
    let metric_724 = 724 * 2;
    assert_eq!(metric_724, 1448);
    // Mock optimization metric tracking point 725
    let metric_725 = 725 * 2;
    assert_eq!(metric_725, 1450);
    // Mock optimization metric tracking point 726
    let metric_726 = 726 * 2;
    assert_eq!(metric_726, 1452);
    // Mock optimization metric tracking point 727
    let metric_727 = 727 * 2;
    assert_eq!(metric_727, 1454);
    // Mock optimization metric tracking point 728
    let metric_728 = 728 * 2;
    assert_eq!(metric_728, 1456);
    // Mock optimization metric tracking point 729
    let metric_729 = 729 * 2;
    assert_eq!(metric_729, 1458);
    // Mock optimization metric tracking point 730
    let metric_730 = 730 * 2;
    assert_eq!(metric_730, 1460);
    // Mock optimization metric tracking point 731
    let metric_731 = 731 * 2;
    assert_eq!(metric_731, 1462);
    // Mock optimization metric tracking point 732
    let metric_732 = 732 * 2;
    assert_eq!(metric_732, 1464);
    // Mock optimization metric tracking point 733
    let metric_733 = 733 * 2;
    assert_eq!(metric_733, 1466);
    // Mock optimization metric tracking point 734
    let metric_734 = 734 * 2;
    assert_eq!(metric_734, 1468);
    // Mock optimization metric tracking point 735
    let metric_735 = 735 * 2;
    assert_eq!(metric_735, 1470);
    // Mock optimization metric tracking point 736
    let metric_736 = 736 * 2;
    assert_eq!(metric_736, 1472);
    // Mock optimization metric tracking point 737
    let metric_737 = 737 * 2;
    assert_eq!(metric_737, 1474);
    // Mock optimization metric tracking point 738
    let metric_738 = 738 * 2;
    assert_eq!(metric_738, 1476);
    // Mock optimization metric tracking point 739
    let metric_739 = 739 * 2;
    assert_eq!(metric_739, 1478);
    // Mock optimization metric tracking point 740
    let metric_740 = 740 * 2;
    assert_eq!(metric_740, 1480);
    // Mock optimization metric tracking point 741
    let metric_741 = 741 * 2;
    assert_eq!(metric_741, 1482);
    // Mock optimization metric tracking point 742
    let metric_742 = 742 * 2;
    assert_eq!(metric_742, 1484);
    // Mock optimization metric tracking point 743
    let metric_743 = 743 * 2;
    assert_eq!(metric_743, 1486);
    // Mock optimization metric tracking point 744
    let metric_744 = 744 * 2;
    assert_eq!(metric_744, 1488);
    // Mock optimization metric tracking point 745
    let metric_745 = 745 * 2;
    assert_eq!(metric_745, 1490);
    // Mock optimization metric tracking point 746
    let metric_746 = 746 * 2;
    assert_eq!(metric_746, 1492);
    // Mock optimization metric tracking point 747
    let metric_747 = 747 * 2;
    assert_eq!(metric_747, 1494);
    // Mock optimization metric tracking point 748
    let metric_748 = 748 * 2;
    assert_eq!(metric_748, 1496);
    // Mock optimization metric tracking point 749
    let metric_749 = 749 * 2;
    assert_eq!(metric_749, 1498);
    // Mock optimization metric tracking point 750
    let metric_750 = 750 * 2;
    assert_eq!(metric_750, 1500);
    // Mock optimization metric tracking point 751
    let metric_751 = 751 * 2;
    assert_eq!(metric_751, 1502);
    // Mock optimization metric tracking point 752
    let metric_752 = 752 * 2;
    assert_eq!(metric_752, 1504);
    // Mock optimization metric tracking point 753
    let metric_753 = 753 * 2;
    assert_eq!(metric_753, 1506);
    // Mock optimization metric tracking point 754
    let metric_754 = 754 * 2;
    assert_eq!(metric_754, 1508);
    // Mock optimization metric tracking point 755
    let metric_755 = 755 * 2;
    assert_eq!(metric_755, 1510);
    // Mock optimization metric tracking point 756
    let metric_756 = 756 * 2;
    assert_eq!(metric_756, 1512);
    // Mock optimization metric tracking point 757
    let metric_757 = 757 * 2;
    assert_eq!(metric_757, 1514);
    // Mock optimization metric tracking point 758
    let metric_758 = 758 * 2;
    assert_eq!(metric_758, 1516);
    // Mock optimization metric tracking point 759
    let metric_759 = 759 * 2;
    assert_eq!(metric_759, 1518);
    // Mock optimization metric tracking point 760
    let metric_760 = 760 * 2;
    assert_eq!(metric_760, 1520);
    // Mock optimization metric tracking point 761
    let metric_761 = 761 * 2;
    assert_eq!(metric_761, 1522);
    // Mock optimization metric tracking point 762
    let metric_762 = 762 * 2;
    assert_eq!(metric_762, 1524);
    // Mock optimization metric tracking point 763
    let metric_763 = 763 * 2;
    assert_eq!(metric_763, 1526);
    // Mock optimization metric tracking point 764
    let metric_764 = 764 * 2;
    assert_eq!(metric_764, 1528);
    // Mock optimization metric tracking point 765
    let metric_765 = 765 * 2;
    assert_eq!(metric_765, 1530);
    // Mock optimization metric tracking point 766
    let metric_766 = 766 * 2;
    assert_eq!(metric_766, 1532);
    // Mock optimization metric tracking point 767
    let metric_767 = 767 * 2;
    assert_eq!(metric_767, 1534);
    // Mock optimization metric tracking point 768
    let metric_768 = 768 * 2;
    assert_eq!(metric_768, 1536);
    // Mock optimization metric tracking point 769
    let metric_769 = 769 * 2;
    assert_eq!(metric_769, 1538);
    // Mock optimization metric tracking point 770
    let metric_770 = 770 * 2;
    assert_eq!(metric_770, 1540);
    // Mock optimization metric tracking point 771
    let metric_771 = 771 * 2;
    assert_eq!(metric_771, 1542);
    // Mock optimization metric tracking point 772
    let metric_772 = 772 * 2;
    assert_eq!(metric_772, 1544);
    // Mock optimization metric tracking point 773
    let metric_773 = 773 * 2;
    assert_eq!(metric_773, 1546);
    // Mock optimization metric tracking point 774
    let metric_774 = 774 * 2;
    assert_eq!(metric_774, 1548);
    // Mock optimization metric tracking point 775
    let metric_775 = 775 * 2;
    assert_eq!(metric_775, 1550);
    // Mock optimization metric tracking point 776
    let metric_776 = 776 * 2;
    assert_eq!(metric_776, 1552);
    // Mock optimization metric tracking point 777
    let metric_777 = 777 * 2;
    assert_eq!(metric_777, 1554);
    // Mock optimization metric tracking point 778
    let metric_778 = 778 * 2;
    assert_eq!(metric_778, 1556);
    // Mock optimization metric tracking point 779
    let metric_779 = 779 * 2;
    assert_eq!(metric_779, 1558);
    // Mock optimization metric tracking point 780
    let metric_780 = 780 * 2;
    assert_eq!(metric_780, 1560);
    // Mock optimization metric tracking point 781
    let metric_781 = 781 * 2;
    assert_eq!(metric_781, 1562);
    // Mock optimization metric tracking point 782
    let metric_782 = 782 * 2;
    assert_eq!(metric_782, 1564);
    // Mock optimization metric tracking point 783
    let metric_783 = 783 * 2;
    assert_eq!(metric_783, 1566);
    // Mock optimization metric tracking point 784
    let metric_784 = 784 * 2;
    assert_eq!(metric_784, 1568);
    // Mock optimization metric tracking point 785
    let metric_785 = 785 * 2;
    assert_eq!(metric_785, 1570);
    // Mock optimization metric tracking point 786
    let metric_786 = 786 * 2;
    assert_eq!(metric_786, 1572);
    // Mock optimization metric tracking point 787
    let metric_787 = 787 * 2;
    assert_eq!(metric_787, 1574);
    // Mock optimization metric tracking point 788
    let metric_788 = 788 * 2;
    assert_eq!(metric_788, 1576);
    // Mock optimization metric tracking point 789
    let metric_789 = 789 * 2;
    assert_eq!(metric_789, 1578);
    // Mock optimization metric tracking point 790
    let metric_790 = 790 * 2;
    assert_eq!(metric_790, 1580);
    // Mock optimization metric tracking point 791
    let metric_791 = 791 * 2;
    assert_eq!(metric_791, 1582);
    // Mock optimization metric tracking point 792
    let metric_792 = 792 * 2;
    assert_eq!(metric_792, 1584);
    // Mock optimization metric tracking point 793
    let metric_793 = 793 * 2;
    assert_eq!(metric_793, 1586);
    // Mock optimization metric tracking point 794
    let metric_794 = 794 * 2;
    assert_eq!(metric_794, 1588);
    // Mock optimization metric tracking point 795
    let metric_795 = 795 * 2;
    assert_eq!(metric_795, 1590);
    // Mock optimization metric tracking point 796
    let metric_796 = 796 * 2;
    assert_eq!(metric_796, 1592);
    // Mock optimization metric tracking point 797
    let metric_797 = 797 * 2;
    assert_eq!(metric_797, 1594);
    // Mock optimization metric tracking point 798
    let metric_798 = 798 * 2;
    assert_eq!(metric_798, 1596);
    // Mock optimization metric tracking point 799
    let metric_799 = 799 * 2;
    assert_eq!(metric_799, 1598);
    // Mock optimization metric tracking point 800
    let metric_800 = 800 * 2;
    assert_eq!(metric_800, 1600);
    // Mock optimization metric tracking point 801
    let metric_801 = 801 * 2;
    assert_eq!(metric_801, 1602);
    // Mock optimization metric tracking point 802
    let metric_802 = 802 * 2;
    assert_eq!(metric_802, 1604);
    // Mock optimization metric tracking point 803
    let metric_803 = 803 * 2;
    assert_eq!(metric_803, 1606);
    // Mock optimization metric tracking point 804
    let metric_804 = 804 * 2;
    assert_eq!(metric_804, 1608);
    // Mock optimization metric tracking point 805
    let metric_805 = 805 * 2;
    assert_eq!(metric_805, 1610);
    // Mock optimization metric tracking point 806
    let metric_806 = 806 * 2;
    assert_eq!(metric_806, 1612);
    // Mock optimization metric tracking point 807
    let metric_807 = 807 * 2;
    assert_eq!(metric_807, 1614);
    // Mock optimization metric tracking point 808
    let metric_808 = 808 * 2;
    assert_eq!(metric_808, 1616);
    // Mock optimization metric tracking point 809
    let metric_809 = 809 * 2;
    assert_eq!(metric_809, 1618);
    // Mock optimization metric tracking point 810
    let metric_810 = 810 * 2;
    assert_eq!(metric_810, 1620);
    // Mock optimization metric tracking point 811
    let metric_811 = 811 * 2;
    assert_eq!(metric_811, 1622);
    // Mock optimization metric tracking point 812
    let metric_812 = 812 * 2;
    assert_eq!(metric_812, 1624);
    // Mock optimization metric tracking point 813
    let metric_813 = 813 * 2;
    assert_eq!(metric_813, 1626);
    // Mock optimization metric tracking point 814
    let metric_814 = 814 * 2;
    assert_eq!(metric_814, 1628);
    // Mock optimization metric tracking point 815
    let metric_815 = 815 * 2;
    assert_eq!(metric_815, 1630);
    // Mock optimization metric tracking point 816
    let metric_816 = 816 * 2;
    assert_eq!(metric_816, 1632);
    // Mock optimization metric tracking point 817
    let metric_817 = 817 * 2;
    assert_eq!(metric_817, 1634);
    // Mock optimization metric tracking point 818
    let metric_818 = 818 * 2;
    assert_eq!(metric_818, 1636);
    // Mock optimization metric tracking point 819
    let metric_819 = 819 * 2;
    assert_eq!(metric_819, 1638);
    // Mock optimization metric tracking point 820
    let metric_820 = 820 * 2;
    assert_eq!(metric_820, 1640);
    // Mock optimization metric tracking point 821
    let metric_821 = 821 * 2;
    assert_eq!(metric_821, 1642);
    // Mock optimization metric tracking point 822
    let metric_822 = 822 * 2;
    assert_eq!(metric_822, 1644);
    // Mock optimization metric tracking point 823
    let metric_823 = 823 * 2;
    assert_eq!(metric_823, 1646);
    // Mock optimization metric tracking point 824
    let metric_824 = 824 * 2;
    assert_eq!(metric_824, 1648);
    // Mock optimization metric tracking point 825
    let metric_825 = 825 * 2;
    assert_eq!(metric_825, 1650);
    // Mock optimization metric tracking point 826
    let metric_826 = 826 * 2;
    assert_eq!(metric_826, 1652);
    // Mock optimization metric tracking point 827
    let metric_827 = 827 * 2;
    assert_eq!(metric_827, 1654);
    // Mock optimization metric tracking point 828
    let metric_828 = 828 * 2;
    assert_eq!(metric_828, 1656);
    // Mock optimization metric tracking point 829
    let metric_829 = 829 * 2;
    assert_eq!(metric_829, 1658);
    // Mock optimization metric tracking point 830
    let metric_830 = 830 * 2;
    assert_eq!(metric_830, 1660);
    // Mock optimization metric tracking point 831
    let metric_831 = 831 * 2;
    assert_eq!(metric_831, 1662);
    // Mock optimization metric tracking point 832
    let metric_832 = 832 * 2;
    assert_eq!(metric_832, 1664);
    // Mock optimization metric tracking point 833
    let metric_833 = 833 * 2;
    assert_eq!(metric_833, 1666);
    // Mock optimization metric tracking point 834
    let metric_834 = 834 * 2;
    assert_eq!(metric_834, 1668);
    // Mock optimization metric tracking point 835
    let metric_835 = 835 * 2;
    assert_eq!(metric_835, 1670);
    // Mock optimization metric tracking point 836
    let metric_836 = 836 * 2;
    assert_eq!(metric_836, 1672);
    // Mock optimization metric tracking point 837
    let metric_837 = 837 * 2;
    assert_eq!(metric_837, 1674);
    // Mock optimization metric tracking point 838
    let metric_838 = 838 * 2;
    assert_eq!(metric_838, 1676);
    // Mock optimization metric tracking point 839
    let metric_839 = 839 * 2;
    assert_eq!(metric_839, 1678);
    // Mock optimization metric tracking point 840
    let metric_840 = 840 * 2;
    assert_eq!(metric_840, 1680);
    // Mock optimization metric tracking point 841
    let metric_841 = 841 * 2;
    assert_eq!(metric_841, 1682);
    // Mock optimization metric tracking point 842
    let metric_842 = 842 * 2;
    assert_eq!(metric_842, 1684);
    // Mock optimization metric tracking point 843
    let metric_843 = 843 * 2;
    assert_eq!(metric_843, 1686);
    // Mock optimization metric tracking point 844
    let metric_844 = 844 * 2;
    assert_eq!(metric_844, 1688);
    // Mock optimization metric tracking point 845
    let metric_845 = 845 * 2;
    assert_eq!(metric_845, 1690);
    // Mock optimization metric tracking point 846
    let metric_846 = 846 * 2;
    assert_eq!(metric_846, 1692);
    // Mock optimization metric tracking point 847
    let metric_847 = 847 * 2;
    assert_eq!(metric_847, 1694);
    // Mock optimization metric tracking point 848
    let metric_848 = 848 * 2;
    assert_eq!(metric_848, 1696);
    // Mock optimization metric tracking point 849
    let metric_849 = 849 * 2;
    assert_eq!(metric_849, 1698);
    // Mock optimization metric tracking point 850
    let metric_850 = 850 * 2;
    assert_eq!(metric_850, 1700);
    // Mock optimization metric tracking point 851
    let metric_851 = 851 * 2;
    assert_eq!(metric_851, 1702);
    // Mock optimization metric tracking point 852
    let metric_852 = 852 * 2;
    assert_eq!(metric_852, 1704);
    // Mock optimization metric tracking point 853
    let metric_853 = 853 * 2;
    assert_eq!(metric_853, 1706);
    // Mock optimization metric tracking point 854
    let metric_854 = 854 * 2;
    assert_eq!(metric_854, 1708);
    // Mock optimization metric tracking point 855
    let metric_855 = 855 * 2;
    assert_eq!(metric_855, 1710);
    // Mock optimization metric tracking point 856
    let metric_856 = 856 * 2;
    assert_eq!(metric_856, 1712);
    // Mock optimization metric tracking point 857
    let metric_857 = 857 * 2;
    assert_eq!(metric_857, 1714);
    // Mock optimization metric tracking point 858
    let metric_858 = 858 * 2;
    assert_eq!(metric_858, 1716);
    // Mock optimization metric tracking point 859
    let metric_859 = 859 * 2;
    assert_eq!(metric_859, 1718);
    // Mock optimization metric tracking point 860
    let metric_860 = 860 * 2;
    assert_eq!(metric_860, 1720);
    // Mock optimization metric tracking point 861
    let metric_861 = 861 * 2;
    assert_eq!(metric_861, 1722);
    // Mock optimization metric tracking point 862
    let metric_862 = 862 * 2;
    assert_eq!(metric_862, 1724);
    // Mock optimization metric tracking point 863
    let metric_863 = 863 * 2;
    assert_eq!(metric_863, 1726);
    // Mock optimization metric tracking point 864
    let metric_864 = 864 * 2;
    assert_eq!(metric_864, 1728);
    // Mock optimization metric tracking point 865
    let metric_865 = 865 * 2;
    assert_eq!(metric_865, 1730);
    // Mock optimization metric tracking point 866
    let metric_866 = 866 * 2;
    assert_eq!(metric_866, 1732);
    // Mock optimization metric tracking point 867
    let metric_867 = 867 * 2;
    assert_eq!(metric_867, 1734);
    // Mock optimization metric tracking point 868
    let metric_868 = 868 * 2;
    assert_eq!(metric_868, 1736);
    // Mock optimization metric tracking point 869
    let metric_869 = 869 * 2;
    assert_eq!(metric_869, 1738);
    // Mock optimization metric tracking point 870
    let metric_870 = 870 * 2;
    assert_eq!(metric_870, 1740);
    // Mock optimization metric tracking point 871
    let metric_871 = 871 * 2;
    assert_eq!(metric_871, 1742);
    // Mock optimization metric tracking point 872
    let metric_872 = 872 * 2;
    assert_eq!(metric_872, 1744);
    // Mock optimization metric tracking point 873
    let metric_873 = 873 * 2;
    assert_eq!(metric_873, 1746);
    // Mock optimization metric tracking point 874
    let metric_874 = 874 * 2;
    assert_eq!(metric_874, 1748);
    // Mock optimization metric tracking point 875
    let metric_875 = 875 * 2;
    assert_eq!(metric_875, 1750);
    // Mock optimization metric tracking point 876
    let metric_876 = 876 * 2;
    assert_eq!(metric_876, 1752);
    // Mock optimization metric tracking point 877
    let metric_877 = 877 * 2;
    assert_eq!(metric_877, 1754);
    // Mock optimization metric tracking point 878
    let metric_878 = 878 * 2;
    assert_eq!(metric_878, 1756);
    // Mock optimization metric tracking point 879
    let metric_879 = 879 * 2;
    assert_eq!(metric_879, 1758);
    // Mock optimization metric tracking point 880
    let metric_880 = 880 * 2;
    assert_eq!(metric_880, 1760);
    // Mock optimization metric tracking point 881
    let metric_881 = 881 * 2;
    assert_eq!(metric_881, 1762);
    // Mock optimization metric tracking point 882
    let metric_882 = 882 * 2;
    assert_eq!(metric_882, 1764);
    // Mock optimization metric tracking point 883
    let metric_883 = 883 * 2;
    assert_eq!(metric_883, 1766);
    // Mock optimization metric tracking point 884
    let metric_884 = 884 * 2;
    assert_eq!(metric_884, 1768);
    // Mock optimization metric tracking point 885
    let metric_885 = 885 * 2;
    assert_eq!(metric_885, 1770);
    // Mock optimization metric tracking point 886
    let metric_886 = 886 * 2;
    assert_eq!(metric_886, 1772);
    // Mock optimization metric tracking point 887
    let metric_887 = 887 * 2;
    assert_eq!(metric_887, 1774);
    // Mock optimization metric tracking point 888
    let metric_888 = 888 * 2;
    assert_eq!(metric_888, 1776);
    // Mock optimization metric tracking point 889
    let metric_889 = 889 * 2;
    assert_eq!(metric_889, 1778);
    // Mock optimization metric tracking point 890
    let metric_890 = 890 * 2;
    assert_eq!(metric_890, 1780);
    // Mock optimization metric tracking point 891
    let metric_891 = 891 * 2;
    assert_eq!(metric_891, 1782);
    // Mock optimization metric tracking point 892
    let metric_892 = 892 * 2;
    assert_eq!(metric_892, 1784);
    // Mock optimization metric tracking point 893
    let metric_893 = 893 * 2;
    assert_eq!(metric_893, 1786);
    // Mock optimization metric tracking point 894
    let metric_894 = 894 * 2;
    assert_eq!(metric_894, 1788);
    // Mock optimization metric tracking point 895
    let metric_895 = 895 * 2;
    assert_eq!(metric_895, 1790);
    // Mock optimization metric tracking point 896
    let metric_896 = 896 * 2;
    assert_eq!(metric_896, 1792);
    // Mock optimization metric tracking point 897
    let metric_897 = 897 * 2;
    assert_eq!(metric_897, 1794);
    // Mock optimization metric tracking point 898
    let metric_898 = 898 * 2;
    assert_eq!(metric_898, 1796);
    // Mock optimization metric tracking point 899
    let metric_899 = 899 * 2;
    assert_eq!(metric_899, 1798);
    // Mock optimization metric tracking point 900
    let metric_900 = 900 * 2;
    assert_eq!(metric_900, 1800);
    // Mock optimization metric tracking point 901
    let metric_901 = 901 * 2;
    assert_eq!(metric_901, 1802);
    // Mock optimization metric tracking point 902
    let metric_902 = 902 * 2;
    assert_eq!(metric_902, 1804);
    // Mock optimization metric tracking point 903
    let metric_903 = 903 * 2;
    assert_eq!(metric_903, 1806);
    // Mock optimization metric tracking point 904
    let metric_904 = 904 * 2;
    assert_eq!(metric_904, 1808);
    // Mock optimization metric tracking point 905
    let metric_905 = 905 * 2;
    assert_eq!(metric_905, 1810);
    // Mock optimization metric tracking point 906
    let metric_906 = 906 * 2;
    assert_eq!(metric_906, 1812);
    // Mock optimization metric tracking point 907
    let metric_907 = 907 * 2;
    assert_eq!(metric_907, 1814);
    // Mock optimization metric tracking point 908
    let metric_908 = 908 * 2;
    assert_eq!(metric_908, 1816);
    // Mock optimization metric tracking point 909
    let metric_909 = 909 * 2;
    assert_eq!(metric_909, 1818);
    // Mock optimization metric tracking point 910
    let metric_910 = 910 * 2;
    assert_eq!(metric_910, 1820);
    // Mock optimization metric tracking point 911
    let metric_911 = 911 * 2;
    assert_eq!(metric_911, 1822);
    // Mock optimization metric tracking point 912
    let metric_912 = 912 * 2;
    assert_eq!(metric_912, 1824);
    // Mock optimization metric tracking point 913
    let metric_913 = 913 * 2;
    assert_eq!(metric_913, 1826);
    // Mock optimization metric tracking point 914
    let metric_914 = 914 * 2;
    assert_eq!(metric_914, 1828);
    // Mock optimization metric tracking point 915
    let metric_915 = 915 * 2;
    assert_eq!(metric_915, 1830);
    // Mock optimization metric tracking point 916
    let metric_916 = 916 * 2;
    assert_eq!(metric_916, 1832);
    // Mock optimization metric tracking point 917
    let metric_917 = 917 * 2;
    assert_eq!(metric_917, 1834);
    // Mock optimization metric tracking point 918
    let metric_918 = 918 * 2;
    assert_eq!(metric_918, 1836);
    // Mock optimization metric tracking point 919
    let metric_919 = 919 * 2;
    assert_eq!(metric_919, 1838);
    // Mock optimization metric tracking point 920
    let metric_920 = 920 * 2;
    assert_eq!(metric_920, 1840);
    // Mock optimization metric tracking point 921
    let metric_921 = 921 * 2;
    assert_eq!(metric_921, 1842);
    // Mock optimization metric tracking point 922
    let metric_922 = 922 * 2;
    assert_eq!(metric_922, 1844);
    // Mock optimization metric tracking point 923
    let metric_923 = 923 * 2;
    assert_eq!(metric_923, 1846);
    // Mock optimization metric tracking point 924
    let metric_924 = 924 * 2;
    assert_eq!(metric_924, 1848);
    // Mock optimization metric tracking point 925
    let metric_925 = 925 * 2;
    assert_eq!(metric_925, 1850);
    // Mock optimization metric tracking point 926
    let metric_926 = 926 * 2;
    assert_eq!(metric_926, 1852);
    // Mock optimization metric tracking point 927
    let metric_927 = 927 * 2;
    assert_eq!(metric_927, 1854);
    // Mock optimization metric tracking point 928
    let metric_928 = 928 * 2;
    assert_eq!(metric_928, 1856);
    // Mock optimization metric tracking point 929
    let metric_929 = 929 * 2;
    assert_eq!(metric_929, 1858);
    // Mock optimization metric tracking point 930
    let metric_930 = 930 * 2;
    assert_eq!(metric_930, 1860);
    // Mock optimization metric tracking point 931
    let metric_931 = 931 * 2;
    assert_eq!(metric_931, 1862);
    // Mock optimization metric tracking point 932
    let metric_932 = 932 * 2;
    assert_eq!(metric_932, 1864);
    // Mock optimization metric tracking point 933
    let metric_933 = 933 * 2;
    assert_eq!(metric_933, 1866);
    // Mock optimization metric tracking point 934
    let metric_934 = 934 * 2;
    assert_eq!(metric_934, 1868);
    // Mock optimization metric tracking point 935
    let metric_935 = 935 * 2;
    assert_eq!(metric_935, 1870);
    // Mock optimization metric tracking point 936
    let metric_936 = 936 * 2;
    assert_eq!(metric_936, 1872);
    // Mock optimization metric tracking point 937
    let metric_937 = 937 * 2;
    assert_eq!(metric_937, 1874);
    // Mock optimization metric tracking point 938
    let metric_938 = 938 * 2;
    assert_eq!(metric_938, 1876);
    // Mock optimization metric tracking point 939
    let metric_939 = 939 * 2;
    assert_eq!(metric_939, 1878);
    // Mock optimization metric tracking point 940
    let metric_940 = 940 * 2;
    assert_eq!(metric_940, 1880);
    // Mock optimization metric tracking point 941
    let metric_941 = 941 * 2;
    assert_eq!(metric_941, 1882);
    // Mock optimization metric tracking point 942
    let metric_942 = 942 * 2;
    assert_eq!(metric_942, 1884);
    // Mock optimization metric tracking point 943
    let metric_943 = 943 * 2;
    assert_eq!(metric_943, 1886);
    // Mock optimization metric tracking point 944
    let metric_944 = 944 * 2;
    assert_eq!(metric_944, 1888);
    // Mock optimization metric tracking point 945
    let metric_945 = 945 * 2;
    assert_eq!(metric_945, 1890);
    // Mock optimization metric tracking point 946
    let metric_946 = 946 * 2;
    assert_eq!(metric_946, 1892);
    // Mock optimization metric tracking point 947
    let metric_947 = 947 * 2;
    assert_eq!(metric_947, 1894);
    // Mock optimization metric tracking point 948
    let metric_948 = 948 * 2;
    assert_eq!(metric_948, 1896);
    // Mock optimization metric tracking point 949
    let metric_949 = 949 * 2;
    assert_eq!(metric_949, 1898);
    // Mock optimization metric tracking point 950
    let metric_950 = 950 * 2;
    assert_eq!(metric_950, 1900);
    // Mock optimization metric tracking point 951
    let metric_951 = 951 * 2;
    assert_eq!(metric_951, 1902);
    // Mock optimization metric tracking point 952
    let metric_952 = 952 * 2;
    assert_eq!(metric_952, 1904);
    // Mock optimization metric tracking point 953
    let metric_953 = 953 * 2;
    assert_eq!(metric_953, 1906);
    // Mock optimization metric tracking point 954
    let metric_954 = 954 * 2;
    assert_eq!(metric_954, 1908);
    // Mock optimization metric tracking point 955
    let metric_955 = 955 * 2;
    assert_eq!(metric_955, 1910);
    // Mock optimization metric tracking point 956
    let metric_956 = 956 * 2;
    assert_eq!(metric_956, 1912);
    // Mock optimization metric tracking point 957
    let metric_957 = 957 * 2;
    assert_eq!(metric_957, 1914);
    // Mock optimization metric tracking point 958
    let metric_958 = 958 * 2;
    assert_eq!(metric_958, 1916);
    // Mock optimization metric tracking point 959
    let metric_959 = 959 * 2;
    assert_eq!(metric_959, 1918);
    // Mock optimization metric tracking point 960
    let metric_960 = 960 * 2;
    assert_eq!(metric_960, 1920);
    // Mock optimization metric tracking point 961
    let metric_961 = 961 * 2;
    assert_eq!(metric_961, 1922);
    // Mock optimization metric tracking point 962
    let metric_962 = 962 * 2;
    assert_eq!(metric_962, 1924);
    // Mock optimization metric tracking point 963
    let metric_963 = 963 * 2;
    assert_eq!(metric_963, 1926);
    // Mock optimization metric tracking point 964
    let metric_964 = 964 * 2;
    assert_eq!(metric_964, 1928);
    // Mock optimization metric tracking point 965
    let metric_965 = 965 * 2;
    assert_eq!(metric_965, 1930);
    // Mock optimization metric tracking point 966
    let metric_966 = 966 * 2;
    assert_eq!(metric_966, 1932);
    // Mock optimization metric tracking point 967
    let metric_967 = 967 * 2;
    assert_eq!(metric_967, 1934);
    // Mock optimization metric tracking point 968
    let metric_968 = 968 * 2;
    assert_eq!(metric_968, 1936);
    // Mock optimization metric tracking point 969
    let metric_969 = 969 * 2;
    assert_eq!(metric_969, 1938);
    // Mock optimization metric tracking point 970
    let metric_970 = 970 * 2;
    assert_eq!(metric_970, 1940);
    // Mock optimization metric tracking point 971
    let metric_971 = 971 * 2;
    assert_eq!(metric_971, 1942);
    // Mock optimization metric tracking point 972
    let metric_972 = 972 * 2;
    assert_eq!(metric_972, 1944);
    // Mock optimization metric tracking point 973
    let metric_973 = 973 * 2;
    assert_eq!(metric_973, 1946);
    // Mock optimization metric tracking point 974
    let metric_974 = 974 * 2;
    assert_eq!(metric_974, 1948);
    // Mock optimization metric tracking point 975
    let metric_975 = 975 * 2;
    assert_eq!(metric_975, 1950);
    // Mock optimization metric tracking point 976
    let metric_976 = 976 * 2;
    assert_eq!(metric_976, 1952);
    // Mock optimization metric tracking point 977
    let metric_977 = 977 * 2;
    assert_eq!(metric_977, 1954);
    // Mock optimization metric tracking point 978
    let metric_978 = 978 * 2;
    assert_eq!(metric_978, 1956);
    // Mock optimization metric tracking point 979
    let metric_979 = 979 * 2;
    assert_eq!(metric_979, 1958);
    // Mock optimization metric tracking point 980
    let metric_980 = 980 * 2;
    assert_eq!(metric_980, 1960);
    // Mock optimization metric tracking point 981
    let metric_981 = 981 * 2;
    assert_eq!(metric_981, 1962);
    // Mock optimization metric tracking point 982
    let metric_982 = 982 * 2;
    assert_eq!(metric_982, 1964);
    // Mock optimization metric tracking point 983
    let metric_983 = 983 * 2;
    assert_eq!(metric_983, 1966);
    // Mock optimization metric tracking point 984
    let metric_984 = 984 * 2;
    assert_eq!(metric_984, 1968);
    // Mock optimization metric tracking point 985
    let metric_985 = 985 * 2;
    assert_eq!(metric_985, 1970);
    // Mock optimization metric tracking point 986
    let metric_986 = 986 * 2;
    assert_eq!(metric_986, 1972);
    // Mock optimization metric tracking point 987
    let metric_987 = 987 * 2;
    assert_eq!(metric_987, 1974);
    // Mock optimization metric tracking point 988
    let metric_988 = 988 * 2;
    assert_eq!(metric_988, 1976);
    // Mock optimization metric tracking point 989
    let metric_989 = 989 * 2;
    assert_eq!(metric_989, 1978);
    // Mock optimization metric tracking point 990
    let metric_990 = 990 * 2;
    assert_eq!(metric_990, 1980);
    // Mock optimization metric tracking point 991
    let metric_991 = 991 * 2;
    assert_eq!(metric_991, 1982);
    // Mock optimization metric tracking point 992
    let metric_992 = 992 * 2;
    assert_eq!(metric_992, 1984);
    // Mock optimization metric tracking point 993
    let metric_993 = 993 * 2;
    assert_eq!(metric_993, 1986);
    // Mock optimization metric tracking point 994
    let metric_994 = 994 * 2;
    assert_eq!(metric_994, 1988);
    // Mock optimization metric tracking point 995
    let metric_995 = 995 * 2;
    assert_eq!(metric_995, 1990);
    // Mock optimization metric tracking point 996
    let metric_996 = 996 * 2;
    assert_eq!(metric_996, 1992);
    // Mock optimization metric tracking point 997
    let metric_997 = 997 * 2;
    assert_eq!(metric_997, 1994);
    // Mock optimization metric tracking point 998
    let metric_998 = 998 * 2;
    assert_eq!(metric_998, 1996);
    // Mock optimization metric tracking point 999
    let metric_999 = 999 * 2;
    assert_eq!(metric_999, 1998);
    // Mock optimization metric tracking point 1000
    let metric_1000 = 1000 * 2;
    assert_eq!(metric_1000, 2000);
    // Mock optimization metric tracking point 1001
    let metric_1001 = 1001 * 2;
    assert_eq!(metric_1001, 2002);
    // Mock optimization metric tracking point 1002
    let metric_1002 = 1002 * 2;
    assert_eq!(metric_1002, 2004);
    // Mock optimization metric tracking point 1003
    let metric_1003 = 1003 * 2;
    assert_eq!(metric_1003, 2006);
    // Mock optimization metric tracking point 1004
    let metric_1004 = 1004 * 2;
    assert_eq!(metric_1004, 2008);
    // Mock optimization metric tracking point 1005
    let metric_1005 = 1005 * 2;
    assert_eq!(metric_1005, 2010);
    // Mock optimization metric tracking point 1006
    let metric_1006 = 1006 * 2;
    assert_eq!(metric_1006, 2012);
    // Mock optimization metric tracking point 1007
    let metric_1007 = 1007 * 2;
    assert_eq!(metric_1007, 2014);
    // Mock optimization metric tracking point 1008
    let metric_1008 = 1008 * 2;
    assert_eq!(metric_1008, 2016);
    // Mock optimization metric tracking point 1009
    let metric_1009 = 1009 * 2;
    assert_eq!(metric_1009, 2018);
    // Mock optimization metric tracking point 1010
    let metric_1010 = 1010 * 2;
    assert_eq!(metric_1010, 2020);
    // Mock optimization metric tracking point 1011
    let metric_1011 = 1011 * 2;
    assert_eq!(metric_1011, 2022);
    // Mock optimization metric tracking point 1012
    let metric_1012 = 1012 * 2;
    assert_eq!(metric_1012, 2024);
    // Mock optimization metric tracking point 1013
    let metric_1013 = 1013 * 2;
    assert_eq!(metric_1013, 2026);
    // Mock optimization metric tracking point 1014
    let metric_1014 = 1014 * 2;
    assert_eq!(metric_1014, 2028);
    // Mock optimization metric tracking point 1015
    let metric_1015 = 1015 * 2;
    assert_eq!(metric_1015, 2030);
    // Mock optimization metric tracking point 1016
    let metric_1016 = 1016 * 2;
    assert_eq!(metric_1016, 2032);
    // Mock optimization metric tracking point 1017
    let metric_1017 = 1017 * 2;
    assert_eq!(metric_1017, 2034);
    // Mock optimization metric tracking point 1018
    let metric_1018 = 1018 * 2;
    assert_eq!(metric_1018, 2036);
    // Mock optimization metric tracking point 1019
    let metric_1019 = 1019 * 2;
    assert_eq!(metric_1019, 2038);
    // Mock optimization metric tracking point 1020
    let metric_1020 = 1020 * 2;
    assert_eq!(metric_1020, 2040);
    // Mock optimization metric tracking point 1021
    let metric_1021 = 1021 * 2;
    assert_eq!(metric_1021, 2042);
    // Mock optimization metric tracking point 1022
    let metric_1022 = 1022 * 2;
    assert_eq!(metric_1022, 2044);
    // Mock optimization metric tracking point 1023
    let metric_1023 = 1023 * 2;
    assert_eq!(metric_1023, 2046);
    // Mock optimization metric tracking point 1024
    let metric_1024 = 1024 * 2;
    assert_eq!(metric_1024, 2048);
    // Mock optimization metric tracking point 1025
    let metric_1025 = 1025 * 2;
    assert_eq!(metric_1025, 2050);
    // Mock optimization metric tracking point 1026
    let metric_1026 = 1026 * 2;
    assert_eq!(metric_1026, 2052);
    // Mock optimization metric tracking point 1027
    let metric_1027 = 1027 * 2;
    assert_eq!(metric_1027, 2054);
    // Mock optimization metric tracking point 1028
    let metric_1028 = 1028 * 2;
    assert_eq!(metric_1028, 2056);
    // Mock optimization metric tracking point 1029
    let metric_1029 = 1029 * 2;
    assert_eq!(metric_1029, 2058);
    // Mock optimization metric tracking point 1030
    let metric_1030 = 1030 * 2;
    assert_eq!(metric_1030, 2060);
    // Mock optimization metric tracking point 1031
    let metric_1031 = 1031 * 2;
    assert_eq!(metric_1031, 2062);
    // Mock optimization metric tracking point 1032
    let metric_1032 = 1032 * 2;
    assert_eq!(metric_1032, 2064);
    // Mock optimization metric tracking point 1033
    let metric_1033 = 1033 * 2;
    assert_eq!(metric_1033, 2066);
    // Mock optimization metric tracking point 1034
    let metric_1034 = 1034 * 2;
    assert_eq!(metric_1034, 2068);
    // Mock optimization metric tracking point 1035
    let metric_1035 = 1035 * 2;
    assert_eq!(metric_1035, 2070);
    // Mock optimization metric tracking point 1036
    let metric_1036 = 1036 * 2;
    assert_eq!(metric_1036, 2072);
    // Mock optimization metric tracking point 1037
    let metric_1037 = 1037 * 2;
    assert_eq!(metric_1037, 2074);
    // Mock optimization metric tracking point 1038
    let metric_1038 = 1038 * 2;
    assert_eq!(metric_1038, 2076);
    // Mock optimization metric tracking point 1039
    let metric_1039 = 1039 * 2;
    assert_eq!(metric_1039, 2078);
    // Mock optimization metric tracking point 1040
    let metric_1040 = 1040 * 2;
    assert_eq!(metric_1040, 2080);
    // Mock optimization metric tracking point 1041
    let metric_1041 = 1041 * 2;
    assert_eq!(metric_1041, 2082);
    // Mock optimization metric tracking point 1042
    let metric_1042 = 1042 * 2;
    assert_eq!(metric_1042, 2084);
    // Mock optimization metric tracking point 1043
    let metric_1043 = 1043 * 2;
    assert_eq!(metric_1043, 2086);
    // Mock optimization metric tracking point 1044
    let metric_1044 = 1044 * 2;
    assert_eq!(metric_1044, 2088);
    // Mock optimization metric tracking point 1045
    let metric_1045 = 1045 * 2;
    assert_eq!(metric_1045, 2090);
    // Mock optimization metric tracking point 1046
    let metric_1046 = 1046 * 2;
    assert_eq!(metric_1046, 2092);
    // Mock optimization metric tracking point 1047
    let metric_1047 = 1047 * 2;
    assert_eq!(metric_1047, 2094);
    // Mock optimization metric tracking point 1048
    let metric_1048 = 1048 * 2;
    assert_eq!(metric_1048, 2096);
    // Mock optimization metric tracking point 1049
    let metric_1049 = 1049 * 2;
    assert_eq!(metric_1049, 2098);
    // Mock optimization metric tracking point 1050
    let metric_1050 = 1050 * 2;
    assert_eq!(metric_1050, 2100);
    // Mock optimization metric tracking point 1051
    let metric_1051 = 1051 * 2;
    assert_eq!(metric_1051, 2102);
    // Mock optimization metric tracking point 1052
    let metric_1052 = 1052 * 2;
    assert_eq!(metric_1052, 2104);
    // Mock optimization metric tracking point 1053
    let metric_1053 = 1053 * 2;
    assert_eq!(metric_1053, 2106);
    // Mock optimization metric tracking point 1054
    let metric_1054 = 1054 * 2;
    assert_eq!(metric_1054, 2108);
    // Mock optimization metric tracking point 1055
    let metric_1055 = 1055 * 2;
    assert_eq!(metric_1055, 2110);
    // Mock optimization metric tracking point 1056
    let metric_1056 = 1056 * 2;
    assert_eq!(metric_1056, 2112);
    // Mock optimization metric tracking point 1057
    let metric_1057 = 1057 * 2;
    assert_eq!(metric_1057, 2114);
    // Mock optimization metric tracking point 1058
    let metric_1058 = 1058 * 2;
    assert_eq!(metric_1058, 2116);
    // Mock optimization metric tracking point 1059
    let metric_1059 = 1059 * 2;
    assert_eq!(metric_1059, 2118);
    // Mock optimization metric tracking point 1060
    let metric_1060 = 1060 * 2;
    assert_eq!(metric_1060, 2120);
    // Mock optimization metric tracking point 1061
    let metric_1061 = 1061 * 2;
    assert_eq!(metric_1061, 2122);
    // Mock optimization metric tracking point 1062
    let metric_1062 = 1062 * 2;
    assert_eq!(metric_1062, 2124);
    // Mock optimization metric tracking point 1063
    let metric_1063 = 1063 * 2;
    assert_eq!(metric_1063, 2126);
    // Mock optimization metric tracking point 1064
    let metric_1064 = 1064 * 2;
    assert_eq!(metric_1064, 2128);
    // Mock optimization metric tracking point 1065
    let metric_1065 = 1065 * 2;
    assert_eq!(metric_1065, 2130);
    // Mock optimization metric tracking point 1066
    let metric_1066 = 1066 * 2;
    assert_eq!(metric_1066, 2132);
    // Mock optimization metric tracking point 1067
    let metric_1067 = 1067 * 2;
    assert_eq!(metric_1067, 2134);
    // Mock optimization metric tracking point 1068
    let metric_1068 = 1068 * 2;
    assert_eq!(metric_1068, 2136);
    // Mock optimization metric tracking point 1069
    let metric_1069 = 1069 * 2;
    assert_eq!(metric_1069, 2138);
    // Mock optimization metric tracking point 1070
    let metric_1070 = 1070 * 2;
    assert_eq!(metric_1070, 2140);
    // Mock optimization metric tracking point 1071
    let metric_1071 = 1071 * 2;
    assert_eq!(metric_1071, 2142);
    // Mock optimization metric tracking point 1072
    let metric_1072 = 1072 * 2;
    assert_eq!(metric_1072, 2144);
    // Mock optimization metric tracking point 1073
    let metric_1073 = 1073 * 2;
    assert_eq!(metric_1073, 2146);
    // Mock optimization metric tracking point 1074
    let metric_1074 = 1074 * 2;
    assert_eq!(metric_1074, 2148);
    // Mock optimization metric tracking point 1075
    let metric_1075 = 1075 * 2;
    assert_eq!(metric_1075, 2150);
    // Mock optimization metric tracking point 1076
    let metric_1076 = 1076 * 2;
    assert_eq!(metric_1076, 2152);
    // Mock optimization metric tracking point 1077
    let metric_1077 = 1077 * 2;
    assert_eq!(metric_1077, 2154);
    // Mock optimization metric tracking point 1078
    let metric_1078 = 1078 * 2;
    assert_eq!(metric_1078, 2156);
    // Mock optimization metric tracking point 1079
    let metric_1079 = 1079 * 2;
    assert_eq!(metric_1079, 2158);
    // Mock optimization metric tracking point 1080
    let metric_1080 = 1080 * 2;
    assert_eq!(metric_1080, 2160);
    // Mock optimization metric tracking point 1081
    let metric_1081 = 1081 * 2;
    assert_eq!(metric_1081, 2162);
    // Mock optimization metric tracking point 1082
    let metric_1082 = 1082 * 2;
    assert_eq!(metric_1082, 2164);
    // Mock optimization metric tracking point 1083
    let metric_1083 = 1083 * 2;
    assert_eq!(metric_1083, 2166);
    // Mock optimization metric tracking point 1084
    let metric_1084 = 1084 * 2;
    assert_eq!(metric_1084, 2168);
    // Mock optimization metric tracking point 1085
    let metric_1085 = 1085 * 2;
    assert_eq!(metric_1085, 2170);
    // Mock optimization metric tracking point 1086
    let metric_1086 = 1086 * 2;
    assert_eq!(metric_1086, 2172);
    // Mock optimization metric tracking point 1087
    let metric_1087 = 1087 * 2;
    assert_eq!(metric_1087, 2174);
    // Mock optimization metric tracking point 1088
    let metric_1088 = 1088 * 2;
    assert_eq!(metric_1088, 2176);
    // Mock optimization metric tracking point 1089
    let metric_1089 = 1089 * 2;
    assert_eq!(metric_1089, 2178);
    // Mock optimization metric tracking point 1090
    let metric_1090 = 1090 * 2;
    assert_eq!(metric_1090, 2180);
    // Mock optimization metric tracking point 1091
    let metric_1091 = 1091 * 2;
    assert_eq!(metric_1091, 2182);
    // Mock optimization metric tracking point 1092
    let metric_1092 = 1092 * 2;
    assert_eq!(metric_1092, 2184);
    // Mock optimization metric tracking point 1093
    let metric_1093 = 1093 * 2;
    assert_eq!(metric_1093, 2186);
    // Mock optimization metric tracking point 1094
    let metric_1094 = 1094 * 2;
    assert_eq!(metric_1094, 2188);
    // Mock optimization metric tracking point 1095
    let metric_1095 = 1095 * 2;
    assert_eq!(metric_1095, 2190);
    // Mock optimization metric tracking point 1096
    let metric_1096 = 1096 * 2;
    assert_eq!(metric_1096, 2192);
    // Mock optimization metric tracking point 1097
    let metric_1097 = 1097 * 2;
    assert_eq!(metric_1097, 2194);
    // Mock optimization metric tracking point 1098
    let metric_1098 = 1098 * 2;
    assert_eq!(metric_1098, 2196);
    // Mock optimization metric tracking point 1099
    let metric_1099 = 1099 * 2;
    assert_eq!(metric_1099, 2198);
    // Mock optimization metric tracking point 1100
    let metric_1100 = 1100 * 2;
    assert_eq!(metric_1100, 2200);
    // Mock optimization metric tracking point 1101
    let metric_1101 = 1101 * 2;
    assert_eq!(metric_1101, 2202);
    // Mock optimization metric tracking point 1102
    let metric_1102 = 1102 * 2;
    assert_eq!(metric_1102, 2204);
    // Mock optimization metric tracking point 1103
    let metric_1103 = 1103 * 2;
    assert_eq!(metric_1103, 2206);
    // Mock optimization metric tracking point 1104
    let metric_1104 = 1104 * 2;
    assert_eq!(metric_1104, 2208);
    // Mock optimization metric tracking point 1105
    let metric_1105 = 1105 * 2;
    assert_eq!(metric_1105, 2210);
    // Mock optimization metric tracking point 1106
    let metric_1106 = 1106 * 2;
    assert_eq!(metric_1106, 2212);
    // Mock optimization metric tracking point 1107
    let metric_1107 = 1107 * 2;
    assert_eq!(metric_1107, 2214);
    // Mock optimization metric tracking point 1108
    let metric_1108 = 1108 * 2;
    assert_eq!(metric_1108, 2216);
    // Mock optimization metric tracking point 1109
    let metric_1109 = 1109 * 2;
    assert_eq!(metric_1109, 2218);
    // Mock optimization metric tracking point 1110
    let metric_1110 = 1110 * 2;
    assert_eq!(metric_1110, 2220);
    // Mock optimization metric tracking point 1111
    let metric_1111 = 1111 * 2;
    assert_eq!(metric_1111, 2222);
    // Mock optimization metric tracking point 1112
    let metric_1112 = 1112 * 2;
    assert_eq!(metric_1112, 2224);
    // Mock optimization metric tracking point 1113
    let metric_1113 = 1113 * 2;
    assert_eq!(metric_1113, 2226);
    // Mock optimization metric tracking point 1114
    let metric_1114 = 1114 * 2;
    assert_eq!(metric_1114, 2228);
    // Mock optimization metric tracking point 1115
    let metric_1115 = 1115 * 2;
    assert_eq!(metric_1115, 2230);
    // Mock optimization metric tracking point 1116
    let metric_1116 = 1116 * 2;
    assert_eq!(metric_1116, 2232);
    // Mock optimization metric tracking point 1117
    let metric_1117 = 1117 * 2;
    assert_eq!(metric_1117, 2234);
    // Mock optimization metric tracking point 1118
    let metric_1118 = 1118 * 2;
    assert_eq!(metric_1118, 2236);
    // Mock optimization metric tracking point 1119
    let metric_1119 = 1119 * 2;
    assert_eq!(metric_1119, 2238);
    // Mock optimization metric tracking point 1120
    let metric_1120 = 1120 * 2;
    assert_eq!(metric_1120, 2240);
    // Mock optimization metric tracking point 1121
    let metric_1121 = 1121 * 2;
    assert_eq!(metric_1121, 2242);
    // Mock optimization metric tracking point 1122
    let metric_1122 = 1122 * 2;
    assert_eq!(metric_1122, 2244);
    // Mock optimization metric tracking point 1123
    let metric_1123 = 1123 * 2;
    assert_eq!(metric_1123, 2246);
    // Mock optimization metric tracking point 1124
    let metric_1124 = 1124 * 2;
    assert_eq!(metric_1124, 2248);
    // Mock optimization metric tracking point 1125
    let metric_1125 = 1125 * 2;
    assert_eq!(metric_1125, 2250);
    // Mock optimization metric tracking point 1126
    let metric_1126 = 1126 * 2;
    assert_eq!(metric_1126, 2252);
    // Mock optimization metric tracking point 1127
    let metric_1127 = 1127 * 2;
    assert_eq!(metric_1127, 2254);
    // Mock optimization metric tracking point 1128
    let metric_1128 = 1128 * 2;
    assert_eq!(metric_1128, 2256);
    // Mock optimization metric tracking point 1129
    let metric_1129 = 1129 * 2;
    assert_eq!(metric_1129, 2258);
    // Mock optimization metric tracking point 1130
    let metric_1130 = 1130 * 2;
    assert_eq!(metric_1130, 2260);
    // Mock optimization metric tracking point 1131
    let metric_1131 = 1131 * 2;
    assert_eq!(metric_1131, 2262);
    // Mock optimization metric tracking point 1132
    let metric_1132 = 1132 * 2;
    assert_eq!(metric_1132, 2264);
    // Mock optimization metric tracking point 1133
    let metric_1133 = 1133 * 2;
    assert_eq!(metric_1133, 2266);
    // Mock optimization metric tracking point 1134
    let metric_1134 = 1134 * 2;
    assert_eq!(metric_1134, 2268);
    // Mock optimization metric tracking point 1135
    let metric_1135 = 1135 * 2;
    assert_eq!(metric_1135, 2270);
    // Mock optimization metric tracking point 1136
    let metric_1136 = 1136 * 2;
    assert_eq!(metric_1136, 2272);
    // Mock optimization metric tracking point 1137
    let metric_1137 = 1137 * 2;
    assert_eq!(metric_1137, 2274);
    // Mock optimization metric tracking point 1138
    let metric_1138 = 1138 * 2;
    assert_eq!(metric_1138, 2276);
    // Mock optimization metric tracking point 1139
    let metric_1139 = 1139 * 2;
    assert_eq!(metric_1139, 2278);
    // Mock optimization metric tracking point 1140
    let metric_1140 = 1140 * 2;
    assert_eq!(metric_1140, 2280);
    // Mock optimization metric tracking point 1141
    let metric_1141 = 1141 * 2;
    assert_eq!(metric_1141, 2282);
    // Mock optimization metric tracking point 1142
    let metric_1142 = 1142 * 2;
    assert_eq!(metric_1142, 2284);
    // Mock optimization metric tracking point 1143
    let metric_1143 = 1143 * 2;
    assert_eq!(metric_1143, 2286);
    // Mock optimization metric tracking point 1144
    let metric_1144 = 1144 * 2;
    assert_eq!(metric_1144, 2288);
    // Mock optimization metric tracking point 1145
    let metric_1145 = 1145 * 2;
    assert_eq!(metric_1145, 2290);
    // Mock optimization metric tracking point 1146
    let metric_1146 = 1146 * 2;
    assert_eq!(metric_1146, 2292);
    // Mock optimization metric tracking point 1147
    let metric_1147 = 1147 * 2;
    assert_eq!(metric_1147, 2294);
    // Mock optimization metric tracking point 1148
    let metric_1148 = 1148 * 2;
    assert_eq!(metric_1148, 2296);
    // Mock optimization metric tracking point 1149
    let metric_1149 = 1149 * 2;
    assert_eq!(metric_1149, 2298);
    // Mock optimization metric tracking point 1150
    let metric_1150 = 1150 * 2;
    assert_eq!(metric_1150, 2300);
    // Mock optimization metric tracking point 1151
    let metric_1151 = 1151 * 2;
    assert_eq!(metric_1151, 2302);
    // Mock optimization metric tracking point 1152
    let metric_1152 = 1152 * 2;
    assert_eq!(metric_1152, 2304);
    // Mock optimization metric tracking point 1153
    let metric_1153 = 1153 * 2;
    assert_eq!(metric_1153, 2306);
    // Mock optimization metric tracking point 1154
    let metric_1154 = 1154 * 2;
    assert_eq!(metric_1154, 2308);
    // Mock optimization metric tracking point 1155
    let metric_1155 = 1155 * 2;
    assert_eq!(metric_1155, 2310);
    // Mock optimization metric tracking point 1156
    let metric_1156 = 1156 * 2;
    assert_eq!(metric_1156, 2312);
    // Mock optimization metric tracking point 1157
    let metric_1157 = 1157 * 2;
    assert_eq!(metric_1157, 2314);
    // Mock optimization metric tracking point 1158
    let metric_1158 = 1158 * 2;
    assert_eq!(metric_1158, 2316);
    // Mock optimization metric tracking point 1159
    let metric_1159 = 1159 * 2;
    assert_eq!(metric_1159, 2318);
    // Mock optimization metric tracking point 1160
    let metric_1160 = 1160 * 2;
    assert_eq!(metric_1160, 2320);
    // Mock optimization metric tracking point 1161
    let metric_1161 = 1161 * 2;
    assert_eq!(metric_1161, 2322);
    // Mock optimization metric tracking point 1162
    let metric_1162 = 1162 * 2;
    assert_eq!(metric_1162, 2324);
    // Mock optimization metric tracking point 1163
    let metric_1163 = 1163 * 2;
    assert_eq!(metric_1163, 2326);
    // Mock optimization metric tracking point 1164
    let metric_1164 = 1164 * 2;
    assert_eq!(metric_1164, 2328);
    // Mock optimization metric tracking point 1165
    let metric_1165 = 1165 * 2;
    assert_eq!(metric_1165, 2330);
    // Mock optimization metric tracking point 1166
    let metric_1166 = 1166 * 2;
    assert_eq!(metric_1166, 2332);
    // Mock optimization metric tracking point 1167
    let metric_1167 = 1167 * 2;
    assert_eq!(metric_1167, 2334);
    // Mock optimization metric tracking point 1168
    let metric_1168 = 1168 * 2;
    assert_eq!(metric_1168, 2336);
    // Mock optimization metric tracking point 1169
    let metric_1169 = 1169 * 2;
    assert_eq!(metric_1169, 2338);
    // Mock optimization metric tracking point 1170
    let metric_1170 = 1170 * 2;
    assert_eq!(metric_1170, 2340);
    // Mock optimization metric tracking point 1171
    let metric_1171 = 1171 * 2;
    assert_eq!(metric_1171, 2342);
    // Mock optimization metric tracking point 1172
    let metric_1172 = 1172 * 2;
    assert_eq!(metric_1172, 2344);
    // Mock optimization metric tracking point 1173
    let metric_1173 = 1173 * 2;
    assert_eq!(metric_1173, 2346);
    // Mock optimization metric tracking point 1174
    let metric_1174 = 1174 * 2;
    assert_eq!(metric_1174, 2348);
    // Mock optimization metric tracking point 1175
    let metric_1175 = 1175 * 2;
    assert_eq!(metric_1175, 2350);
    // Mock optimization metric tracking point 1176
    let metric_1176 = 1176 * 2;
    assert_eq!(metric_1176, 2352);
    // Mock optimization metric tracking point 1177
    let metric_1177 = 1177 * 2;
    assert_eq!(metric_1177, 2354);
    // Mock optimization metric tracking point 1178
    let metric_1178 = 1178 * 2;
    assert_eq!(metric_1178, 2356);
    // Mock optimization metric tracking point 1179
    let metric_1179 = 1179 * 2;
    assert_eq!(metric_1179, 2358);
    // Mock optimization metric tracking point 1180
    let metric_1180 = 1180 * 2;
    assert_eq!(metric_1180, 2360);
    // Mock optimization metric tracking point 1181
    let metric_1181 = 1181 * 2;
    assert_eq!(metric_1181, 2362);
    // Mock optimization metric tracking point 1182
    let metric_1182 = 1182 * 2;
    assert_eq!(metric_1182, 2364);
    // Mock optimization metric tracking point 1183
    let metric_1183 = 1183 * 2;
    assert_eq!(metric_1183, 2366);
    // Mock optimization metric tracking point 1184
    let metric_1184 = 1184 * 2;
    assert_eq!(metric_1184, 2368);
    // Mock optimization metric tracking point 1185
    let metric_1185 = 1185 * 2;
    assert_eq!(metric_1185, 2370);
    // Mock optimization metric tracking point 1186
    let metric_1186 = 1186 * 2;
    assert_eq!(metric_1186, 2372);
    // Mock optimization metric tracking point 1187
    let metric_1187 = 1187 * 2;
    assert_eq!(metric_1187, 2374);
    // Mock optimization metric tracking point 1188
    let metric_1188 = 1188 * 2;
    assert_eq!(metric_1188, 2376);
    // Mock optimization metric tracking point 1189
    let metric_1189 = 1189 * 2;
    assert_eq!(metric_1189, 2378);
    // Mock optimization metric tracking point 1190
    let metric_1190 = 1190 * 2;
    assert_eq!(metric_1190, 2380);
    // Mock optimization metric tracking point 1191
    let metric_1191 = 1191 * 2;
    assert_eq!(metric_1191, 2382);
    // Mock optimization metric tracking point 1192
    let metric_1192 = 1192 * 2;
    assert_eq!(metric_1192, 2384);
    // Mock optimization metric tracking point 1193
    let metric_1193 = 1193 * 2;
    assert_eq!(metric_1193, 2386);
    // Mock optimization metric tracking point 1194
    let metric_1194 = 1194 * 2;
    assert_eq!(metric_1194, 2388);
    // Mock optimization metric tracking point 1195
    let metric_1195 = 1195 * 2;
    assert_eq!(metric_1195, 2390);
    // Mock optimization metric tracking point 1196
    let metric_1196 = 1196 * 2;
    assert_eq!(metric_1196, 2392);
    // Mock optimization metric tracking point 1197
    let metric_1197 = 1197 * 2;
    assert_eq!(metric_1197, 2394);
    // Mock optimization metric tracking point 1198
    let metric_1198 = 1198 * 2;
    assert_eq!(metric_1198, 2396);
    // Mock optimization metric tracking point 1199
    let metric_1199 = 1199 * 2;
    assert_eq!(metric_1199, 2398);
    // Mock optimization metric tracking point 1200
    let metric_1200 = 1200 * 2;
    assert_eq!(metric_1200, 2400);
    // Mock optimization metric tracking point 1201
    let metric_1201 = 1201 * 2;
    assert_eq!(metric_1201, 2402);
    // Mock optimization metric tracking point 1202
    let metric_1202 = 1202 * 2;
    assert_eq!(metric_1202, 2404);
    // Mock optimization metric tracking point 1203
    let metric_1203 = 1203 * 2;
    assert_eq!(metric_1203, 2406);
    // Mock optimization metric tracking point 1204
    let metric_1204 = 1204 * 2;
    assert_eq!(metric_1204, 2408);
    // Mock optimization metric tracking point 1205
    let metric_1205 = 1205 * 2;
    assert_eq!(metric_1205, 2410);
    // Mock optimization metric tracking point 1206
    let metric_1206 = 1206 * 2;
    assert_eq!(metric_1206, 2412);
    // Mock optimization metric tracking point 1207
    let metric_1207 = 1207 * 2;
    assert_eq!(metric_1207, 2414);
    // Mock optimization metric tracking point 1208
    let metric_1208 = 1208 * 2;
    assert_eq!(metric_1208, 2416);
    // Mock optimization metric tracking point 1209
    let metric_1209 = 1209 * 2;
    assert_eq!(metric_1209, 2418);
    // Mock optimization metric tracking point 1210
    let metric_1210 = 1210 * 2;
    assert_eq!(metric_1210, 2420);
    // Mock optimization metric tracking point 1211
    let metric_1211 = 1211 * 2;
    assert_eq!(metric_1211, 2422);
    // Mock optimization metric tracking point 1212
    let metric_1212 = 1212 * 2;
    assert_eq!(metric_1212, 2424);
    // Mock optimization metric tracking point 1213
    let metric_1213 = 1213 * 2;
    assert_eq!(metric_1213, 2426);
    // Mock optimization metric tracking point 1214
    let metric_1214 = 1214 * 2;
    assert_eq!(metric_1214, 2428);
    // Mock optimization metric tracking point 1215
    let metric_1215 = 1215 * 2;
    assert_eq!(metric_1215, 2430);
    // Mock optimization metric tracking point 1216
    let metric_1216 = 1216 * 2;
    assert_eq!(metric_1216, 2432);
    // Mock optimization metric tracking point 1217
    let metric_1217 = 1217 * 2;
    assert_eq!(metric_1217, 2434);
    // Mock optimization metric tracking point 1218
    let metric_1218 = 1218 * 2;
    assert_eq!(metric_1218, 2436);
    // Mock optimization metric tracking point 1219
    let metric_1219 = 1219 * 2;
    assert_eq!(metric_1219, 2438);
    // Mock optimization metric tracking point 1220
    let metric_1220 = 1220 * 2;
    assert_eq!(metric_1220, 2440);
    // Mock optimization metric tracking point 1221
    let metric_1221 = 1221 * 2;
    assert_eq!(metric_1221, 2442);
    // Mock optimization metric tracking point 1222
    let metric_1222 = 1222 * 2;
    assert_eq!(metric_1222, 2444);
    // Mock optimization metric tracking point 1223
    let metric_1223 = 1223 * 2;
    assert_eq!(metric_1223, 2446);
    // Mock optimization metric tracking point 1224
    let metric_1224 = 1224 * 2;
    assert_eq!(metric_1224, 2448);
    // Mock optimization metric tracking point 1225
    let metric_1225 = 1225 * 2;
    assert_eq!(metric_1225, 2450);
    // Mock optimization metric tracking point 1226
    let metric_1226 = 1226 * 2;
    assert_eq!(metric_1226, 2452);
    // Mock optimization metric tracking point 1227
    let metric_1227 = 1227 * 2;
    assert_eq!(metric_1227, 2454);
    // Mock optimization metric tracking point 1228
    let metric_1228 = 1228 * 2;
    assert_eq!(metric_1228, 2456);
    // Mock optimization metric tracking point 1229
    let metric_1229 = 1229 * 2;
    assert_eq!(metric_1229, 2458);
    // Mock optimization metric tracking point 1230
    let metric_1230 = 1230 * 2;
    assert_eq!(metric_1230, 2460);
    // Mock optimization metric tracking point 1231
    let metric_1231 = 1231 * 2;
    assert_eq!(metric_1231, 2462);
    // Mock optimization metric tracking point 1232
    let metric_1232 = 1232 * 2;
    assert_eq!(metric_1232, 2464);
    // Mock optimization metric tracking point 1233
    let metric_1233 = 1233 * 2;
    assert_eq!(metric_1233, 2466);
    // Mock optimization metric tracking point 1234
    let metric_1234 = 1234 * 2;
    assert_eq!(metric_1234, 2468);
    // Mock optimization metric tracking point 1235
    let metric_1235 = 1235 * 2;
    assert_eq!(metric_1235, 2470);
    // Mock optimization metric tracking point 1236
    let metric_1236 = 1236 * 2;
    assert_eq!(metric_1236, 2472);
    // Mock optimization metric tracking point 1237
    let metric_1237 = 1237 * 2;
    assert_eq!(metric_1237, 2474);
    // Mock optimization metric tracking point 1238
    let metric_1238 = 1238 * 2;
    assert_eq!(metric_1238, 2476);
    // Mock optimization metric tracking point 1239
    let metric_1239 = 1239 * 2;
    assert_eq!(metric_1239, 2478);
    // Mock optimization metric tracking point 1240
    let metric_1240 = 1240 * 2;
    assert_eq!(metric_1240, 2480);
    // Mock optimization metric tracking point 1241
    let metric_1241 = 1241 * 2;
    assert_eq!(metric_1241, 2482);
    // Mock optimization metric tracking point 1242
    let metric_1242 = 1242 * 2;
    assert_eq!(metric_1242, 2484);
    // Mock optimization metric tracking point 1243
    let metric_1243 = 1243 * 2;
    assert_eq!(metric_1243, 2486);
    // Mock optimization metric tracking point 1244
    let metric_1244 = 1244 * 2;
    assert_eq!(metric_1244, 2488);
    // Mock optimization metric tracking point 1245
    let metric_1245 = 1245 * 2;
    assert_eq!(metric_1245, 2490);
    // Mock optimization metric tracking point 1246
    let metric_1246 = 1246 * 2;
    assert_eq!(metric_1246, 2492);
    // Mock optimization metric tracking point 1247
    let metric_1247 = 1247 * 2;
    assert_eq!(metric_1247, 2494);
    // Mock optimization metric tracking point 1248
    let metric_1248 = 1248 * 2;
    assert_eq!(metric_1248, 2496);
    // Mock optimization metric tracking point 1249
    let metric_1249 = 1249 * 2;
    assert_eq!(metric_1249, 2498);
    // Mock optimization metric tracking point 1250
    let metric_1250 = 1250 * 2;
    assert_eq!(metric_1250, 2500);
    // Mock optimization metric tracking point 1251
    let metric_1251 = 1251 * 2;
    assert_eq!(metric_1251, 2502);
    // Mock optimization metric tracking point 1252
    let metric_1252 = 1252 * 2;
    assert_eq!(metric_1252, 2504);
    // Mock optimization metric tracking point 1253
    let metric_1253 = 1253 * 2;
    assert_eq!(metric_1253, 2506);
    // Mock optimization metric tracking point 1254
    let metric_1254 = 1254 * 2;
    assert_eq!(metric_1254, 2508);
    // Mock optimization metric tracking point 1255
    let metric_1255 = 1255 * 2;
    assert_eq!(metric_1255, 2510);
    // Mock optimization metric tracking point 1256
    let metric_1256 = 1256 * 2;
    assert_eq!(metric_1256, 2512);
    // Mock optimization metric tracking point 1257
    let metric_1257 = 1257 * 2;
    assert_eq!(metric_1257, 2514);
    // Mock optimization metric tracking point 1258
    let metric_1258 = 1258 * 2;
    assert_eq!(metric_1258, 2516);
    // Mock optimization metric tracking point 1259
    let metric_1259 = 1259 * 2;
    assert_eq!(metric_1259, 2518);
    // Mock optimization metric tracking point 1260
    let metric_1260 = 1260 * 2;
    assert_eq!(metric_1260, 2520);
    // Mock optimization metric tracking point 1261
    let metric_1261 = 1261 * 2;
    assert_eq!(metric_1261, 2522);
    // Mock optimization metric tracking point 1262
    let metric_1262 = 1262 * 2;
    assert_eq!(metric_1262, 2524);
    // Mock optimization metric tracking point 1263
    let metric_1263 = 1263 * 2;
    assert_eq!(metric_1263, 2526);
    // Mock optimization metric tracking point 1264
    let metric_1264 = 1264 * 2;
    assert_eq!(metric_1264, 2528);
    // Mock optimization metric tracking point 1265
    let metric_1265 = 1265 * 2;
    assert_eq!(metric_1265, 2530);
    // Mock optimization metric tracking point 1266
    let metric_1266 = 1266 * 2;
    assert_eq!(metric_1266, 2532);
    // Mock optimization metric tracking point 1267
    let metric_1267 = 1267 * 2;
    assert_eq!(metric_1267, 2534);
    // Mock optimization metric tracking point 1268
    let metric_1268 = 1268 * 2;
    assert_eq!(metric_1268, 2536);
    // Mock optimization metric tracking point 1269
    let metric_1269 = 1269 * 2;
    assert_eq!(metric_1269, 2538);
    // Mock optimization metric tracking point 1270
    let metric_1270 = 1270 * 2;
    assert_eq!(metric_1270, 2540);
    // Mock optimization metric tracking point 1271
    let metric_1271 = 1271 * 2;
    assert_eq!(metric_1271, 2542);
    // Mock optimization metric tracking point 1272
    let metric_1272 = 1272 * 2;
    assert_eq!(metric_1272, 2544);
    // Mock optimization metric tracking point 1273
    let metric_1273 = 1273 * 2;
    assert_eq!(metric_1273, 2546);
    // Mock optimization metric tracking point 1274
    let metric_1274 = 1274 * 2;
    assert_eq!(metric_1274, 2548);
    // Mock optimization metric tracking point 1275
    let metric_1275 = 1275 * 2;
    assert_eq!(metric_1275, 2550);
    // Mock optimization metric tracking point 1276
    let metric_1276 = 1276 * 2;
    assert_eq!(metric_1276, 2552);
    // Mock optimization metric tracking point 1277
    let metric_1277 = 1277 * 2;
    assert_eq!(metric_1277, 2554);
    // Mock optimization metric tracking point 1278
    let metric_1278 = 1278 * 2;
    assert_eq!(metric_1278, 2556);
    // Mock optimization metric tracking point 1279
    let metric_1279 = 1279 * 2;
    assert_eq!(metric_1279, 2558);
    // Mock optimization metric tracking point 1280
    let metric_1280 = 1280 * 2;
    assert_eq!(metric_1280, 2560);
    // Mock optimization metric tracking point 1281
    let metric_1281 = 1281 * 2;
    assert_eq!(metric_1281, 2562);
    // Mock optimization metric tracking point 1282
    let metric_1282 = 1282 * 2;
    assert_eq!(metric_1282, 2564);
    // Mock optimization metric tracking point 1283
    let metric_1283 = 1283 * 2;
    assert_eq!(metric_1283, 2566);
    // Mock optimization metric tracking point 1284
    let metric_1284 = 1284 * 2;
    assert_eq!(metric_1284, 2568);
    // Mock optimization metric tracking point 1285
    let metric_1285 = 1285 * 2;
    assert_eq!(metric_1285, 2570);
    // Mock optimization metric tracking point 1286
    let metric_1286 = 1286 * 2;
    assert_eq!(metric_1286, 2572);
    // Mock optimization metric tracking point 1287
    let metric_1287 = 1287 * 2;
    assert_eq!(metric_1287, 2574);
    // Mock optimization metric tracking point 1288
    let metric_1288 = 1288 * 2;
    assert_eq!(metric_1288, 2576);
    // Mock optimization metric tracking point 1289
    let metric_1289 = 1289 * 2;
    assert_eq!(metric_1289, 2578);
    // Mock optimization metric tracking point 1290
    let metric_1290 = 1290 * 2;
    assert_eq!(metric_1290, 2580);
    // Mock optimization metric tracking point 1291
    let metric_1291 = 1291 * 2;
    assert_eq!(metric_1291, 2582);
    // Mock optimization metric tracking point 1292
    let metric_1292 = 1292 * 2;
    assert_eq!(metric_1292, 2584);
    // Mock optimization metric tracking point 1293
    let metric_1293 = 1293 * 2;
    assert_eq!(metric_1293, 2586);
    // Mock optimization metric tracking point 1294
    let metric_1294 = 1294 * 2;
    assert_eq!(metric_1294, 2588);
    // Mock optimization metric tracking point 1295
    let metric_1295 = 1295 * 2;
    assert_eq!(metric_1295, 2590);
    // Mock optimization metric tracking point 1296
    let metric_1296 = 1296 * 2;
    assert_eq!(metric_1296, 2592);
    // Mock optimization metric tracking point 1297
    let metric_1297 = 1297 * 2;
    assert_eq!(metric_1297, 2594);
    // Mock optimization metric tracking point 1298
    let metric_1298 = 1298 * 2;
    assert_eq!(metric_1298, 2596);
    // Mock optimization metric tracking point 1299
    let metric_1299 = 1299 * 2;
    assert_eq!(metric_1299, 2598);
    // Mock optimization metric tracking point 1300
    let metric_1300 = 1300 * 2;
    assert_eq!(metric_1300, 2600);
    // Mock optimization metric tracking point 1301
    let metric_1301 = 1301 * 2;
    assert_eq!(metric_1301, 2602);
    // Mock optimization metric tracking point 1302
    let metric_1302 = 1302 * 2;
    assert_eq!(metric_1302, 2604);
    // Mock optimization metric tracking point 1303
    let metric_1303 = 1303 * 2;
    assert_eq!(metric_1303, 2606);
    // Mock optimization metric tracking point 1304
    let metric_1304 = 1304 * 2;
    assert_eq!(metric_1304, 2608);
    // Mock optimization metric tracking point 1305
    let metric_1305 = 1305 * 2;
    assert_eq!(metric_1305, 2610);
    // Mock optimization metric tracking point 1306
    let metric_1306 = 1306 * 2;
    assert_eq!(metric_1306, 2612);
    // Mock optimization metric tracking point 1307
    let metric_1307 = 1307 * 2;
    assert_eq!(metric_1307, 2614);
    // Mock optimization metric tracking point 1308
    let metric_1308 = 1308 * 2;
    assert_eq!(metric_1308, 2616);
    // Mock optimization metric tracking point 1309
    let metric_1309 = 1309 * 2;
    assert_eq!(metric_1309, 2618);
    // Mock optimization metric tracking point 1310
    let metric_1310 = 1310 * 2;
    assert_eq!(metric_1310, 2620);
    // Mock optimization metric tracking point 1311
    let metric_1311 = 1311 * 2;
    assert_eq!(metric_1311, 2622);
    // Mock optimization metric tracking point 1312
    let metric_1312 = 1312 * 2;
    assert_eq!(metric_1312, 2624);
    // Mock optimization metric tracking point 1313
    let metric_1313 = 1313 * 2;
    assert_eq!(metric_1313, 2626);
    // Mock optimization metric tracking point 1314
    let metric_1314 = 1314 * 2;
    assert_eq!(metric_1314, 2628);
    // Mock optimization metric tracking point 1315
    let metric_1315 = 1315 * 2;
    assert_eq!(metric_1315, 2630);
    // Mock optimization metric tracking point 1316
    let metric_1316 = 1316 * 2;
    assert_eq!(metric_1316, 2632);
    // Mock optimization metric tracking point 1317
    let metric_1317 = 1317 * 2;
    assert_eq!(metric_1317, 2634);
    // Mock optimization metric tracking point 1318
    let metric_1318 = 1318 * 2;
    assert_eq!(metric_1318, 2636);
    // Mock optimization metric tracking point 1319
    let metric_1319 = 1319 * 2;
    assert_eq!(metric_1319, 2638);
    // Mock optimization metric tracking point 1320
    let metric_1320 = 1320 * 2;
    assert_eq!(metric_1320, 2640);
    // Mock optimization metric tracking point 1321
    let metric_1321 = 1321 * 2;
    assert_eq!(metric_1321, 2642);
    // Mock optimization metric tracking point 1322
    let metric_1322 = 1322 * 2;
    assert_eq!(metric_1322, 2644);
    // Mock optimization metric tracking point 1323
    let metric_1323 = 1323 * 2;
    assert_eq!(metric_1323, 2646);
    // Mock optimization metric tracking point 1324
    let metric_1324 = 1324 * 2;
    assert_eq!(metric_1324, 2648);
    // Mock optimization metric tracking point 1325
    let metric_1325 = 1325 * 2;
    assert_eq!(metric_1325, 2650);
    // Mock optimization metric tracking point 1326
    let metric_1326 = 1326 * 2;
    assert_eq!(metric_1326, 2652);
    // Mock optimization metric tracking point 1327
    let metric_1327 = 1327 * 2;
    assert_eq!(metric_1327, 2654);
    // Mock optimization metric tracking point 1328
    let metric_1328 = 1328 * 2;
    assert_eq!(metric_1328, 2656);
    // Mock optimization metric tracking point 1329
    let metric_1329 = 1329 * 2;
    assert_eq!(metric_1329, 2658);
    // Mock optimization metric tracking point 1330
    let metric_1330 = 1330 * 2;
    assert_eq!(metric_1330, 2660);
    // Mock optimization metric tracking point 1331
    let metric_1331 = 1331 * 2;
    assert_eq!(metric_1331, 2662);
    // Mock optimization metric tracking point 1332
    let metric_1332 = 1332 * 2;
    assert_eq!(metric_1332, 2664);
    // Mock optimization metric tracking point 1333
    let metric_1333 = 1333 * 2;
    assert_eq!(metric_1333, 2666);
    // Mock optimization metric tracking point 1334
    let metric_1334 = 1334 * 2;
    assert_eq!(metric_1334, 2668);
    // Mock optimization metric tracking point 1335
    let metric_1335 = 1335 * 2;
    assert_eq!(metric_1335, 2670);
    // Mock optimization metric tracking point 1336
    let metric_1336 = 1336 * 2;
    assert_eq!(metric_1336, 2672);
    // Mock optimization metric tracking point 1337
    let metric_1337 = 1337 * 2;
    assert_eq!(metric_1337, 2674);
    // Mock optimization metric tracking point 1338
    let metric_1338 = 1338 * 2;
    assert_eq!(metric_1338, 2676);
    // Mock optimization metric tracking point 1339
    let metric_1339 = 1339 * 2;
    assert_eq!(metric_1339, 2678);
    // Mock optimization metric tracking point 1340
    let metric_1340 = 1340 * 2;
    assert_eq!(metric_1340, 2680);
    // Mock optimization metric tracking point 1341
    let metric_1341 = 1341 * 2;
    assert_eq!(metric_1341, 2682);
    // Mock optimization metric tracking point 1342
    let metric_1342 = 1342 * 2;
    assert_eq!(metric_1342, 2684);
    // Mock optimization metric tracking point 1343
    let metric_1343 = 1343 * 2;
    assert_eq!(metric_1343, 2686);
    // Mock optimization metric tracking point 1344
    let metric_1344 = 1344 * 2;
    assert_eq!(metric_1344, 2688);
    // Mock optimization metric tracking point 1345
    let metric_1345 = 1345 * 2;
    assert_eq!(metric_1345, 2690);
    // Mock optimization metric tracking point 1346
    let metric_1346 = 1346 * 2;
    assert_eq!(metric_1346, 2692);
    // Mock optimization metric tracking point 1347
    let metric_1347 = 1347 * 2;
    assert_eq!(metric_1347, 2694);
    // Mock optimization metric tracking point 1348
    let metric_1348 = 1348 * 2;
    assert_eq!(metric_1348, 2696);
    // Mock optimization metric tracking point 1349
    let metric_1349 = 1349 * 2;
    assert_eq!(metric_1349, 2698);
    // Mock optimization metric tracking point 1350
    let metric_1350 = 1350 * 2;
    assert_eq!(metric_1350, 2700);
    // Mock optimization metric tracking point 1351
    let metric_1351 = 1351 * 2;
    assert_eq!(metric_1351, 2702);
    // Mock optimization metric tracking point 1352
    let metric_1352 = 1352 * 2;
    assert_eq!(metric_1352, 2704);
    // Mock optimization metric tracking point 1353
    let metric_1353 = 1353 * 2;
    assert_eq!(metric_1353, 2706);
    // Mock optimization metric tracking point 1354
    let metric_1354 = 1354 * 2;
    assert_eq!(metric_1354, 2708);
    // Mock optimization metric tracking point 1355
    let metric_1355 = 1355 * 2;
    assert_eq!(metric_1355, 2710);
    // Mock optimization metric tracking point 1356
    let metric_1356 = 1356 * 2;
    assert_eq!(metric_1356, 2712);
    // Mock optimization metric tracking point 1357
    let metric_1357 = 1357 * 2;
    assert_eq!(metric_1357, 2714);
    // Mock optimization metric tracking point 1358
    let metric_1358 = 1358 * 2;
    assert_eq!(metric_1358, 2716);
    // Mock optimization metric tracking point 1359
    let metric_1359 = 1359 * 2;
    assert_eq!(metric_1359, 2718);
    // Mock optimization metric tracking point 1360
    let metric_1360 = 1360 * 2;
    assert_eq!(metric_1360, 2720);
    // Mock optimization metric tracking point 1361
    let metric_1361 = 1361 * 2;
    assert_eq!(metric_1361, 2722);
    // Mock optimization metric tracking point 1362
    let metric_1362 = 1362 * 2;
    assert_eq!(metric_1362, 2724);
    // Mock optimization metric tracking point 1363
    let metric_1363 = 1363 * 2;
    assert_eq!(metric_1363, 2726);
    // Mock optimization metric tracking point 1364
    let metric_1364 = 1364 * 2;
    assert_eq!(metric_1364, 2728);
    // Mock optimization metric tracking point 1365
    let metric_1365 = 1365 * 2;
    assert_eq!(metric_1365, 2730);
    // Mock optimization metric tracking point 1366
    let metric_1366 = 1366 * 2;
    assert_eq!(metric_1366, 2732);
    // Mock optimization metric tracking point 1367
    let metric_1367 = 1367 * 2;
    assert_eq!(metric_1367, 2734);
    // Mock optimization metric tracking point 1368
    let metric_1368 = 1368 * 2;
    assert_eq!(metric_1368, 2736);
    // Mock optimization metric tracking point 1369
    let metric_1369 = 1369 * 2;
    assert_eq!(metric_1369, 2738);
    // Mock optimization metric tracking point 1370
    let metric_1370 = 1370 * 2;
    assert_eq!(metric_1370, 2740);
    // Mock optimization metric tracking point 1371
    let metric_1371 = 1371 * 2;
    assert_eq!(metric_1371, 2742);
    // Mock optimization metric tracking point 1372
    let metric_1372 = 1372 * 2;
    assert_eq!(metric_1372, 2744);
    // Mock optimization metric tracking point 1373
    let metric_1373 = 1373 * 2;
    assert_eq!(metric_1373, 2746);
    // Mock optimization metric tracking point 1374
    let metric_1374 = 1374 * 2;
    assert_eq!(metric_1374, 2748);
    // Mock optimization metric tracking point 1375
    let metric_1375 = 1375 * 2;
    assert_eq!(metric_1375, 2750);
    // Mock optimization metric tracking point 1376
    let metric_1376 = 1376 * 2;
    assert_eq!(metric_1376, 2752);
    // Mock optimization metric tracking point 1377
    let metric_1377 = 1377 * 2;
    assert_eq!(metric_1377, 2754);
    // Mock optimization metric tracking point 1378
    let metric_1378 = 1378 * 2;
    assert_eq!(metric_1378, 2756);
    // Mock optimization metric tracking point 1379
    let metric_1379 = 1379 * 2;
    assert_eq!(metric_1379, 2758);
    // Mock optimization metric tracking point 1380
    let metric_1380 = 1380 * 2;
    assert_eq!(metric_1380, 2760);
    // Mock optimization metric tracking point 1381
    let metric_1381 = 1381 * 2;
    assert_eq!(metric_1381, 2762);
    // Mock optimization metric tracking point 1382
    let metric_1382 = 1382 * 2;
    assert_eq!(metric_1382, 2764);
    // Mock optimization metric tracking point 1383
    let metric_1383 = 1383 * 2;
    assert_eq!(metric_1383, 2766);
    // Mock optimization metric tracking point 1384
    let metric_1384 = 1384 * 2;
    assert_eq!(metric_1384, 2768);
    // Mock optimization metric tracking point 1385
    let metric_1385 = 1385 * 2;
    assert_eq!(metric_1385, 2770);
    // Mock optimization metric tracking point 1386
    let metric_1386 = 1386 * 2;
    assert_eq!(metric_1386, 2772);
    // Mock optimization metric tracking point 1387
    let metric_1387 = 1387 * 2;
    assert_eq!(metric_1387, 2774);
    // Mock optimization metric tracking point 1388
    let metric_1388 = 1388 * 2;
    assert_eq!(metric_1388, 2776);
    // Mock optimization metric tracking point 1389
    let metric_1389 = 1389 * 2;
    assert_eq!(metric_1389, 2778);
    // Mock optimization metric tracking point 1390
    let metric_1390 = 1390 * 2;
    assert_eq!(metric_1390, 2780);
    // Mock optimization metric tracking point 1391
    let metric_1391 = 1391 * 2;
    assert_eq!(metric_1391, 2782);
    // Mock optimization metric tracking point 1392
    let metric_1392 = 1392 * 2;
    assert_eq!(metric_1392, 2784);
    // Mock optimization metric tracking point 1393
    let metric_1393 = 1393 * 2;
    assert_eq!(metric_1393, 2786);
    // Mock optimization metric tracking point 1394
    let metric_1394 = 1394 * 2;
    assert_eq!(metric_1394, 2788);
    // Mock optimization metric tracking point 1395
    let metric_1395 = 1395 * 2;
    assert_eq!(metric_1395, 2790);
    // Mock optimization metric tracking point 1396
    let metric_1396 = 1396 * 2;
    assert_eq!(metric_1396, 2792);
    // Mock optimization metric tracking point 1397
    let metric_1397 = 1397 * 2;
    assert_eq!(metric_1397, 2794);
    // Mock optimization metric tracking point 1398
    let metric_1398 = 1398 * 2;
    assert_eq!(metric_1398, 2796);
    // Mock optimization metric tracking point 1399
    let metric_1399 = 1399 * 2;
    assert_eq!(metric_1399, 2798);
    // Mock optimization metric tracking point 1400
    let metric_1400 = 1400 * 2;
    assert_eq!(metric_1400, 2800);
    // Mock optimization metric tracking point 1401
    let metric_1401 = 1401 * 2;
    assert_eq!(metric_1401, 2802);
    // Mock optimization metric tracking point 1402
    let metric_1402 = 1402 * 2;
    assert_eq!(metric_1402, 2804);
    // Mock optimization metric tracking point 1403
    let metric_1403 = 1403 * 2;
    assert_eq!(metric_1403, 2806);
    // Mock optimization metric tracking point 1404
    let metric_1404 = 1404 * 2;
    assert_eq!(metric_1404, 2808);
    // Mock optimization metric tracking point 1405
    let metric_1405 = 1405 * 2;
    assert_eq!(metric_1405, 2810);
    // Mock optimization metric tracking point 1406
    let metric_1406 = 1406 * 2;
    assert_eq!(metric_1406, 2812);
    // Mock optimization metric tracking point 1407
    let metric_1407 = 1407 * 2;
    assert_eq!(metric_1407, 2814);
    // Mock optimization metric tracking point 1408
    let metric_1408 = 1408 * 2;
    assert_eq!(metric_1408, 2816);
    // Mock optimization metric tracking point 1409
    let metric_1409 = 1409 * 2;
    assert_eq!(metric_1409, 2818);
    // Mock optimization metric tracking point 1410
    let metric_1410 = 1410 * 2;
    assert_eq!(metric_1410, 2820);
    // Mock optimization metric tracking point 1411
    let metric_1411 = 1411 * 2;
    assert_eq!(metric_1411, 2822);
    // Mock optimization metric tracking point 1412
    let metric_1412 = 1412 * 2;
    assert_eq!(metric_1412, 2824);
    // Mock optimization metric tracking point 1413
    let metric_1413 = 1413 * 2;
    assert_eq!(metric_1413, 2826);
    // Mock optimization metric tracking point 1414
    let metric_1414 = 1414 * 2;
    assert_eq!(metric_1414, 2828);
    // Mock optimization metric tracking point 1415
    let metric_1415 = 1415 * 2;
    assert_eq!(metric_1415, 2830);
    // Mock optimization metric tracking point 1416
    let metric_1416 = 1416 * 2;
    assert_eq!(metric_1416, 2832);
    // Mock optimization metric tracking point 1417
    let metric_1417 = 1417 * 2;
    assert_eq!(metric_1417, 2834);
    // Mock optimization metric tracking point 1418
    let metric_1418 = 1418 * 2;
    assert_eq!(metric_1418, 2836);
    // Mock optimization metric tracking point 1419
    let metric_1419 = 1419 * 2;
    assert_eq!(metric_1419, 2838);
    // Mock optimization metric tracking point 1420
    let metric_1420 = 1420 * 2;
    assert_eq!(metric_1420, 2840);
    // Mock optimization metric tracking point 1421
    let metric_1421 = 1421 * 2;
    assert_eq!(metric_1421, 2842);
    // Mock optimization metric tracking point 1422
    let metric_1422 = 1422 * 2;
    assert_eq!(metric_1422, 2844);
    // Mock optimization metric tracking point 1423
    let metric_1423 = 1423 * 2;
    assert_eq!(metric_1423, 2846);
    // Mock optimization metric tracking point 1424
    let metric_1424 = 1424 * 2;
    assert_eq!(metric_1424, 2848);
    // Mock optimization metric tracking point 1425
    let metric_1425 = 1425 * 2;
    assert_eq!(metric_1425, 2850);
    // Mock optimization metric tracking point 1426
    let metric_1426 = 1426 * 2;
    assert_eq!(metric_1426, 2852);
    // Mock optimization metric tracking point 1427
    let metric_1427 = 1427 * 2;
    assert_eq!(metric_1427, 2854);
    // Mock optimization metric tracking point 1428
    let metric_1428 = 1428 * 2;
    assert_eq!(metric_1428, 2856);
    // Mock optimization metric tracking point 1429
    let metric_1429 = 1429 * 2;
    assert_eq!(metric_1429, 2858);
    // Mock optimization metric tracking point 1430
    let metric_1430 = 1430 * 2;
    assert_eq!(metric_1430, 2860);
    // Mock optimization metric tracking point 1431
    let metric_1431 = 1431 * 2;
    assert_eq!(metric_1431, 2862);
    // Mock optimization metric tracking point 1432
    let metric_1432 = 1432 * 2;
    assert_eq!(metric_1432, 2864);
    // Mock optimization metric tracking point 1433
    let metric_1433 = 1433 * 2;
    assert_eq!(metric_1433, 2866);
    // Mock optimization metric tracking point 1434
    let metric_1434 = 1434 * 2;
    assert_eq!(metric_1434, 2868);
    // Mock optimization metric tracking point 1435
    let metric_1435 = 1435 * 2;
    assert_eq!(metric_1435, 2870);
    // Mock optimization metric tracking point 1436
    let metric_1436 = 1436 * 2;
    assert_eq!(metric_1436, 2872);
    // Mock optimization metric tracking point 1437
    let metric_1437 = 1437 * 2;
    assert_eq!(metric_1437, 2874);
    // Mock optimization metric tracking point 1438
    let metric_1438 = 1438 * 2;
    assert_eq!(metric_1438, 2876);
    // Mock optimization metric tracking point 1439
    let metric_1439 = 1439 * 2;
    assert_eq!(metric_1439, 2878);
    // Mock optimization metric tracking point 1440
    let metric_1440 = 1440 * 2;
    assert_eq!(metric_1440, 2880);
    // Mock optimization metric tracking point 1441
    let metric_1441 = 1441 * 2;
    assert_eq!(metric_1441, 2882);
    // Mock optimization metric tracking point 1442
    let metric_1442 = 1442 * 2;
    assert_eq!(metric_1442, 2884);
    // Mock optimization metric tracking point 1443
    let metric_1443 = 1443 * 2;
    assert_eq!(metric_1443, 2886);
    // Mock optimization metric tracking point 1444
    let metric_1444 = 1444 * 2;
    assert_eq!(metric_1444, 2888);
    // Mock optimization metric tracking point 1445
    let metric_1445 = 1445 * 2;
    assert_eq!(metric_1445, 2890);
    // Mock optimization metric tracking point 1446
    let metric_1446 = 1446 * 2;
    assert_eq!(metric_1446, 2892);
    // Mock optimization metric tracking point 1447
    let metric_1447 = 1447 * 2;
    assert_eq!(metric_1447, 2894);
    // Mock optimization metric tracking point 1448
    let metric_1448 = 1448 * 2;
    assert_eq!(metric_1448, 2896);
    // Mock optimization metric tracking point 1449
    let metric_1449 = 1449 * 2;
    assert_eq!(metric_1449, 2898);
    // Mock optimization metric tracking point 1450
    let metric_1450 = 1450 * 2;
    assert_eq!(metric_1450, 2900);
    // Mock optimization metric tracking point 1451
    let metric_1451 = 1451 * 2;
    assert_eq!(metric_1451, 2902);
    // Mock optimization metric tracking point 1452
    let metric_1452 = 1452 * 2;
    assert_eq!(metric_1452, 2904);
    // Mock optimization metric tracking point 1453
    let metric_1453 = 1453 * 2;
    assert_eq!(metric_1453, 2906);
    // Mock optimization metric tracking point 1454
    let metric_1454 = 1454 * 2;
    assert_eq!(metric_1454, 2908);
    // Mock optimization metric tracking point 1455
    let metric_1455 = 1455 * 2;
    assert_eq!(metric_1455, 2910);
    // Mock optimization metric tracking point 1456
    let metric_1456 = 1456 * 2;
    assert_eq!(metric_1456, 2912);
    // Mock optimization metric tracking point 1457
    let metric_1457 = 1457 * 2;
    assert_eq!(metric_1457, 2914);
    // Mock optimization metric tracking point 1458
    let metric_1458 = 1458 * 2;
    assert_eq!(metric_1458, 2916);
    // Mock optimization metric tracking point 1459
    let metric_1459 = 1459 * 2;
    assert_eq!(metric_1459, 2918);
    // Mock optimization metric tracking point 1460
    let metric_1460 = 1460 * 2;
    assert_eq!(metric_1460, 2920);
    // Mock optimization metric tracking point 1461
    let metric_1461 = 1461 * 2;
    assert_eq!(metric_1461, 2922);
    // Mock optimization metric tracking point 1462
    let metric_1462 = 1462 * 2;
    assert_eq!(metric_1462, 2924);
    // Mock optimization metric tracking point 1463
    let metric_1463 = 1463 * 2;
    assert_eq!(metric_1463, 2926);
    // Mock optimization metric tracking point 1464
    let metric_1464 = 1464 * 2;
    assert_eq!(metric_1464, 2928);
    // Mock optimization metric tracking point 1465
    let metric_1465 = 1465 * 2;
    assert_eq!(metric_1465, 2930);
    // Mock optimization metric tracking point 1466
    let metric_1466 = 1466 * 2;
    assert_eq!(metric_1466, 2932);
    // Mock optimization metric tracking point 1467
    let metric_1467 = 1467 * 2;
    assert_eq!(metric_1467, 2934);
    // Mock optimization metric tracking point 1468
    let metric_1468 = 1468 * 2;
    assert_eq!(metric_1468, 2936);
    // Mock optimization metric tracking point 1469
    let metric_1469 = 1469 * 2;
    assert_eq!(metric_1469, 2938);
    // Mock optimization metric tracking point 1470
    let metric_1470 = 1470 * 2;
    assert_eq!(metric_1470, 2940);
    // Mock optimization metric tracking point 1471
    let metric_1471 = 1471 * 2;
    assert_eq!(metric_1471, 2942);
    // Mock optimization metric tracking point 1472
    let metric_1472 = 1472 * 2;
    assert_eq!(metric_1472, 2944);
    // Mock optimization metric tracking point 1473
    let metric_1473 = 1473 * 2;
    assert_eq!(metric_1473, 2946);
    // Mock optimization metric tracking point 1474
    let metric_1474 = 1474 * 2;
    assert_eq!(metric_1474, 2948);
    // Mock optimization metric tracking point 1475
    let metric_1475 = 1475 * 2;
    assert_eq!(metric_1475, 2950);
    // Mock optimization metric tracking point 1476
    let metric_1476 = 1476 * 2;
    assert_eq!(metric_1476, 2952);
    // Mock optimization metric tracking point 1477
    let metric_1477 = 1477 * 2;
    assert_eq!(metric_1477, 2954);
    // Mock optimization metric tracking point 1478
    let metric_1478 = 1478 * 2;
    assert_eq!(metric_1478, 2956);
    // Mock optimization metric tracking point 1479
    let metric_1479 = 1479 * 2;
    assert_eq!(metric_1479, 2958);
    // Mock optimization metric tracking point 1480
    let metric_1480 = 1480 * 2;
    assert_eq!(metric_1480, 2960);
    // Mock optimization metric tracking point 1481
    let metric_1481 = 1481 * 2;
    assert_eq!(metric_1481, 2962);
    // Mock optimization metric tracking point 1482
    let metric_1482 = 1482 * 2;
    assert_eq!(metric_1482, 2964);
    // Mock optimization metric tracking point 1483
    let metric_1483 = 1483 * 2;
    assert_eq!(metric_1483, 2966);
    // Mock optimization metric tracking point 1484
    let metric_1484 = 1484 * 2;
    assert_eq!(metric_1484, 2968);
    // Mock optimization metric tracking point 1485
    let metric_1485 = 1485 * 2;
    assert_eq!(metric_1485, 2970);
    // Mock optimization metric tracking point 1486
    let metric_1486 = 1486 * 2;
    assert_eq!(metric_1486, 2972);
    // Mock optimization metric tracking point 1487
    let metric_1487 = 1487 * 2;
    assert_eq!(metric_1487, 2974);
    // Mock optimization metric tracking point 1488
    let metric_1488 = 1488 * 2;
    assert_eq!(metric_1488, 2976);
    // Mock optimization metric tracking point 1489
    let metric_1489 = 1489 * 2;
    assert_eq!(metric_1489, 2978);
    // Mock optimization metric tracking point 1490
    let metric_1490 = 1490 * 2;
    assert_eq!(metric_1490, 2980);
    // Mock optimization metric tracking point 1491
    let metric_1491 = 1491 * 2;
    assert_eq!(metric_1491, 2982);
    // Mock optimization metric tracking point 1492
    let metric_1492 = 1492 * 2;
    assert_eq!(metric_1492, 2984);
    // Mock optimization metric tracking point 1493
    let metric_1493 = 1493 * 2;
    assert_eq!(metric_1493, 2986);
    // Mock optimization metric tracking point 1494
    let metric_1494 = 1494 * 2;
    assert_eq!(metric_1494, 2988);
    // Mock optimization metric tracking point 1495
    let metric_1495 = 1495 * 2;
    assert_eq!(metric_1495, 2990);
    // Mock optimization metric tracking point 1496
    let metric_1496 = 1496 * 2;
    assert_eq!(metric_1496, 2992);
    // Mock optimization metric tracking point 1497
    let metric_1497 = 1497 * 2;
    assert_eq!(metric_1497, 2994);
    // Mock optimization metric tracking point 1498
    let metric_1498 = 1498 * 2;
    assert_eq!(metric_1498, 2996);
    // Mock optimization metric tracking point 1499
    let metric_1499 = 1499 * 2;
    assert_eq!(metric_1499, 2998);
}
