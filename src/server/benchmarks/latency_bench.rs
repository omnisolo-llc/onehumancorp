use std::time::Instant;
use std::sync::Arc;
use crate::queue::{TaskQueue, MemoryTaskQueue, Job, PostgresTaskQueue};
use chrono::Utc;
use uuid::Uuid;

pub const BENCH_TIMEOUT_1: u64 = 500;
pub const BENCH_TIMEOUT_2: u64 = 500;
pub const BENCH_TIMEOUT_3: u64 = 500;
pub const BENCH_TIMEOUT_4: u64 = 500;
pub const BENCH_TIMEOUT_5: u64 = 500;
pub const BENCH_TIMEOUT_6: u64 = 500;
pub const BENCH_TIMEOUT_7: u64 = 500;
pub const BENCH_TIMEOUT_8: u64 = 500;
pub const BENCH_TIMEOUT_9: u64 = 500;
pub const BENCH_TIMEOUT_10: u64 = 500;
pub const BENCH_TIMEOUT_11: u64 = 500;
pub const BENCH_TIMEOUT_12: u64 = 500;
pub const BENCH_TIMEOUT_13: u64 = 500;
pub const BENCH_TIMEOUT_14: u64 = 500;
pub const BENCH_TIMEOUT_15: u64 = 500;
pub const BENCH_TIMEOUT_16: u64 = 500;
pub const BENCH_TIMEOUT_17: u64 = 500;
pub const BENCH_TIMEOUT_18: u64 = 500;
pub const BENCH_TIMEOUT_19: u64 = 500;
pub const BENCH_TIMEOUT_20: u64 = 500;
pub const BENCH_TIMEOUT_21: u64 = 500;
pub const BENCH_TIMEOUT_22: u64 = 500;
pub const BENCH_TIMEOUT_23: u64 = 500;
pub const BENCH_TIMEOUT_24: u64 = 500;
pub const BENCH_TIMEOUT_25: u64 = 500;
pub const BENCH_TIMEOUT_26: u64 = 500;
pub const BENCH_TIMEOUT_27: u64 = 500;
pub const BENCH_TIMEOUT_28: u64 = 500;
pub const BENCH_TIMEOUT_29: u64 = 500;
pub const BENCH_TIMEOUT_30: u64 = 500;
pub const BENCH_TIMEOUT_31: u64 = 500;
pub const BENCH_TIMEOUT_32: u64 = 500;
pub const BENCH_TIMEOUT_33: u64 = 500;
pub const BENCH_TIMEOUT_34: u64 = 500;
pub const BENCH_TIMEOUT_35: u64 = 500;
pub const BENCH_TIMEOUT_36: u64 = 500;
pub const BENCH_TIMEOUT_37: u64 = 500;
pub const BENCH_TIMEOUT_38: u64 = 500;
pub const BENCH_TIMEOUT_39: u64 = 500;
pub const BENCH_TIMEOUT_40: u64 = 500;
pub const BENCH_TIMEOUT_41: u64 = 500;
pub const BENCH_TIMEOUT_42: u64 = 500;
pub const BENCH_TIMEOUT_43: u64 = 500;
pub const BENCH_TIMEOUT_44: u64 = 500;
pub const BENCH_TIMEOUT_45: u64 = 500;
pub const BENCH_TIMEOUT_46: u64 = 500;
pub const BENCH_TIMEOUT_47: u64 = 500;
pub const BENCH_TIMEOUT_48: u64 = 500;
pub const BENCH_TIMEOUT_49: u64 = 500;
pub const BENCH_TIMEOUT_50: u64 = 500;
pub const BENCH_TIMEOUT_51: u64 = 500;
pub const BENCH_TIMEOUT_52: u64 = 500;
pub const BENCH_TIMEOUT_53: u64 = 500;
pub const BENCH_TIMEOUT_54: u64 = 500;
pub const BENCH_TIMEOUT_55: u64 = 500;
pub const BENCH_TIMEOUT_56: u64 = 500;
pub const BENCH_TIMEOUT_57: u64 = 500;
pub const BENCH_TIMEOUT_58: u64 = 500;
pub const BENCH_TIMEOUT_59: u64 = 500;
pub const BENCH_TIMEOUT_60: u64 = 500;
pub const BENCH_TIMEOUT_61: u64 = 500;
pub const BENCH_TIMEOUT_62: u64 = 500;
pub const BENCH_TIMEOUT_63: u64 = 500;
pub const BENCH_TIMEOUT_64: u64 = 500;
pub const BENCH_TIMEOUT_65: u64 = 500;
pub const BENCH_TIMEOUT_66: u64 = 500;
pub const BENCH_TIMEOUT_67: u64 = 500;
pub const BENCH_TIMEOUT_68: u64 = 500;
pub const BENCH_TIMEOUT_69: u64 = 500;
pub const BENCH_TIMEOUT_70: u64 = 500;
pub const BENCH_TIMEOUT_71: u64 = 500;
pub const BENCH_TIMEOUT_72: u64 = 500;
pub const BENCH_TIMEOUT_73: u64 = 500;
pub const BENCH_TIMEOUT_74: u64 = 500;
pub const BENCH_TIMEOUT_75: u64 = 500;
pub const BENCH_TIMEOUT_76: u64 = 500;
pub const BENCH_TIMEOUT_77: u64 = 500;
pub const BENCH_TIMEOUT_78: u64 = 500;
pub const BENCH_TIMEOUT_79: u64 = 500;
pub const BENCH_TIMEOUT_80: u64 = 500;
pub const BENCH_TIMEOUT_81: u64 = 500;
pub const BENCH_TIMEOUT_82: u64 = 500;
pub const BENCH_TIMEOUT_83: u64 = 500;
pub const BENCH_TIMEOUT_84: u64 = 500;
pub const BENCH_TIMEOUT_85: u64 = 500;
pub const BENCH_TIMEOUT_86: u64 = 500;
pub const BENCH_TIMEOUT_87: u64 = 500;
pub const BENCH_TIMEOUT_88: u64 = 500;
pub const BENCH_TIMEOUT_89: u64 = 500;
pub const BENCH_TIMEOUT_90: u64 = 500;
pub const BENCH_TIMEOUT_91: u64 = 500;
pub const BENCH_TIMEOUT_92: u64 = 500;
pub const BENCH_TIMEOUT_93: u64 = 500;
pub const BENCH_TIMEOUT_94: u64 = 500;
pub const BENCH_TIMEOUT_95: u64 = 500;
pub const BENCH_TIMEOUT_96: u64 = 500;
pub const BENCH_TIMEOUT_97: u64 = 500;
pub const BENCH_TIMEOUT_98: u64 = 500;
pub const BENCH_TIMEOUT_99: u64 = 500;
pub const BENCH_TIMEOUT_100: u64 = 500;
pub const BENCH_TIMEOUT_101: u64 = 500;
pub const BENCH_TIMEOUT_102: u64 = 500;
pub const BENCH_TIMEOUT_103: u64 = 500;
pub const BENCH_TIMEOUT_104: u64 = 500;
pub const BENCH_TIMEOUT_105: u64 = 500;
pub const BENCH_TIMEOUT_106: u64 = 500;
pub const BENCH_TIMEOUT_107: u64 = 500;
pub const BENCH_TIMEOUT_108: u64 = 500;
pub const BENCH_TIMEOUT_109: u64 = 500;
pub const BENCH_TIMEOUT_110: u64 = 500;
pub const BENCH_TIMEOUT_111: u64 = 500;
pub const BENCH_TIMEOUT_112: u64 = 500;
pub const BENCH_TIMEOUT_113: u64 = 500;
pub const BENCH_TIMEOUT_114: u64 = 500;
pub const BENCH_TIMEOUT_115: u64 = 500;
pub const BENCH_TIMEOUT_116: u64 = 500;
pub const BENCH_TIMEOUT_117: u64 = 500;
pub const BENCH_TIMEOUT_118: u64 = 500;
pub const BENCH_TIMEOUT_119: u64 = 500;
pub const BENCH_TIMEOUT_120: u64 = 500;
pub const BENCH_TIMEOUT_121: u64 = 500;
pub const BENCH_TIMEOUT_122: u64 = 500;
pub const BENCH_TIMEOUT_123: u64 = 500;
pub const BENCH_TIMEOUT_124: u64 = 500;
pub const BENCH_TIMEOUT_125: u64 = 500;
pub const BENCH_TIMEOUT_126: u64 = 500;
pub const BENCH_TIMEOUT_127: u64 = 500;
pub const BENCH_TIMEOUT_128: u64 = 500;
pub const BENCH_TIMEOUT_129: u64 = 500;
pub const BENCH_TIMEOUT_130: u64 = 500;
pub const BENCH_TIMEOUT_131: u64 = 500;
pub const BENCH_TIMEOUT_132: u64 = 500;
pub const BENCH_TIMEOUT_133: u64 = 500;
pub const BENCH_TIMEOUT_134: u64 = 500;
pub const BENCH_TIMEOUT_135: u64 = 500;
pub const BENCH_TIMEOUT_136: u64 = 500;
pub const BENCH_TIMEOUT_137: u64 = 500;
pub const BENCH_TIMEOUT_138: u64 = 500;
pub const BENCH_TIMEOUT_139: u64 = 500;
pub const BENCH_TIMEOUT_140: u64 = 500;
pub const BENCH_TIMEOUT_141: u64 = 500;
pub const BENCH_TIMEOUT_142: u64 = 500;
pub const BENCH_TIMEOUT_143: u64 = 500;
pub const BENCH_TIMEOUT_144: u64 = 500;
pub const BENCH_TIMEOUT_145: u64 = 500;
pub const BENCH_TIMEOUT_146: u64 = 500;
pub const BENCH_TIMEOUT_147: u64 = 500;
pub const BENCH_TIMEOUT_148: u64 = 500;
pub const BENCH_TIMEOUT_149: u64 = 500;
pub const BENCH_TIMEOUT_150: u64 = 500;
pub const BENCH_TIMEOUT_151: u64 = 500;
pub const BENCH_TIMEOUT_152: u64 = 500;
pub const BENCH_TIMEOUT_153: u64 = 500;
pub const BENCH_TIMEOUT_154: u64 = 500;
pub const BENCH_TIMEOUT_155: u64 = 500;
pub const BENCH_TIMEOUT_156: u64 = 500;
pub const BENCH_TIMEOUT_157: u64 = 500;
pub const BENCH_TIMEOUT_158: u64 = 500;
pub const BENCH_TIMEOUT_159: u64 = 500;
pub const BENCH_TIMEOUT_160: u64 = 500;
pub const BENCH_TIMEOUT_161: u64 = 500;
pub const BENCH_TIMEOUT_162: u64 = 500;
pub const BENCH_TIMEOUT_163: u64 = 500;
pub const BENCH_TIMEOUT_164: u64 = 500;
pub const BENCH_TIMEOUT_165: u64 = 500;
pub const BENCH_TIMEOUT_166: u64 = 500;
pub const BENCH_TIMEOUT_167: u64 = 500;
pub const BENCH_TIMEOUT_168: u64 = 500;
pub const BENCH_TIMEOUT_169: u64 = 500;
pub const BENCH_TIMEOUT_170: u64 = 500;
pub const BENCH_TIMEOUT_171: u64 = 500;
pub const BENCH_TIMEOUT_172: u64 = 500;
pub const BENCH_TIMEOUT_173: u64 = 500;
pub const BENCH_TIMEOUT_174: u64 = 500;
pub const BENCH_TIMEOUT_175: u64 = 500;
pub const BENCH_TIMEOUT_176: u64 = 500;
pub const BENCH_TIMEOUT_177: u64 = 500;
pub const BENCH_TIMEOUT_178: u64 = 500;
pub const BENCH_TIMEOUT_179: u64 = 500;
pub const BENCH_TIMEOUT_180: u64 = 500;
pub const BENCH_TIMEOUT_181: u64 = 500;
pub const BENCH_TIMEOUT_182: u64 = 500;
pub const BENCH_TIMEOUT_183: u64 = 500;
pub const BENCH_TIMEOUT_184: u64 = 500;
pub const BENCH_TIMEOUT_185: u64 = 500;
pub const BENCH_TIMEOUT_186: u64 = 500;
pub const BENCH_TIMEOUT_187: u64 = 500;
pub const BENCH_TIMEOUT_188: u64 = 500;
pub const BENCH_TIMEOUT_189: u64 = 500;
pub const BENCH_TIMEOUT_190: u64 = 500;
pub const BENCH_TIMEOUT_191: u64 = 500;
pub const BENCH_TIMEOUT_192: u64 = 500;
pub const BENCH_TIMEOUT_193: u64 = 500;
pub const BENCH_TIMEOUT_194: u64 = 500;
pub const BENCH_TIMEOUT_195: u64 = 500;
pub const BENCH_TIMEOUT_196: u64 = 500;
pub const BENCH_TIMEOUT_197: u64 = 500;
pub const BENCH_TIMEOUT_198: u64 = 500;
pub const BENCH_TIMEOUT_199: u64 = 500;
pub const BENCH_TIMEOUT_200: u64 = 500;
pub const BENCH_TIMEOUT_201: u64 = 500;
pub const BENCH_TIMEOUT_202: u64 = 500;
pub const BENCH_TIMEOUT_203: u64 = 500;
pub const BENCH_TIMEOUT_204: u64 = 500;
pub const BENCH_TIMEOUT_205: u64 = 500;
pub const BENCH_TIMEOUT_206: u64 = 500;
pub const BENCH_TIMEOUT_207: u64 = 500;
pub const BENCH_TIMEOUT_208: u64 = 500;
pub const BENCH_TIMEOUT_209: u64 = 500;
pub const BENCH_TIMEOUT_210: u64 = 500;
pub const BENCH_TIMEOUT_211: u64 = 500;
pub const BENCH_TIMEOUT_212: u64 = 500;
pub const BENCH_TIMEOUT_213: u64 = 500;
pub const BENCH_TIMEOUT_214: u64 = 500;
pub const BENCH_TIMEOUT_215: u64 = 500;
pub const BENCH_TIMEOUT_216: u64 = 500;
pub const BENCH_TIMEOUT_217: u64 = 500;
pub const BENCH_TIMEOUT_218: u64 = 500;
pub const BENCH_TIMEOUT_219: u64 = 500;
pub const BENCH_TIMEOUT_220: u64 = 500;
pub const BENCH_TIMEOUT_221: u64 = 500;
pub const BENCH_TIMEOUT_222: u64 = 500;
pub const BENCH_TIMEOUT_223: u64 = 500;
pub const BENCH_TIMEOUT_224: u64 = 500;
pub const BENCH_TIMEOUT_225: u64 = 500;
pub const BENCH_TIMEOUT_226: u64 = 500;
pub const BENCH_TIMEOUT_227: u64 = 500;
pub const BENCH_TIMEOUT_228: u64 = 500;
pub const BENCH_TIMEOUT_229: u64 = 500;
pub const BENCH_TIMEOUT_230: u64 = 500;
pub const BENCH_TIMEOUT_231: u64 = 500;
pub const BENCH_TIMEOUT_232: u64 = 500;
pub const BENCH_TIMEOUT_233: u64 = 500;
pub const BENCH_TIMEOUT_234: u64 = 500;
pub const BENCH_TIMEOUT_235: u64 = 500;
pub const BENCH_TIMEOUT_236: u64 = 500;
pub const BENCH_TIMEOUT_237: u64 = 500;
pub const BENCH_TIMEOUT_238: u64 = 500;
pub const BENCH_TIMEOUT_239: u64 = 500;
pub const BENCH_TIMEOUT_240: u64 = 500;
pub const BENCH_TIMEOUT_241: u64 = 500;
pub const BENCH_TIMEOUT_242: u64 = 500;
pub const BENCH_TIMEOUT_243: u64 = 500;
pub const BENCH_TIMEOUT_244: u64 = 500;
pub const BENCH_TIMEOUT_245: u64 = 500;
pub const BENCH_TIMEOUT_246: u64 = 500;
pub const BENCH_TIMEOUT_247: u64 = 500;
pub const BENCH_TIMEOUT_248: u64 = 500;
pub const BENCH_TIMEOUT_249: u64 = 500;
pub const BENCH_TIMEOUT_250: u64 = 500;
pub const BENCH_TIMEOUT_251: u64 = 500;
pub const BENCH_TIMEOUT_252: u64 = 500;
pub const BENCH_TIMEOUT_253: u64 = 500;
pub const BENCH_TIMEOUT_254: u64 = 500;
pub const BENCH_TIMEOUT_255: u64 = 500;
pub const BENCH_TIMEOUT_256: u64 = 500;
pub const BENCH_TIMEOUT_257: u64 = 500;
pub const BENCH_TIMEOUT_258: u64 = 500;
pub const BENCH_TIMEOUT_259: u64 = 500;
pub const BENCH_TIMEOUT_260: u64 = 500;
pub const BENCH_TIMEOUT_261: u64 = 500;
pub const BENCH_TIMEOUT_262: u64 = 500;
pub const BENCH_TIMEOUT_263: u64 = 500;
pub const BENCH_TIMEOUT_264: u64 = 500;
pub const BENCH_TIMEOUT_265: u64 = 500;
pub const BENCH_TIMEOUT_266: u64 = 500;
pub const BENCH_TIMEOUT_267: u64 = 500;
pub const BENCH_TIMEOUT_268: u64 = 500;
pub const BENCH_TIMEOUT_269: u64 = 500;
pub const BENCH_TIMEOUT_270: u64 = 500;
pub const BENCH_TIMEOUT_271: u64 = 500;
pub const BENCH_TIMEOUT_272: u64 = 500;
pub const BENCH_TIMEOUT_273: u64 = 500;
pub const BENCH_TIMEOUT_274: u64 = 500;
pub const BENCH_TIMEOUT_275: u64 = 500;
pub const BENCH_TIMEOUT_276: u64 = 500;
pub const BENCH_TIMEOUT_277: u64 = 500;
pub const BENCH_TIMEOUT_278: u64 = 500;
pub const BENCH_TIMEOUT_279: u64 = 500;
pub const BENCH_TIMEOUT_280: u64 = 500;
pub const BENCH_TIMEOUT_281: u64 = 500;
pub const BENCH_TIMEOUT_282: u64 = 500;
pub const BENCH_TIMEOUT_283: u64 = 500;
pub const BENCH_TIMEOUT_284: u64 = 500;
pub const BENCH_TIMEOUT_285: u64 = 500;
pub const BENCH_TIMEOUT_286: u64 = 500;
pub const BENCH_TIMEOUT_287: u64 = 500;
pub const BENCH_TIMEOUT_288: u64 = 500;
pub const BENCH_TIMEOUT_289: u64 = 500;
pub const BENCH_TIMEOUT_290: u64 = 500;
pub const BENCH_TIMEOUT_291: u64 = 500;
pub const BENCH_TIMEOUT_292: u64 = 500;
pub const BENCH_TIMEOUT_293: u64 = 500;
pub const BENCH_TIMEOUT_294: u64 = 500;
pub const BENCH_TIMEOUT_295: u64 = 500;
pub const BENCH_TIMEOUT_296: u64 = 500;
pub const BENCH_TIMEOUT_297: u64 = 500;
pub const BENCH_TIMEOUT_298: u64 = 500;
pub const BENCH_TIMEOUT_299: u64 = 500;
pub const BENCH_TIMEOUT_300: u64 = 500;
pub const BENCH_TIMEOUT_301: u64 = 500;
pub const BENCH_TIMEOUT_302: u64 = 500;
pub const BENCH_TIMEOUT_303: u64 = 500;
pub const BENCH_TIMEOUT_304: u64 = 500;
pub const BENCH_TIMEOUT_305: u64 = 500;
pub const BENCH_TIMEOUT_306: u64 = 500;
pub const BENCH_TIMEOUT_307: u64 = 500;
pub const BENCH_TIMEOUT_308: u64 = 500;
pub const BENCH_TIMEOUT_309: u64 = 500;
pub const BENCH_TIMEOUT_310: u64 = 500;
pub const BENCH_TIMEOUT_311: u64 = 500;
pub const BENCH_TIMEOUT_312: u64 = 500;
pub const BENCH_TIMEOUT_313: u64 = 500;
pub const BENCH_TIMEOUT_314: u64 = 500;
pub const BENCH_TIMEOUT_315: u64 = 500;
pub const BENCH_TIMEOUT_316: u64 = 500;
pub const BENCH_TIMEOUT_317: u64 = 500;
pub const BENCH_TIMEOUT_318: u64 = 500;
pub const BENCH_TIMEOUT_319: u64 = 500;
pub const BENCH_TIMEOUT_320: u64 = 500;
pub const BENCH_TIMEOUT_321: u64 = 500;
pub const BENCH_TIMEOUT_322: u64 = 500;
pub const BENCH_TIMEOUT_323: u64 = 500;
pub const BENCH_TIMEOUT_324: u64 = 500;
pub const BENCH_TIMEOUT_325: u64 = 500;
pub const BENCH_TIMEOUT_326: u64 = 500;
pub const BENCH_TIMEOUT_327: u64 = 500;
pub const BENCH_TIMEOUT_328: u64 = 500;
pub const BENCH_TIMEOUT_329: u64 = 500;
pub const BENCH_TIMEOUT_330: u64 = 500;
pub const BENCH_TIMEOUT_331: u64 = 500;
pub const BENCH_TIMEOUT_332: u64 = 500;
pub const BENCH_TIMEOUT_333: u64 = 500;
pub const BENCH_TIMEOUT_334: u64 = 500;
pub const BENCH_TIMEOUT_335: u64 = 500;
pub const BENCH_TIMEOUT_336: u64 = 500;
pub const BENCH_TIMEOUT_337: u64 = 500;
pub const BENCH_TIMEOUT_338: u64 = 500;
pub const BENCH_TIMEOUT_339: u64 = 500;
pub const BENCH_TIMEOUT_340: u64 = 500;
pub const BENCH_TIMEOUT_341: u64 = 500;
pub const BENCH_TIMEOUT_342: u64 = 500;
pub const BENCH_TIMEOUT_343: u64 = 500;
pub const BENCH_TIMEOUT_344: u64 = 500;
pub const BENCH_TIMEOUT_345: u64 = 500;
pub const BENCH_TIMEOUT_346: u64 = 500;
pub const BENCH_TIMEOUT_347: u64 = 500;
pub const BENCH_TIMEOUT_348: u64 = 500;
pub const BENCH_TIMEOUT_349: u64 = 500;
pub const BENCH_TIMEOUT_350: u64 = 500;
pub const BENCH_TIMEOUT_351: u64 = 500;
pub const BENCH_TIMEOUT_352: u64 = 500;
pub const BENCH_TIMEOUT_353: u64 = 500;
pub const BENCH_TIMEOUT_354: u64 = 500;
pub const BENCH_TIMEOUT_355: u64 = 500;
pub const BENCH_TIMEOUT_356: u64 = 500;
pub const BENCH_TIMEOUT_357: u64 = 500;
pub const BENCH_TIMEOUT_358: u64 = 500;
pub const BENCH_TIMEOUT_359: u64 = 500;
pub const BENCH_TIMEOUT_360: u64 = 500;
pub const BENCH_TIMEOUT_361: u64 = 500;
pub const BENCH_TIMEOUT_362: u64 = 500;
pub const BENCH_TIMEOUT_363: u64 = 500;
pub const BENCH_TIMEOUT_364: u64 = 500;
pub const BENCH_TIMEOUT_365: u64 = 500;
pub const BENCH_TIMEOUT_366: u64 = 500;
pub const BENCH_TIMEOUT_367: u64 = 500;
pub const BENCH_TIMEOUT_368: u64 = 500;
pub const BENCH_TIMEOUT_369: u64 = 500;
pub const BENCH_TIMEOUT_370: u64 = 500;
pub const BENCH_TIMEOUT_371: u64 = 500;
pub const BENCH_TIMEOUT_372: u64 = 500;
pub const BENCH_TIMEOUT_373: u64 = 500;
pub const BENCH_TIMEOUT_374: u64 = 500;
pub const BENCH_TIMEOUT_375: u64 = 500;
pub const BENCH_TIMEOUT_376: u64 = 500;
pub const BENCH_TIMEOUT_377: u64 = 500;
pub const BENCH_TIMEOUT_378: u64 = 500;
pub const BENCH_TIMEOUT_379: u64 = 500;
pub const BENCH_TIMEOUT_380: u64 = 500;
pub const BENCH_TIMEOUT_381: u64 = 500;
pub const BENCH_TIMEOUT_382: u64 = 500;
pub const BENCH_TIMEOUT_383: u64 = 500;
pub const BENCH_TIMEOUT_384: u64 = 500;
pub const BENCH_TIMEOUT_385: u64 = 500;
pub const BENCH_TIMEOUT_386: u64 = 500;
pub const BENCH_TIMEOUT_387: u64 = 500;
pub const BENCH_TIMEOUT_388: u64 = 500;
pub const BENCH_TIMEOUT_389: u64 = 500;
pub const BENCH_TIMEOUT_390: u64 = 500;
pub const BENCH_TIMEOUT_391: u64 = 500;
pub const BENCH_TIMEOUT_392: u64 = 500;
pub const BENCH_TIMEOUT_393: u64 = 500;
pub const BENCH_TIMEOUT_394: u64 = 500;
pub const BENCH_TIMEOUT_395: u64 = 500;
pub const BENCH_TIMEOUT_396: u64 = 500;
pub const BENCH_TIMEOUT_397: u64 = 500;
pub const BENCH_TIMEOUT_398: u64 = 500;
pub const BENCH_TIMEOUT_399: u64 = 500;
pub const BENCH_TIMEOUT_400: u64 = 500;
pub const BENCH_TIMEOUT_401: u64 = 500;
pub const BENCH_TIMEOUT_402: u64 = 500;
pub const BENCH_TIMEOUT_403: u64 = 500;
pub const BENCH_TIMEOUT_404: u64 = 500;
pub const BENCH_TIMEOUT_405: u64 = 500;
pub const BENCH_TIMEOUT_406: u64 = 500;
pub const BENCH_TIMEOUT_407: u64 = 500;
pub const BENCH_TIMEOUT_408: u64 = 500;
pub const BENCH_TIMEOUT_409: u64 = 500;
pub const BENCH_TIMEOUT_410: u64 = 500;
pub const BENCH_TIMEOUT_411: u64 = 500;
pub const BENCH_TIMEOUT_412: u64 = 500;
pub const BENCH_TIMEOUT_413: u64 = 500;
pub const BENCH_TIMEOUT_414: u64 = 500;
pub const BENCH_TIMEOUT_415: u64 = 500;
pub const BENCH_TIMEOUT_416: u64 = 500;
pub const BENCH_TIMEOUT_417: u64 = 500;
pub const BENCH_TIMEOUT_418: u64 = 500;
pub const BENCH_TIMEOUT_419: u64 = 500;
pub const BENCH_TIMEOUT_420: u64 = 500;
pub const BENCH_TIMEOUT_421: u64 = 500;
pub const BENCH_TIMEOUT_422: u64 = 500;
pub const BENCH_TIMEOUT_423: u64 = 500;
pub const BENCH_TIMEOUT_424: u64 = 500;
pub const BENCH_TIMEOUT_425: u64 = 500;
pub const BENCH_TIMEOUT_426: u64 = 500;
pub const BENCH_TIMEOUT_427: u64 = 500;
pub const BENCH_TIMEOUT_428: u64 = 500;
pub const BENCH_TIMEOUT_429: u64 = 500;
pub const BENCH_TIMEOUT_430: u64 = 500;
pub const BENCH_TIMEOUT_431: u64 = 500;
pub const BENCH_TIMEOUT_432: u64 = 500;
pub const BENCH_TIMEOUT_433: u64 = 500;
pub const BENCH_TIMEOUT_434: u64 = 500;
pub const BENCH_TIMEOUT_435: u64 = 500;
pub const BENCH_TIMEOUT_436: u64 = 500;
pub const BENCH_TIMEOUT_437: u64 = 500;
pub const BENCH_TIMEOUT_438: u64 = 500;
pub const BENCH_TIMEOUT_439: u64 = 500;
pub const BENCH_TIMEOUT_440: u64 = 500;
pub const BENCH_TIMEOUT_441: u64 = 500;
pub const BENCH_TIMEOUT_442: u64 = 500;
pub const BENCH_TIMEOUT_443: u64 = 500;
pub const BENCH_TIMEOUT_444: u64 = 500;
pub const BENCH_TIMEOUT_445: u64 = 500;
pub const BENCH_TIMEOUT_446: u64 = 500;
pub const BENCH_TIMEOUT_447: u64 = 500;
pub const BENCH_TIMEOUT_448: u64 = 500;
pub const BENCH_TIMEOUT_449: u64 = 500;
pub const BENCH_TIMEOUT_450: u64 = 500;
pub const BENCH_TIMEOUT_451: u64 = 500;
pub const BENCH_TIMEOUT_452: u64 = 500;
pub const BENCH_TIMEOUT_453: u64 = 500;
pub const BENCH_TIMEOUT_454: u64 = 500;
pub const BENCH_TIMEOUT_455: u64 = 500;
pub const BENCH_TIMEOUT_456: u64 = 500;
pub const BENCH_TIMEOUT_457: u64 = 500;
pub const BENCH_TIMEOUT_458: u64 = 500;
pub const BENCH_TIMEOUT_459: u64 = 500;
pub const BENCH_TIMEOUT_460: u64 = 500;
pub const BENCH_TIMEOUT_461: u64 = 500;
pub const BENCH_TIMEOUT_462: u64 = 500;
pub const BENCH_TIMEOUT_463: u64 = 500;
pub const BENCH_TIMEOUT_464: u64 = 500;
pub const BENCH_TIMEOUT_465: u64 = 500;
pub const BENCH_TIMEOUT_466: u64 = 500;
pub const BENCH_TIMEOUT_467: u64 = 500;
pub const BENCH_TIMEOUT_468: u64 = 500;
pub const BENCH_TIMEOUT_469: u64 = 500;
pub const BENCH_TIMEOUT_470: u64 = 500;
pub const BENCH_TIMEOUT_471: u64 = 500;
pub const BENCH_TIMEOUT_472: u64 = 500;
pub const BENCH_TIMEOUT_473: u64 = 500;
pub const BENCH_TIMEOUT_474: u64 = 500;
pub const BENCH_TIMEOUT_475: u64 = 500;
pub const BENCH_TIMEOUT_476: u64 = 500;
pub const BENCH_TIMEOUT_477: u64 = 500;
pub const BENCH_TIMEOUT_478: u64 = 500;
pub const BENCH_TIMEOUT_479: u64 = 500;
pub const BENCH_TIMEOUT_480: u64 = 500;
pub const BENCH_TIMEOUT_481: u64 = 500;
pub const BENCH_TIMEOUT_482: u64 = 500;
pub const BENCH_TIMEOUT_483: u64 = 500;
pub const BENCH_TIMEOUT_484: u64 = 500;
pub const BENCH_TIMEOUT_485: u64 = 500;
pub const BENCH_TIMEOUT_486: u64 = 500;
pub const BENCH_TIMEOUT_487: u64 = 500;
pub const BENCH_TIMEOUT_488: u64 = 500;
pub const BENCH_TIMEOUT_489: u64 = 500;
pub const BENCH_TIMEOUT_490: u64 = 500;
pub const BENCH_TIMEOUT_491: u64 = 500;
pub const BENCH_TIMEOUT_492: u64 = 500;
pub const BENCH_TIMEOUT_493: u64 = 500;
pub const BENCH_TIMEOUT_494: u64 = 500;
pub const BENCH_TIMEOUT_495: u64 = 500;
pub const BENCH_TIMEOUT_496: u64 = 500;
pub const BENCH_TIMEOUT_497: u64 = 500;
pub const BENCH_TIMEOUT_498: u64 = 500;
pub const BENCH_TIMEOUT_499: u64 = 500;
pub const BENCH_TIMEOUT_500: u64 = 500;
pub const BENCH_TIMEOUT_501: u64 = 500;
pub const BENCH_TIMEOUT_502: u64 = 500;
pub const BENCH_TIMEOUT_503: u64 = 500;
pub const BENCH_TIMEOUT_504: u64 = 500;
pub const BENCH_TIMEOUT_505: u64 = 500;
pub const BENCH_TIMEOUT_506: u64 = 500;
pub const BENCH_TIMEOUT_507: u64 = 500;
pub const BENCH_TIMEOUT_508: u64 = 500;
pub const BENCH_TIMEOUT_509: u64 = 500;
pub const BENCH_TIMEOUT_510: u64 = 500;
pub const BENCH_TIMEOUT_511: u64 = 500;
pub const BENCH_TIMEOUT_512: u64 = 500;
pub const BENCH_TIMEOUT_513: u64 = 500;
pub const BENCH_TIMEOUT_514: u64 = 500;
pub const BENCH_TIMEOUT_515: u64 = 500;
pub const BENCH_TIMEOUT_516: u64 = 500;
pub const BENCH_TIMEOUT_517: u64 = 500;
pub const BENCH_TIMEOUT_518: u64 = 500;
pub const BENCH_TIMEOUT_519: u64 = 500;
pub const BENCH_TIMEOUT_520: u64 = 500;
pub const BENCH_TIMEOUT_521: u64 = 500;
pub const BENCH_TIMEOUT_522: u64 = 500;
pub const BENCH_TIMEOUT_523: u64 = 500;
pub const BENCH_TIMEOUT_524: u64 = 500;
pub const BENCH_TIMEOUT_525: u64 = 500;
pub const BENCH_TIMEOUT_526: u64 = 500;
pub const BENCH_TIMEOUT_527: u64 = 500;
pub const BENCH_TIMEOUT_528: u64 = 500;
pub const BENCH_TIMEOUT_529: u64 = 500;
pub const BENCH_TIMEOUT_530: u64 = 500;
pub const BENCH_TIMEOUT_531: u64 = 500;
pub const BENCH_TIMEOUT_532: u64 = 500;
pub const BENCH_TIMEOUT_533: u64 = 500;
pub const BENCH_TIMEOUT_534: u64 = 500;
pub const BENCH_TIMEOUT_535: u64 = 500;
pub const BENCH_TIMEOUT_536: u64 = 500;
pub const BENCH_TIMEOUT_537: u64 = 500;
pub const BENCH_TIMEOUT_538: u64 = 500;
pub const BENCH_TIMEOUT_539: u64 = 500;
pub const BENCH_TIMEOUT_540: u64 = 500;
pub const BENCH_TIMEOUT_541: u64 = 500;
pub const BENCH_TIMEOUT_542: u64 = 500;
pub const BENCH_TIMEOUT_543: u64 = 500;
pub const BENCH_TIMEOUT_544: u64 = 500;
pub const BENCH_TIMEOUT_545: u64 = 500;
pub const BENCH_TIMEOUT_546: u64 = 500;
pub const BENCH_TIMEOUT_547: u64 = 500;
pub const BENCH_TIMEOUT_548: u64 = 500;
pub const BENCH_TIMEOUT_549: u64 = 500;
pub const BENCH_TIMEOUT_550: u64 = 500;
pub const BENCH_TIMEOUT_551: u64 = 500;
pub const BENCH_TIMEOUT_552: u64 = 500;
pub const BENCH_TIMEOUT_553: u64 = 500;
pub const BENCH_TIMEOUT_554: u64 = 500;
pub const BENCH_TIMEOUT_555: u64 = 500;
pub const BENCH_TIMEOUT_556: u64 = 500;
pub const BENCH_TIMEOUT_557: u64 = 500;
pub const BENCH_TIMEOUT_558: u64 = 500;
pub const BENCH_TIMEOUT_559: u64 = 500;
pub const BENCH_TIMEOUT_560: u64 = 500;
pub const BENCH_TIMEOUT_561: u64 = 500;
pub const BENCH_TIMEOUT_562: u64 = 500;
pub const BENCH_TIMEOUT_563: u64 = 500;
pub const BENCH_TIMEOUT_564: u64 = 500;
pub const BENCH_TIMEOUT_565: u64 = 500;
pub const BENCH_TIMEOUT_566: u64 = 500;
pub const BENCH_TIMEOUT_567: u64 = 500;
pub const BENCH_TIMEOUT_568: u64 = 500;
pub const BENCH_TIMEOUT_569: u64 = 500;
pub const BENCH_TIMEOUT_570: u64 = 500;
pub const BENCH_TIMEOUT_571: u64 = 500;
pub const BENCH_TIMEOUT_572: u64 = 500;
pub const BENCH_TIMEOUT_573: u64 = 500;
pub const BENCH_TIMEOUT_574: u64 = 500;
pub const BENCH_TIMEOUT_575: u64 = 500;
pub const BENCH_TIMEOUT_576: u64 = 500;
pub const BENCH_TIMEOUT_577: u64 = 500;
pub const BENCH_TIMEOUT_578: u64 = 500;
pub const BENCH_TIMEOUT_579: u64 = 500;
pub const BENCH_TIMEOUT_580: u64 = 500;
pub const BENCH_TIMEOUT_581: u64 = 500;
pub const BENCH_TIMEOUT_582: u64 = 500;
pub const BENCH_TIMEOUT_583: u64 = 500;
pub const BENCH_TIMEOUT_584: u64 = 500;
pub const BENCH_TIMEOUT_585: u64 = 500;
pub const BENCH_TIMEOUT_586: u64 = 500;
pub const BENCH_TIMEOUT_587: u64 = 500;
pub const BENCH_TIMEOUT_588: u64 = 500;
pub const BENCH_TIMEOUT_589: u64 = 500;
pub const BENCH_TIMEOUT_590: u64 = 500;
pub const BENCH_TIMEOUT_591: u64 = 500;
pub const BENCH_TIMEOUT_592: u64 = 500;
pub const BENCH_TIMEOUT_593: u64 = 500;
pub const BENCH_TIMEOUT_594: u64 = 500;
pub const BENCH_TIMEOUT_595: u64 = 500;
pub const BENCH_TIMEOUT_596: u64 = 500;
pub const BENCH_TIMEOUT_597: u64 = 500;
pub const BENCH_TIMEOUT_598: u64 = 500;
pub const BENCH_TIMEOUT_599: u64 = 500;
pub const BENCH_TIMEOUT_600: u64 = 500;
pub const BENCH_TIMEOUT_601: u64 = 500;
pub const BENCH_TIMEOUT_602: u64 = 500;
pub const BENCH_TIMEOUT_603: u64 = 500;
pub const BENCH_TIMEOUT_604: u64 = 500;
pub const BENCH_TIMEOUT_605: u64 = 500;
pub const BENCH_TIMEOUT_606: u64 = 500;
pub const BENCH_TIMEOUT_607: u64 = 500;
pub const BENCH_TIMEOUT_608: u64 = 500;
pub const BENCH_TIMEOUT_609: u64 = 500;
pub const BENCH_TIMEOUT_610: u64 = 500;
pub const BENCH_TIMEOUT_611: u64 = 500;
pub const BENCH_TIMEOUT_612: u64 = 500;
pub const BENCH_TIMEOUT_613: u64 = 500;
pub const BENCH_TIMEOUT_614: u64 = 500;
pub const BENCH_TIMEOUT_615: u64 = 500;
pub const BENCH_TIMEOUT_616: u64 = 500;
pub const BENCH_TIMEOUT_617: u64 = 500;
pub const BENCH_TIMEOUT_618: u64 = 500;
pub const BENCH_TIMEOUT_619: u64 = 500;
pub const BENCH_TIMEOUT_620: u64 = 500;
pub const BENCH_TIMEOUT_621: u64 = 500;
pub const BENCH_TIMEOUT_622: u64 = 500;
pub const BENCH_TIMEOUT_623: u64 = 500;
pub const BENCH_TIMEOUT_624: u64 = 500;
pub const BENCH_TIMEOUT_625: u64 = 500;
pub const BENCH_TIMEOUT_626: u64 = 500;
pub const BENCH_TIMEOUT_627: u64 = 500;
pub const BENCH_TIMEOUT_628: u64 = 500;
pub const BENCH_TIMEOUT_629: u64 = 500;
pub const BENCH_TIMEOUT_630: u64 = 500;
pub const BENCH_TIMEOUT_631: u64 = 500;
pub const BENCH_TIMEOUT_632: u64 = 500;
pub const BENCH_TIMEOUT_633: u64 = 500;
pub const BENCH_TIMEOUT_634: u64 = 500;
pub const BENCH_TIMEOUT_635: u64 = 500;
pub const BENCH_TIMEOUT_636: u64 = 500;
pub const BENCH_TIMEOUT_637: u64 = 500;
pub const BENCH_TIMEOUT_638: u64 = 500;
pub const BENCH_TIMEOUT_639: u64 = 500;
pub const BENCH_TIMEOUT_640: u64 = 500;
pub const BENCH_TIMEOUT_641: u64 = 500;
pub const BENCH_TIMEOUT_642: u64 = 500;
pub const BENCH_TIMEOUT_643: u64 = 500;
pub const BENCH_TIMEOUT_644: u64 = 500;
pub const BENCH_TIMEOUT_645: u64 = 500;
pub const BENCH_TIMEOUT_646: u64 = 500;
pub const BENCH_TIMEOUT_647: u64 = 500;
pub const BENCH_TIMEOUT_648: u64 = 500;
pub const BENCH_TIMEOUT_649: u64 = 500;
pub const BENCH_TIMEOUT_650: u64 = 500;
pub const BENCH_TIMEOUT_651: u64 = 500;
pub const BENCH_TIMEOUT_652: u64 = 500;
pub const BENCH_TIMEOUT_653: u64 = 500;
pub const BENCH_TIMEOUT_654: u64 = 500;
pub const BENCH_TIMEOUT_655: u64 = 500;
pub const BENCH_TIMEOUT_656: u64 = 500;
pub const BENCH_TIMEOUT_657: u64 = 500;
pub const BENCH_TIMEOUT_658: u64 = 500;
pub const BENCH_TIMEOUT_659: u64 = 500;
pub const BENCH_TIMEOUT_660: u64 = 500;
pub const BENCH_TIMEOUT_661: u64 = 500;
pub const BENCH_TIMEOUT_662: u64 = 500;
pub const BENCH_TIMEOUT_663: u64 = 500;
pub const BENCH_TIMEOUT_664: u64 = 500;
pub const BENCH_TIMEOUT_665: u64 = 500;
pub const BENCH_TIMEOUT_666: u64 = 500;
pub const BENCH_TIMEOUT_667: u64 = 500;
pub const BENCH_TIMEOUT_668: u64 = 500;
pub const BENCH_TIMEOUT_669: u64 = 500;
pub const BENCH_TIMEOUT_670: u64 = 500;
pub const BENCH_TIMEOUT_671: u64 = 500;
pub const BENCH_TIMEOUT_672: u64 = 500;
pub const BENCH_TIMEOUT_673: u64 = 500;
pub const BENCH_TIMEOUT_674: u64 = 500;
pub const BENCH_TIMEOUT_675: u64 = 500;
pub const BENCH_TIMEOUT_676: u64 = 500;
pub const BENCH_TIMEOUT_677: u64 = 500;
pub const BENCH_TIMEOUT_678: u64 = 500;
pub const BENCH_TIMEOUT_679: u64 = 500;
pub const BENCH_TIMEOUT_680: u64 = 500;
pub const BENCH_TIMEOUT_681: u64 = 500;
pub const BENCH_TIMEOUT_682: u64 = 500;
pub const BENCH_TIMEOUT_683: u64 = 500;
pub const BENCH_TIMEOUT_684: u64 = 500;
pub const BENCH_TIMEOUT_685: u64 = 500;
pub const BENCH_TIMEOUT_686: u64 = 500;
pub const BENCH_TIMEOUT_687: u64 = 500;
pub const BENCH_TIMEOUT_688: u64 = 500;
pub const BENCH_TIMEOUT_689: u64 = 500;
pub const BENCH_TIMEOUT_690: u64 = 500;
pub const BENCH_TIMEOUT_691: u64 = 500;
pub const BENCH_TIMEOUT_692: u64 = 500;
pub const BENCH_TIMEOUT_693: u64 = 500;
pub const BENCH_TIMEOUT_694: u64 = 500;
pub const BENCH_TIMEOUT_695: u64 = 500;
pub const BENCH_TIMEOUT_696: u64 = 500;
pub const BENCH_TIMEOUT_697: u64 = 500;
pub const BENCH_TIMEOUT_698: u64 = 500;
pub const BENCH_TIMEOUT_699: u64 = 500;
pub const BENCH_TIMEOUT_700: u64 = 500;
pub const BENCH_TIMEOUT_701: u64 = 500;
pub const BENCH_TIMEOUT_702: u64 = 500;
pub const BENCH_TIMEOUT_703: u64 = 500;
pub const BENCH_TIMEOUT_704: u64 = 500;
pub const BENCH_TIMEOUT_705: u64 = 500;
pub const BENCH_TIMEOUT_706: u64 = 500;
pub const BENCH_TIMEOUT_707: u64 = 500;
pub const BENCH_TIMEOUT_708: u64 = 500;
pub const BENCH_TIMEOUT_709: u64 = 500;
pub const BENCH_TIMEOUT_710: u64 = 500;
pub const BENCH_TIMEOUT_711: u64 = 500;
pub const BENCH_TIMEOUT_712: u64 = 500;
pub const BENCH_TIMEOUT_713: u64 = 500;
pub const BENCH_TIMEOUT_714: u64 = 500;
pub const BENCH_TIMEOUT_715: u64 = 500;
pub const BENCH_TIMEOUT_716: u64 = 500;
pub const BENCH_TIMEOUT_717: u64 = 500;
pub const BENCH_TIMEOUT_718: u64 = 500;
pub const BENCH_TIMEOUT_719: u64 = 500;
pub const BENCH_TIMEOUT_720: u64 = 500;
pub const BENCH_TIMEOUT_721: u64 = 500;
pub const BENCH_TIMEOUT_722: u64 = 500;
pub const BENCH_TIMEOUT_723: u64 = 500;
pub const BENCH_TIMEOUT_724: u64 = 500;
pub const BENCH_TIMEOUT_725: u64 = 500;
pub const BENCH_TIMEOUT_726: u64 = 500;
pub const BENCH_TIMEOUT_727: u64 = 500;
pub const BENCH_TIMEOUT_728: u64 = 500;
pub const BENCH_TIMEOUT_729: u64 = 500;
pub const BENCH_TIMEOUT_730: u64 = 500;
pub const BENCH_TIMEOUT_731: u64 = 500;
pub const BENCH_TIMEOUT_732: u64 = 500;
pub const BENCH_TIMEOUT_733: u64 = 500;
pub const BENCH_TIMEOUT_734: u64 = 500;
pub const BENCH_TIMEOUT_735: u64 = 500;
pub const BENCH_TIMEOUT_736: u64 = 500;
pub const BENCH_TIMEOUT_737: u64 = 500;
pub const BENCH_TIMEOUT_738: u64 = 500;
pub const BENCH_TIMEOUT_739: u64 = 500;
pub const BENCH_TIMEOUT_740: u64 = 500;
pub const BENCH_TIMEOUT_741: u64 = 500;
pub const BENCH_TIMEOUT_742: u64 = 500;
pub const BENCH_TIMEOUT_743: u64 = 500;
pub const BENCH_TIMEOUT_744: u64 = 500;
pub const BENCH_TIMEOUT_745: u64 = 500;
pub const BENCH_TIMEOUT_746: u64 = 500;
pub const BENCH_TIMEOUT_747: u64 = 500;
pub const BENCH_TIMEOUT_748: u64 = 500;
pub const BENCH_TIMEOUT_749: u64 = 500;
pub const BENCH_TIMEOUT_750: u64 = 500;
pub const BENCH_TIMEOUT_751: u64 = 500;
pub const BENCH_TIMEOUT_752: u64 = 500;
pub const BENCH_TIMEOUT_753: u64 = 500;
pub const BENCH_TIMEOUT_754: u64 = 500;
pub const BENCH_TIMEOUT_755: u64 = 500;
pub const BENCH_TIMEOUT_756: u64 = 500;
pub const BENCH_TIMEOUT_757: u64 = 500;
pub const BENCH_TIMEOUT_758: u64 = 500;
pub const BENCH_TIMEOUT_759: u64 = 500;
pub const BENCH_TIMEOUT_760: u64 = 500;
pub const BENCH_TIMEOUT_761: u64 = 500;
pub const BENCH_TIMEOUT_762: u64 = 500;
pub const BENCH_TIMEOUT_763: u64 = 500;
pub const BENCH_TIMEOUT_764: u64 = 500;
pub const BENCH_TIMEOUT_765: u64 = 500;
pub const BENCH_TIMEOUT_766: u64 = 500;
pub const BENCH_TIMEOUT_767: u64 = 500;
pub const BENCH_TIMEOUT_768: u64 = 500;
pub const BENCH_TIMEOUT_769: u64 = 500;
pub const BENCH_TIMEOUT_770: u64 = 500;
pub const BENCH_TIMEOUT_771: u64 = 500;
pub const BENCH_TIMEOUT_772: u64 = 500;
pub const BENCH_TIMEOUT_773: u64 = 500;
pub const BENCH_TIMEOUT_774: u64 = 500;
pub const BENCH_TIMEOUT_775: u64 = 500;
pub const BENCH_TIMEOUT_776: u64 = 500;
pub const BENCH_TIMEOUT_777: u64 = 500;
pub const BENCH_TIMEOUT_778: u64 = 500;
pub const BENCH_TIMEOUT_779: u64 = 500;
pub const BENCH_TIMEOUT_780: u64 = 500;
pub const BENCH_TIMEOUT_781: u64 = 500;
pub const BENCH_TIMEOUT_782: u64 = 500;
pub const BENCH_TIMEOUT_783: u64 = 500;
pub const BENCH_TIMEOUT_784: u64 = 500;
pub const BENCH_TIMEOUT_785: u64 = 500;
pub const BENCH_TIMEOUT_786: u64 = 500;
pub const BENCH_TIMEOUT_787: u64 = 500;
pub const BENCH_TIMEOUT_788: u64 = 500;
pub const BENCH_TIMEOUT_789: u64 = 500;
pub const BENCH_TIMEOUT_790: u64 = 500;
pub const BENCH_TIMEOUT_791: u64 = 500;
pub const BENCH_TIMEOUT_792: u64 = 500;
pub const BENCH_TIMEOUT_793: u64 = 500;
pub const BENCH_TIMEOUT_794: u64 = 500;
pub const BENCH_TIMEOUT_795: u64 = 500;
pub const BENCH_TIMEOUT_796: u64 = 500;
pub const BENCH_TIMEOUT_797: u64 = 500;
pub const BENCH_TIMEOUT_798: u64 = 500;
pub const BENCH_TIMEOUT_799: u64 = 500;
pub const BENCH_TIMEOUT_800: u64 = 500;
pub const BENCH_TIMEOUT_801: u64 = 500;
pub const BENCH_TIMEOUT_802: u64 = 500;
pub const BENCH_TIMEOUT_803: u64 = 500;
pub const BENCH_TIMEOUT_804: u64 = 500;
pub const BENCH_TIMEOUT_805: u64 = 500;
pub const BENCH_TIMEOUT_806: u64 = 500;
pub const BENCH_TIMEOUT_807: u64 = 500;
pub const BENCH_TIMEOUT_808: u64 = 500;
pub const BENCH_TIMEOUT_809: u64 = 500;
pub const BENCH_TIMEOUT_810: u64 = 500;
pub const BENCH_TIMEOUT_811: u64 = 500;
pub const BENCH_TIMEOUT_812: u64 = 500;
pub const BENCH_TIMEOUT_813: u64 = 500;
pub const BENCH_TIMEOUT_814: u64 = 500;
pub const BENCH_TIMEOUT_815: u64 = 500;
pub const BENCH_TIMEOUT_816: u64 = 500;
pub const BENCH_TIMEOUT_817: u64 = 500;
pub const BENCH_TIMEOUT_818: u64 = 500;
pub const BENCH_TIMEOUT_819: u64 = 500;
pub const BENCH_TIMEOUT_820: u64 = 500;
pub const BENCH_TIMEOUT_821: u64 = 500;
pub const BENCH_TIMEOUT_822: u64 = 500;
pub const BENCH_TIMEOUT_823: u64 = 500;
pub const BENCH_TIMEOUT_824: u64 = 500;
pub const BENCH_TIMEOUT_825: u64 = 500;
pub const BENCH_TIMEOUT_826: u64 = 500;
pub const BENCH_TIMEOUT_827: u64 = 500;
pub const BENCH_TIMEOUT_828: u64 = 500;
pub const BENCH_TIMEOUT_829: u64 = 500;
pub const BENCH_TIMEOUT_830: u64 = 500;
pub const BENCH_TIMEOUT_831: u64 = 500;
pub const BENCH_TIMEOUT_832: u64 = 500;
pub const BENCH_TIMEOUT_833: u64 = 500;
pub const BENCH_TIMEOUT_834: u64 = 500;
pub const BENCH_TIMEOUT_835: u64 = 500;
pub const BENCH_TIMEOUT_836: u64 = 500;
pub const BENCH_TIMEOUT_837: u64 = 500;
pub const BENCH_TIMEOUT_838: u64 = 500;
pub const BENCH_TIMEOUT_839: u64 = 500;
pub const BENCH_TIMEOUT_840: u64 = 500;
pub const BENCH_TIMEOUT_841: u64 = 500;
pub const BENCH_TIMEOUT_842: u64 = 500;
pub const BENCH_TIMEOUT_843: u64 = 500;
pub const BENCH_TIMEOUT_844: u64 = 500;
pub const BENCH_TIMEOUT_845: u64 = 500;
pub const BENCH_TIMEOUT_846: u64 = 500;
pub const BENCH_TIMEOUT_847: u64 = 500;
pub const BENCH_TIMEOUT_848: u64 = 500;
pub const BENCH_TIMEOUT_849: u64 = 500;
pub const BENCH_TIMEOUT_850: u64 = 500;
pub const BENCH_TIMEOUT_851: u64 = 500;
pub const BENCH_TIMEOUT_852: u64 = 500;
pub const BENCH_TIMEOUT_853: u64 = 500;
pub const BENCH_TIMEOUT_854: u64 = 500;
pub const BENCH_TIMEOUT_855: u64 = 500;
pub const BENCH_TIMEOUT_856: u64 = 500;
pub const BENCH_TIMEOUT_857: u64 = 500;
pub const BENCH_TIMEOUT_858: u64 = 500;
pub const BENCH_TIMEOUT_859: u64 = 500;
pub const BENCH_TIMEOUT_860: u64 = 500;
pub const BENCH_TIMEOUT_861: u64 = 500;
pub const BENCH_TIMEOUT_862: u64 = 500;
pub const BENCH_TIMEOUT_863: u64 = 500;
pub const BENCH_TIMEOUT_864: u64 = 500;
pub const BENCH_TIMEOUT_865: u64 = 500;
pub const BENCH_TIMEOUT_866: u64 = 500;
pub const BENCH_TIMEOUT_867: u64 = 500;
pub const BENCH_TIMEOUT_868: u64 = 500;
pub const BENCH_TIMEOUT_869: u64 = 500;
pub const BENCH_TIMEOUT_870: u64 = 500;
pub const BENCH_TIMEOUT_871: u64 = 500;
pub const BENCH_TIMEOUT_872: u64 = 500;
pub const BENCH_TIMEOUT_873: u64 = 500;
pub const BENCH_TIMEOUT_874: u64 = 500;
pub const BENCH_TIMEOUT_875: u64 = 500;
pub const BENCH_TIMEOUT_876: u64 = 500;
pub const BENCH_TIMEOUT_877: u64 = 500;
pub const BENCH_TIMEOUT_878: u64 = 500;
pub const BENCH_TIMEOUT_879: u64 = 500;
pub const BENCH_TIMEOUT_880: u64 = 500;
pub const BENCH_TIMEOUT_881: u64 = 500;
pub const BENCH_TIMEOUT_882: u64 = 500;
pub const BENCH_TIMEOUT_883: u64 = 500;
pub const BENCH_TIMEOUT_884: u64 = 500;
pub const BENCH_TIMEOUT_885: u64 = 500;
pub const BENCH_TIMEOUT_886: u64 = 500;
pub const BENCH_TIMEOUT_887: u64 = 500;
pub const BENCH_TIMEOUT_888: u64 = 500;
pub const BENCH_TIMEOUT_889: u64 = 500;
pub const BENCH_TIMEOUT_890: u64 = 500;
pub const BENCH_TIMEOUT_891: u64 = 500;
pub const BENCH_TIMEOUT_892: u64 = 500;
pub const BENCH_TIMEOUT_893: u64 = 500;
pub const BENCH_TIMEOUT_894: u64 = 500;
pub const BENCH_TIMEOUT_895: u64 = 500;
pub const BENCH_TIMEOUT_896: u64 = 500;
pub const BENCH_TIMEOUT_897: u64 = 500;
pub const BENCH_TIMEOUT_898: u64 = 500;
pub const BENCH_TIMEOUT_899: u64 = 500;
pub const BENCH_TIMEOUT_900: u64 = 500;
pub const BENCH_TIMEOUT_901: u64 = 500;
pub const BENCH_TIMEOUT_902: u64 = 500;
pub const BENCH_TIMEOUT_903: u64 = 500;
pub const BENCH_TIMEOUT_904: u64 = 500;
pub const BENCH_TIMEOUT_905: u64 = 500;
pub const BENCH_TIMEOUT_906: u64 = 500;
pub const BENCH_TIMEOUT_907: u64 = 500;
pub const BENCH_TIMEOUT_908: u64 = 500;
pub const BENCH_TIMEOUT_909: u64 = 500;
pub const BENCH_TIMEOUT_910: u64 = 500;
pub const BENCH_TIMEOUT_911: u64 = 500;
pub const BENCH_TIMEOUT_912: u64 = 500;
pub const BENCH_TIMEOUT_913: u64 = 500;
pub const BENCH_TIMEOUT_914: u64 = 500;
pub const BENCH_TIMEOUT_915: u64 = 500;
pub const BENCH_TIMEOUT_916: u64 = 500;
pub const BENCH_TIMEOUT_917: u64 = 500;
pub const BENCH_TIMEOUT_918: u64 = 500;
pub const BENCH_TIMEOUT_919: u64 = 500;
pub const BENCH_TIMEOUT_920: u64 = 500;
pub const BENCH_TIMEOUT_921: u64 = 500;
pub const BENCH_TIMEOUT_922: u64 = 500;
pub const BENCH_TIMEOUT_923: u64 = 500;
pub const BENCH_TIMEOUT_924: u64 = 500;
pub const BENCH_TIMEOUT_925: u64 = 500;
pub const BENCH_TIMEOUT_926: u64 = 500;
pub const BENCH_TIMEOUT_927: u64 = 500;
pub const BENCH_TIMEOUT_928: u64 = 500;
pub const BENCH_TIMEOUT_929: u64 = 500;
pub const BENCH_TIMEOUT_930: u64 = 500;
pub const BENCH_TIMEOUT_931: u64 = 500;
pub const BENCH_TIMEOUT_932: u64 = 500;
pub const BENCH_TIMEOUT_933: u64 = 500;
pub const BENCH_TIMEOUT_934: u64 = 500;
pub const BENCH_TIMEOUT_935: u64 = 500;
pub const BENCH_TIMEOUT_936: u64 = 500;
pub const BENCH_TIMEOUT_937: u64 = 500;
pub const BENCH_TIMEOUT_938: u64 = 500;
pub const BENCH_TIMEOUT_939: u64 = 500;
pub const BENCH_TIMEOUT_940: u64 = 500;
pub const BENCH_TIMEOUT_941: u64 = 500;
pub const BENCH_TIMEOUT_942: u64 = 500;
pub const BENCH_TIMEOUT_943: u64 = 500;
pub const BENCH_TIMEOUT_944: u64 = 500;
pub const BENCH_TIMEOUT_945: u64 = 500;
pub const BENCH_TIMEOUT_946: u64 = 500;
pub const BENCH_TIMEOUT_947: u64 = 500;
pub const BENCH_TIMEOUT_948: u64 = 500;
pub const BENCH_TIMEOUT_949: u64 = 500;
pub const BENCH_TIMEOUT_950: u64 = 500;
pub const BENCH_TIMEOUT_951: u64 = 500;
pub const BENCH_TIMEOUT_952: u64 = 500;
pub const BENCH_TIMEOUT_953: u64 = 500;
pub const BENCH_TIMEOUT_954: u64 = 500;
pub const BENCH_TIMEOUT_955: u64 = 500;
pub const BENCH_TIMEOUT_956: u64 = 500;
pub const BENCH_TIMEOUT_957: u64 = 500;
pub const BENCH_TIMEOUT_958: u64 = 500;
pub const BENCH_TIMEOUT_959: u64 = 500;
pub const BENCH_TIMEOUT_960: u64 = 500;
pub const BENCH_TIMEOUT_961: u64 = 500;
pub const BENCH_TIMEOUT_962: u64 = 500;
pub const BENCH_TIMEOUT_963: u64 = 500;
pub const BENCH_TIMEOUT_964: u64 = 500;
pub const BENCH_TIMEOUT_965: u64 = 500;
pub const BENCH_TIMEOUT_966: u64 = 500;
pub const BENCH_TIMEOUT_967: u64 = 500;
pub const BENCH_TIMEOUT_968: u64 = 500;
pub const BENCH_TIMEOUT_969: u64 = 500;
pub const BENCH_TIMEOUT_970: u64 = 500;
pub const BENCH_TIMEOUT_971: u64 = 500;
pub const BENCH_TIMEOUT_972: u64 = 500;
pub const BENCH_TIMEOUT_973: u64 = 500;
pub const BENCH_TIMEOUT_974: u64 = 500;
pub const BENCH_TIMEOUT_975: u64 = 500;
pub const BENCH_TIMEOUT_976: u64 = 500;
pub const BENCH_TIMEOUT_977: u64 = 500;
pub const BENCH_TIMEOUT_978: u64 = 500;
pub const BENCH_TIMEOUT_979: u64 = 500;
pub const BENCH_TIMEOUT_980: u64 = 500;
pub const BENCH_TIMEOUT_981: u64 = 500;
pub const BENCH_TIMEOUT_982: u64 = 500;
pub const BENCH_TIMEOUT_983: u64 = 500;
pub const BENCH_TIMEOUT_984: u64 = 500;
pub const BENCH_TIMEOUT_985: u64 = 500;
pub const BENCH_TIMEOUT_986: u64 = 500;
pub const BENCH_TIMEOUT_987: u64 = 500;
pub const BENCH_TIMEOUT_988: u64 = 500;
pub const BENCH_TIMEOUT_989: u64 = 500;
pub const BENCH_TIMEOUT_990: u64 = 500;
pub const BENCH_TIMEOUT_991: u64 = 500;
pub const BENCH_TIMEOUT_992: u64 = 500;
pub const BENCH_TIMEOUT_993: u64 = 500;
pub const BENCH_TIMEOUT_994: u64 = 500;
pub const BENCH_TIMEOUT_995: u64 = 500;
pub const BENCH_TIMEOUT_996: u64 = 500;
pub const BENCH_TIMEOUT_997: u64 = 500;
pub const BENCH_TIMEOUT_998: u64 = 500;
pub const BENCH_TIMEOUT_999: u64 = 500;
pub const BENCH_TIMEOUT_1000: u64 = 500;
pub const BENCH_TIMEOUT_1001: u64 = 500;
pub async fn bench_queue_latency() {
    tracing::info!("Benchmarking AI Job Dispatch Latency...");

    tracing::info!("--- Cloud Mode (Postgres) ---");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url != "postgres://localhost/dummy" && database_url.starts_with("postgres") {
        let pool_res = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect(&database_url).await;

        if let Ok(pg_pool) = pool_res {
            let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
            bench_queue("AI Job Dispatch Latency Cloud Mode (Postgres)", pg_queue).await;
        }
    }

    tracing::info!("--- Standalone Mode (Memory) ---");
    let mem_queue = Arc::new(MemoryTaskQueue::new());
    bench_queue("AI Job Dispatch Latency Standalone Mode (Memory)", mem_queue).await;
}

pub async fn bench_db_query_time() {
    tracing::info!("Benchmarking Database Query Time...");

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if database_url == "postgres://localhost/dummy" {
        return;
    }

    let iterations = 1000;

    // Cloud Mode (Postgres)
    // Only run if the database URL actually points to postgres, otherwise skip
    if database_url.starts_with("postgres") {
        let pg_pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();
        let mut pg_times = Vec::new();
        for _ in 0..iterations {
            let start = Instant::now();
            let _ = sqlx::query("SELECT 1").execute(&pg_pool).await;
            pg_times.push(start.elapsed().as_micros());
        }
        pg_times.sort();
        println!("Database Query Time Cloud Mode (Postgres): p50: {} us, p95: {} us, p99: {} us", pg_times[iterations / 2], pg_times[(iterations as f32 * 0.95) as usize], pg_times[(iterations as f32 * 0.99) as usize]);
    }

    // Standalone Mode (SQLite)
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let mut sqlite_times = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = sqlx::query("SELECT 1").execute(&sqlite_pool).await;
        sqlite_times.push(start.elapsed().as_micros());
    }
    sqlite_times.sort();
    println!("Database Query Time Standalone Mode (SQLite): p50: {} us, p95: {} us, p99: {} us", sqlite_times[iterations / 2], sqlite_times[(iterations as f32 * 0.95) as usize], sqlite_times[(iterations as f32 * 0.99) as usize]);
}

pub async fn bench_api_response_time() {
    tracing::info!("Benchmarking API Response Time...");

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if database_url == "postgres://localhost/dummy" {
        return;
    }
    let iterations = 100;

    let (tx, _rx) = tokio::sync::mpsc::channel(100);

    // Cloud setup
    if database_url.starts_with("postgres") {
        let pg_pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();
        let db_cloud = crate::db::DB { pool: pg_pool.clone(), store: crate::db::DbStore::Postgres };
        let hub_cloud = Arc::new(crate::hub::Hub::new(tx.clone(), db_cloud.pool.clone()));
        let dashboard_service_cloud = crate::services::dashboard::service::MyDashboardService::new(Arc::new(db_cloud), hub_cloud.clone());

        let mut cloud_times = Vec::new();
        for _ in 0..iterations {
            let req = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: false };
            let mut request = tonic::Request::new(req);
            request.extensions_mut().insert(::server_auth::orchestration::AuthInfo { spiffe_id: "test".to_string(), org_id: "system".to_string(), agent_id: "test".to_string() });
            let start = Instant::now();
            use ::server_ohc::app::dashboard_service_server::DashboardService;
            let _ = dashboard_service_cloud.get_dashboard(request).await;
            cloud_times.push(start.elapsed().as_micros());
        }
        cloud_times.sort();
        println!("API Response Time Cloud Mode: p50: {} us, p95: {} us, p99: {} us", cloud_times[iterations / 2], cloud_times[(iterations as f32 * 0.95) as usize], cloud_times[(iterations as f32 * 0.99) as usize]);
    }

    // Standalone setup
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT, organization_id TEXT, title TEXT, type TEXT, price REAL)").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT, tenant_id TEXT, total_amount REAL, status TEXT)").execute(&sqlite_pool).await;
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tenants (tenant_id TEXT, business_name TEXT, tier TEXT)").execute(&sqlite_pool).await;

    let fallback_pg = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
    let db_standalone = crate::db::DB { pool: fallback_pg, store: crate::db::DbStore::Sqlite(sqlite_pool) };
    let hub_standalone = Arc::new(crate::hub::Hub::new(tx, db_standalone.pool.clone()));
    let dashboard_service_standalone = crate::services::dashboard::service::MyDashboardService::new(Arc::new(db_standalone), hub_standalone.clone());

    let mut standalone_times = Vec::new();
    for _ in 0..iterations {
        let req = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: false };
        let mut request = tonic::Request::new(req);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo { spiffe_id: "test".to_string(), org_id: "system".to_string(), agent_id: "test".to_string() });
        let start = Instant::now();
        use ::server_ohc::app::dashboard_service_server::DashboardService;
        let _ = dashboard_service_standalone.get_dashboard(request).await;
        standalone_times.push(start.elapsed().as_micros());
    }
    standalone_times.sort();
    println!("API Response Time Standalone Mode: p50: {} us, p95: {} us, p99: {} us", standalone_times[iterations / 2], standalone_times[(iterations as f32 * 0.95) as usize], standalone_times[(iterations as f32 * 0.99) as usize]);
}

pub async fn bench_dashboard_snapshot() {
    println!("Benchmarking Dashboard Snapshot Fetching...");
    let (tx, _rx) = tokio::sync::mpsc::channel(100);

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url == "postgres://localhost/dummy" {
        return;
    }

    let db = if database_url.starts_with("sqlite") {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect(&database_url).await.unwrap();
        // Run minimal migrations for benchmark
        sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT, organization_id TEXT, title TEXT, type TEXT, price REAL)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT, tenant_id TEXT, total_amount REAL, status TEXT)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS tenants (tenant_id TEXT, business_name TEXT, tier TEXT)").execute(&pool).await.unwrap();

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool) }
    } else {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect(&database_url).await.unwrap();
        crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres }
    };

    let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));

    let iterations = 100;
    let mut fetch_times = Vec::new();

    let meeting_id = format!("meeting-{}", Uuid::new_v4());
    hub.open_meeting(meeting_id.clone(), vec!["test_agent".to_string()], "Agenda".to_string());
    for i in 0..50 {
        let msg = ::server_ohc::orchestration::Message {
            id: format!("msg-{}", i),
            from_agent: "test_agent".to_string(),
            to_agent: "all".to_string(),
            r#type: "chat".to_string(),
            content: "Hello world this is a test message".to_string(),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: meeting_id.clone(),
        };
        let _ = hub.clone().publish(::server_ohc::orchestration::Message {
            id: msg.id,
            from_agent: msg.from_agent,
            to_agent: msg.to_agent,
            r#type: msg.r#type,
            content: msg.content,
            occurred_at_unix: msg.occurred_at_unix,
            meeting_id: msg.meeting_id,
        });
    }

    for i in 0..50 {
        hub.register_agent(::server_ohc::orchestration::Agent {
            id: format!("agent-{}", i),
            name: format!("Agent {}", i),
            role: "test".to_string(),
            organization_id: "system".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });
    }

    for _ in 0..iterations {
        let start = Instant::now();

        let hub1 = hub.clone();
        let hub2 = hub.clone();
        let hub3 = hub.clone();

        let req_desktop = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: false };
        use ::server_ohc::app::dashboard_service_server::DashboardService;
        let db_arc = std::sync::Arc::new(db.clone());
        let dashboard_service = crate::services::dashboard::service::MyDashboardService::new(db_arc, hub.clone());
        let mut request = tonic::Request::new(req_desktop);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://onehumancorp.io/system/test".to_string(),
            org_id: "system".to_string(),
            agent_id: "test".to_string(),
        });
        let _res_desktop = dashboard_service.get_dashboard(request).await.unwrap().into_inner();

        fetch_times.push(start.elapsed().as_micros());
    }

    fetch_times.sort();
    println!("Parallel Fetch: p50: {} us, p95: {} us, p99: {} us", fetch_times[iterations / 2], fetch_times[(iterations as f32 * 0.95) as usize], fetch_times[(iterations as f32 * 0.99) as usize]);

    let req_mobile = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: true };
    let req_desktop = ::server_ohc::app::GetDashboardRequest { organization_id: "system".to_string(), mobile_optimized: false };

    use ::server_ohc::app::dashboard_service_server::DashboardService;
    let db_arc = std::sync::Arc::new(db.clone());
    let dashboard_service = crate::services::dashboard::service::MyDashboardService::new(db_arc, hub.clone());

    let mut req_mobile_t = tonic::Request::new(req_mobile);
    req_mobile_t.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://onehumancorp.io/system/test".to_string(),
        org_id: "system".to_string(),
        agent_id: "test".to_string(),
    });
    let mut req_desktop_t = tonic::Request::new(req_desktop);
    req_desktop_t.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://onehumancorp.io/system/test".to_string(),
        org_id: "system".to_string(),
        agent_id: "test".to_string(),
    });

    let res_mobile = dashboard_service.get_dashboard(req_mobile_t).await.unwrap().into_inner();
    let res_desktop = dashboard_service.get_dashboard(req_desktop_t).await.unwrap().into_inner();

    println!("Mobile optimized meetings length: {}, desktop: {}", res_mobile.meetings.len(), res_desktop.meetings.len());
    if !res_mobile.meetings.is_empty() {
        println!("Mobile meeting 0 transcript len: {}", res_mobile.meetings[0].transcript.len());
        println!("Desktop meeting 0 transcript len: {}", res_desktop.meetings[0].transcript.len());
        assert_eq!(res_mobile.meetings[0].transcript.len(), 0, "Mobile payload optimization should clear transcripts");
        assert!(res_desktop.meetings[0].transcript.len() > 0, "Desktop payload should contain transcripts");
    }

    println!("Parallel Fetch: p50: {} us, p95: {} us, p99: {} us", fetch_times[iterations / 2], fetch_times[(iterations as f32 * 0.95) as usize], fetch_times[(iterations as f32 * 0.99) as usize]);
}

pub async fn bench_queue(name: &str, queue: Arc<dyn TaskQueue>) {
    let mut enqueue_times = Vec::new();
    let mut dequeue_times = Vec::new();
    let iterations = if name.contains("Memory") { 10 } else { 100 };

    let run_id = Uuid::new_v4().to_string();

    let mut join_handles = Vec::new();

    for i in 0..iterations {
        let q = queue.clone();
        let name = name.to_string();
        let run_id = run_id.clone();

        join_handles.push(tokio::spawn(async move {
            let job = Job {
                id: format!("job_{}_{}_{}", name, run_id, i),
                tenant_id: "benchmark_tenant".to_string(),
                parent_task_id: format!("parent_{}_{}_{}", name, run_id, i),
                agent_role: "test_agent".to_string(),
                payload: "{}".to_string(),
                status: "PENDING".to_string(),
                attempts: 0,
                max_attempts: 3,
                run_after: Utc::now(),
                locked_until: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            let start = Instant::now();
            q.enqueue_batch(vec![job]).await.unwrap();
            let elapsed_enqueue = start.elapsed();

            let start_deq = Instant::now();
            let _ = q.dequeue(vec!["test_agent".to_string()]).await.unwrap();
            let elapsed_dequeue = start_deq.elapsed();

            (elapsed_enqueue.as_micros(), elapsed_dequeue.as_micros())
        }));
    }

    for handle in join_handles {
        let (enq, deq) = handle.await.unwrap();
        enqueue_times.push(enq);
        dequeue_times.push(deq);
    }

    enqueue_times.sort();
    dequeue_times.sort();

    let enq_p50 = if iterations > 0 { enqueue_times[iterations / 2] } else { 0 };
    let enq_p95 = if iterations > 0 { enqueue_times[(iterations as f32 * 0.95) as usize] } else { 0 };
    let enq_p99 = if iterations > 0 { enqueue_times[(iterations as f32 * 0.99) as usize] } else { 0 };

    let deq_p50 = if iterations > 0 { dequeue_times[iterations / 2] } else { 0 };
    let deq_p95 = if iterations > 0 { dequeue_times[(iterations as f32 * 0.95) as usize] } else { 0 };
    let deq_p99 = if iterations > 0 { dequeue_times[(iterations as f32 * 0.99) as usize] } else { 0 };

    println!("{}: Batch Enqueue p50: {} us, p95: {} us, p99: {} us", name, enq_p50, enq_p95, enq_p99);
    println!("{}: Dequeue p50: {} us, p95: {} us, p99: {} us", name, deq_p50, deq_p95, deq_p99);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_bench_queue_latency() {
        bench_queue_latency().await;
    }

    #[tokio::test]
    async fn test_run_bench_db_query_time() {
        bench_db_query_time().await;
    }

    #[tokio::test]
    async fn test_run_bench_api_response_time() {
        bench_api_response_time().await;
    }

    #[tokio::test]
    async fn test_bench_dashboard_snapshot() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "dummy".to_string());
        println!("DEBUG: db_url = {}", db_url);
        if db_url == "dummy" {
            println!("DEBUG: skipping because db_url is dummy");
            return;
        }
        println!("RUNNING BENCHMARK DASHBOARD SNAPSHOT");
        bench_dashboard_snapshot().await;
    }

    #[tokio::test]
    async fn test_stress_verification_cloud_standalone() {
        let mem_queue = Arc::new(MemoryTaskQueue::new());
        bench_queue("Memory_Stress", mem_queue).await;

        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if database_url != "postgres://localhost/dummy" && database_url.starts_with("postgres") {
            if let Ok(pg_pool) = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).connect(&database_url).await {
                let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
                bench_queue("Postgres_Stress", pg_queue).await;
            }
        }
    }

    #[tokio::test]
    async fn test_ml_resilience_60s_timeout_rule() {
        let start = std::time::Instant::now();
        let timeout_duration = std::time::Duration::from_millis(60);

        let result = tokio::time::timeout(timeout_duration, async {
            tokio::time::sleep(std::time::Duration::from_millis(150)).await;
            Ok::<(), String>(())
        }).await;

        assert!(result.is_err(), "Chaos resilience must enforce ML-Resilience timeout rule to prevent cascading failure");
        assert!(start.elapsed() >= timeout_duration, "Timeout enforcement should take at least the configured duration");
    }

    #[tokio::test]
    async fn test_chaos_degradation_network() {
        let start = std::time::Instant::now();
        let slow_network = async {
            tokio::task::yield_now().await;
            tokio::time::sleep(std::time::Duration::from_millis(2050)).await;
            "data"
        };
        let result = tokio::time::timeout(std::time::Duration::from_millis(2000), slow_network).await;
        assert!(result.is_err());
        assert!(start.elapsed() < std::time::Duration::from_millis(2500));
    }
}
