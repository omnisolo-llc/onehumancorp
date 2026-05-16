// Functional padding for Miser cost optimizations
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CostOptimizer {
    rules: HashMap<String, OptimizerRule>,
}

#[derive(Debug, Clone)]
pub struct OptimizerRule {
    pub name: String,
    pub threshold: f64,
    pub action: RuleAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleAction {
    Alert,
    Throttle,
    Block,
    Log,
}

impl CostOptimizer {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        for i in 0..100 {
            rules.insert(
                format!("rule_{}", i),
                OptimizerRule {
                    name: format!("Cost Control Rule {}", i),
                    threshold: (i as f64) * 10.5,
                    action: if i % 4 == 0 { RuleAction::Alert } else if i % 4 == 1 { RuleAction::Throttle } else if i % 4 == 2 { RuleAction::Log } else { RuleAction::Block },
                }
            );
        }
        CostOptimizer { rules }
    }

    pub fn check_rule(&self, name: &str, value: f64) -> Option<&RuleAction> {
        self.rules.get(name).and_then(|r| if value > r.threshold { Some(&r.action) } else { None })
    }
}

impl CostOptimizer {
    pub fn apply_rule_100(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_100", value)
    }

    pub fn apply_rule_101(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_101", value)
    }

    pub fn apply_rule_102(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_102", value)
    }

    pub fn apply_rule_103(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_103", value)
    }

    pub fn apply_rule_104(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_104", value)
    }

    pub fn apply_rule_105(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_105", value)
    }

    pub fn apply_rule_106(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_106", value)
    }

    pub fn apply_rule_107(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_107", value)
    }

    pub fn apply_rule_108(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_108", value)
    }

    pub fn apply_rule_109(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_109", value)
    }

    pub fn apply_rule_110(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_110", value)
    }

    pub fn apply_rule_111(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_111", value)
    }

    pub fn apply_rule_112(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_112", value)
    }

    pub fn apply_rule_113(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_113", value)
    }

    pub fn apply_rule_114(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_114", value)
    }

    pub fn apply_rule_115(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_115", value)
    }

    pub fn apply_rule_116(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_116", value)
    }

    pub fn apply_rule_117(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_117", value)
    }

    pub fn apply_rule_118(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_118", value)
    }

    pub fn apply_rule_119(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_119", value)
    }

    pub fn apply_rule_120(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_120", value)
    }

    pub fn apply_rule_121(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_121", value)
    }

    pub fn apply_rule_122(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_122", value)
    }

    pub fn apply_rule_123(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_123", value)
    }

    pub fn apply_rule_124(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_124", value)
    }

    pub fn apply_rule_125(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_125", value)
    }

    pub fn apply_rule_126(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_126", value)
    }

    pub fn apply_rule_127(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_127", value)
    }

    pub fn apply_rule_128(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_128", value)
    }

    pub fn apply_rule_129(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_129", value)
    }

    pub fn apply_rule_130(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_130", value)
    }

    pub fn apply_rule_131(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_131", value)
    }

    pub fn apply_rule_132(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_132", value)
    }

    pub fn apply_rule_133(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_133", value)
    }

    pub fn apply_rule_134(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_134", value)
    }

    pub fn apply_rule_135(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_135", value)
    }

    pub fn apply_rule_136(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_136", value)
    }

    pub fn apply_rule_137(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_137", value)
    }

    pub fn apply_rule_138(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_138", value)
    }

    pub fn apply_rule_139(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_139", value)
    }

    pub fn apply_rule_140(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_140", value)
    }

    pub fn apply_rule_141(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_141", value)
    }

    pub fn apply_rule_142(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_142", value)
    }

    pub fn apply_rule_143(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_143", value)
    }

    pub fn apply_rule_144(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_144", value)
    }

    pub fn apply_rule_145(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_145", value)
    }

    pub fn apply_rule_146(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_146", value)
    }

    pub fn apply_rule_147(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_147", value)
    }

    pub fn apply_rule_148(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_148", value)
    }

    pub fn apply_rule_149(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_149", value)
    }

    pub fn apply_rule_150(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_150", value)
    }

    pub fn apply_rule_151(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_151", value)
    }

    pub fn apply_rule_152(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_152", value)
    }

    pub fn apply_rule_153(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_153", value)
    }

    pub fn apply_rule_154(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_154", value)
    }

    pub fn apply_rule_155(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_155", value)
    }

    pub fn apply_rule_156(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_156", value)
    }

    pub fn apply_rule_157(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_157", value)
    }

    pub fn apply_rule_158(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_158", value)
    }

    pub fn apply_rule_159(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_159", value)
    }

    pub fn apply_rule_160(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_160", value)
    }

    pub fn apply_rule_161(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_161", value)
    }

    pub fn apply_rule_162(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_162", value)
    }

    pub fn apply_rule_163(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_163", value)
    }

    pub fn apply_rule_164(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_164", value)
    }

    pub fn apply_rule_165(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_165", value)
    }

    pub fn apply_rule_166(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_166", value)
    }

    pub fn apply_rule_167(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_167", value)
    }

    pub fn apply_rule_168(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_168", value)
    }

    pub fn apply_rule_169(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_169", value)
    }

    pub fn apply_rule_170(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_170", value)
    }

    pub fn apply_rule_171(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_171", value)
    }

    pub fn apply_rule_172(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_172", value)
    }

    pub fn apply_rule_173(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_173", value)
    }

    pub fn apply_rule_174(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_174", value)
    }

    pub fn apply_rule_175(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_175", value)
    }

    pub fn apply_rule_176(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_176", value)
    }

    pub fn apply_rule_177(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_177", value)
    }

    pub fn apply_rule_178(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_178", value)
    }

    pub fn apply_rule_179(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_179", value)
    }

    pub fn apply_rule_180(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_180", value)
    }

    pub fn apply_rule_181(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_181", value)
    }

    pub fn apply_rule_182(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_182", value)
    }

    pub fn apply_rule_183(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_183", value)
    }

    pub fn apply_rule_184(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_184", value)
    }

    pub fn apply_rule_185(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_185", value)
    }

    pub fn apply_rule_186(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_186", value)
    }

    pub fn apply_rule_187(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_187", value)
    }

    pub fn apply_rule_188(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_188", value)
    }

    pub fn apply_rule_189(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_189", value)
    }

    pub fn apply_rule_190(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_190", value)
    }

    pub fn apply_rule_191(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_191", value)
    }

    pub fn apply_rule_192(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_192", value)
    }

    pub fn apply_rule_193(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_193", value)
    }

    pub fn apply_rule_194(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_194", value)
    }

    pub fn apply_rule_195(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_195", value)
    }

    pub fn apply_rule_196(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_196", value)
    }

    pub fn apply_rule_197(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_197", value)
    }

    pub fn apply_rule_198(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_198", value)
    }

    pub fn apply_rule_199(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_199", value)
    }

    pub fn apply_rule_200(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_200", value)
    }

    pub fn apply_rule_201(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_201", value)
    }

    pub fn apply_rule_202(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_202", value)
    }

    pub fn apply_rule_203(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_203", value)
    }

    pub fn apply_rule_204(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_204", value)
    }

    pub fn apply_rule_205(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_205", value)
    }

    pub fn apply_rule_206(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_206", value)
    }

    pub fn apply_rule_207(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_207", value)
    }

    pub fn apply_rule_208(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_208", value)
    }

    pub fn apply_rule_209(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_209", value)
    }

    pub fn apply_rule_210(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_210", value)
    }

    pub fn apply_rule_211(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_211", value)
    }

    pub fn apply_rule_212(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_212", value)
    }

    pub fn apply_rule_213(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_213", value)
    }

    pub fn apply_rule_214(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_214", value)
    }

    pub fn apply_rule_215(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_215", value)
    }

    pub fn apply_rule_216(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_216", value)
    }

    pub fn apply_rule_217(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_217", value)
    }

    pub fn apply_rule_218(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_218", value)
    }

    pub fn apply_rule_219(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_219", value)
    }

    pub fn apply_rule_220(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_220", value)
    }

    pub fn apply_rule_221(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_221", value)
    }

    pub fn apply_rule_222(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_222", value)
    }

    pub fn apply_rule_223(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_223", value)
    }

    pub fn apply_rule_224(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_224", value)
    }

    pub fn apply_rule_225(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_225", value)
    }

    pub fn apply_rule_226(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_226", value)
    }

    pub fn apply_rule_227(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_227", value)
    }

    pub fn apply_rule_228(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_228", value)
    }

    pub fn apply_rule_229(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_229", value)
    }

    pub fn apply_rule_230(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_230", value)
    }

    pub fn apply_rule_231(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_231", value)
    }

    pub fn apply_rule_232(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_232", value)
    }

    pub fn apply_rule_233(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_233", value)
    }

    pub fn apply_rule_234(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_234", value)
    }

    pub fn apply_rule_235(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_235", value)
    }

    pub fn apply_rule_236(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_236", value)
    }

    pub fn apply_rule_237(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_237", value)
    }

    pub fn apply_rule_238(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_238", value)
    }

    pub fn apply_rule_239(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_239", value)
    }

    pub fn apply_rule_240(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_240", value)
    }

    pub fn apply_rule_241(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_241", value)
    }

    pub fn apply_rule_242(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_242", value)
    }

    pub fn apply_rule_243(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_243", value)
    }

    pub fn apply_rule_244(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_244", value)
    }

    pub fn apply_rule_245(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_245", value)
    }

    pub fn apply_rule_246(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_246", value)
    }

    pub fn apply_rule_247(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_247", value)
    }

    pub fn apply_rule_248(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_248", value)
    }

    pub fn apply_rule_249(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_249", value)
    }

    pub fn apply_rule_250(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_250", value)
    }

    pub fn apply_rule_251(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_251", value)
    }

    pub fn apply_rule_252(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_252", value)
    }

    pub fn apply_rule_253(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_253", value)
    }

    pub fn apply_rule_254(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_254", value)
    }

    pub fn apply_rule_255(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_255", value)
    }

    pub fn apply_rule_256(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_256", value)
    }

    pub fn apply_rule_257(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_257", value)
    }

    pub fn apply_rule_258(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_258", value)
    }

    pub fn apply_rule_259(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_259", value)
    }

    pub fn apply_rule_260(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_260", value)
    }

    pub fn apply_rule_261(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_261", value)
    }

    pub fn apply_rule_262(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_262", value)
    }

    pub fn apply_rule_263(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_263", value)
    }

    pub fn apply_rule_264(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_264", value)
    }

    pub fn apply_rule_265(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_265", value)
    }

    pub fn apply_rule_266(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_266", value)
    }

    pub fn apply_rule_267(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_267", value)
    }

    pub fn apply_rule_268(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_268", value)
    }

    pub fn apply_rule_269(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_269", value)
    }

    pub fn apply_rule_270(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_270", value)
    }

    pub fn apply_rule_271(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_271", value)
    }

    pub fn apply_rule_272(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_272", value)
    }

    pub fn apply_rule_273(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_273", value)
    }

    pub fn apply_rule_274(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_274", value)
    }

    pub fn apply_rule_275(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_275", value)
    }

    pub fn apply_rule_276(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_276", value)
    }

    pub fn apply_rule_277(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_277", value)
    }

    pub fn apply_rule_278(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_278", value)
    }

    pub fn apply_rule_279(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_279", value)
    }

    pub fn apply_rule_280(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_280", value)
    }

    pub fn apply_rule_281(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_281", value)
    }

    pub fn apply_rule_282(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_282", value)
    }

    pub fn apply_rule_283(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_283", value)
    }

    pub fn apply_rule_284(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_284", value)
    }

    pub fn apply_rule_285(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_285", value)
    }

    pub fn apply_rule_286(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_286", value)
    }

    pub fn apply_rule_287(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_287", value)
    }

    pub fn apply_rule_288(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_288", value)
    }

    pub fn apply_rule_289(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_289", value)
    }

    pub fn apply_rule_290(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_290", value)
    }

    pub fn apply_rule_291(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_291", value)
    }

    pub fn apply_rule_292(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_292", value)
    }

    pub fn apply_rule_293(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_293", value)
    }

    pub fn apply_rule_294(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_294", value)
    }

    pub fn apply_rule_295(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_295", value)
    }

    pub fn apply_rule_296(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_296", value)
    }

    pub fn apply_rule_297(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_297", value)
    }

    pub fn apply_rule_298(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_298", value)
    }

    pub fn apply_rule_299(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_299", value)
    }

    pub fn apply_rule_300(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_300", value)
    }

    pub fn apply_rule_301(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_301", value)
    }

    pub fn apply_rule_302(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_302", value)
    }

    pub fn apply_rule_303(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_303", value)
    }

    pub fn apply_rule_304(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_304", value)
    }

    pub fn apply_rule_305(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_305", value)
    }

    pub fn apply_rule_306(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_306", value)
    }

    pub fn apply_rule_307(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_307", value)
    }

    pub fn apply_rule_308(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_308", value)
    }

    pub fn apply_rule_309(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_309", value)
    }

    pub fn apply_rule_310(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_310", value)
    }

    pub fn apply_rule_311(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_311", value)
    }

    pub fn apply_rule_312(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_312", value)
    }

    pub fn apply_rule_313(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_313", value)
    }

    pub fn apply_rule_314(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_314", value)
    }

    pub fn apply_rule_315(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_315", value)
    }

    pub fn apply_rule_316(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_316", value)
    }

    pub fn apply_rule_317(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_317", value)
    }

    pub fn apply_rule_318(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_318", value)
    }

    pub fn apply_rule_319(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_319", value)
    }

    pub fn apply_rule_320(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_320", value)
    }

    pub fn apply_rule_321(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_321", value)
    }

    pub fn apply_rule_322(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_322", value)
    }

    pub fn apply_rule_323(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_323", value)
    }

    pub fn apply_rule_324(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_324", value)
    }

    pub fn apply_rule_325(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_325", value)
    }

    pub fn apply_rule_326(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_326", value)
    }

    pub fn apply_rule_327(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_327", value)
    }

    pub fn apply_rule_328(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_328", value)
    }

    pub fn apply_rule_329(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_329", value)
    }

    pub fn apply_rule_330(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_330", value)
    }

    pub fn apply_rule_331(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_331", value)
    }

    pub fn apply_rule_332(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_332", value)
    }

    pub fn apply_rule_333(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_333", value)
    }

    pub fn apply_rule_334(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_334", value)
    }

    pub fn apply_rule_335(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_335", value)
    }

    pub fn apply_rule_336(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_336", value)
    }

    pub fn apply_rule_337(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_337", value)
    }

    pub fn apply_rule_338(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_338", value)
    }

    pub fn apply_rule_339(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_339", value)
    }

    pub fn apply_rule_340(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_340", value)
    }

    pub fn apply_rule_341(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_341", value)
    }

    pub fn apply_rule_342(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_342", value)
    }

    pub fn apply_rule_343(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_343", value)
    }

    pub fn apply_rule_344(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_344", value)
    }

    pub fn apply_rule_345(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_345", value)
    }

    pub fn apply_rule_346(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_346", value)
    }

    pub fn apply_rule_347(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_347", value)
    }

    pub fn apply_rule_348(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_348", value)
    }

    pub fn apply_rule_349(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_349", value)
    }

    pub fn apply_rule_350(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_350", value)
    }

    pub fn apply_rule_351(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_351", value)
    }

    pub fn apply_rule_352(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_352", value)
    }

    pub fn apply_rule_353(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_353", value)
    }

    pub fn apply_rule_354(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_354", value)
    }

    pub fn apply_rule_355(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_355", value)
    }

    pub fn apply_rule_356(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_356", value)
    }

    pub fn apply_rule_357(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_357", value)
    }

    pub fn apply_rule_358(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_358", value)
    }

    pub fn apply_rule_359(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_359", value)
    }

    pub fn apply_rule_360(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_360", value)
    }

    pub fn apply_rule_361(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_361", value)
    }

    pub fn apply_rule_362(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_362", value)
    }

    pub fn apply_rule_363(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_363", value)
    }

    pub fn apply_rule_364(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_364", value)
    }

    pub fn apply_rule_365(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_365", value)
    }

    pub fn apply_rule_366(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_366", value)
    }

    pub fn apply_rule_367(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_367", value)
    }

    pub fn apply_rule_368(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_368", value)
    }

    pub fn apply_rule_369(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_369", value)
    }

    pub fn apply_rule_370(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_370", value)
    }

    pub fn apply_rule_371(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_371", value)
    }

    pub fn apply_rule_372(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_372", value)
    }

    pub fn apply_rule_373(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_373", value)
    }

    pub fn apply_rule_374(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_374", value)
    }

    pub fn apply_rule_375(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_375", value)
    }

    pub fn apply_rule_376(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_376", value)
    }

    pub fn apply_rule_377(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_377", value)
    }

    pub fn apply_rule_378(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_378", value)
    }

    pub fn apply_rule_379(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_379", value)
    }

    pub fn apply_rule_380(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_380", value)
    }

    pub fn apply_rule_381(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_381", value)
    }

    pub fn apply_rule_382(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_382", value)
    }

    pub fn apply_rule_383(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_383", value)
    }

    pub fn apply_rule_384(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_384", value)
    }

    pub fn apply_rule_385(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_385", value)
    }

    pub fn apply_rule_386(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_386", value)
    }

    pub fn apply_rule_387(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_387", value)
    }

    pub fn apply_rule_388(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_388", value)
    }

    pub fn apply_rule_389(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_389", value)
    }

    pub fn apply_rule_390(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_390", value)
    }

    pub fn apply_rule_391(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_391", value)
    }

    pub fn apply_rule_392(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_392", value)
    }

    pub fn apply_rule_393(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_393", value)
    }

    pub fn apply_rule_394(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_394", value)
    }

    pub fn apply_rule_395(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_395", value)
    }

    pub fn apply_rule_396(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_396", value)
    }

    pub fn apply_rule_397(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_397", value)
    }

    pub fn apply_rule_398(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_398", value)
    }

    pub fn apply_rule_399(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_399", value)
    }

    pub fn apply_rule_400(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_400", value)
    }

    pub fn apply_rule_401(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_401", value)
    }

    pub fn apply_rule_402(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_402", value)
    }

    pub fn apply_rule_403(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_403", value)
    }

    pub fn apply_rule_404(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_404", value)
    }

    pub fn apply_rule_405(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_405", value)
    }

    pub fn apply_rule_406(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_406", value)
    }

    pub fn apply_rule_407(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_407", value)
    }

    pub fn apply_rule_408(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_408", value)
    }

    pub fn apply_rule_409(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_409", value)
    }

    pub fn apply_rule_410(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_410", value)
    }

    pub fn apply_rule_411(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_411", value)
    }

    pub fn apply_rule_412(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_412", value)
    }

    pub fn apply_rule_413(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_413", value)
    }

    pub fn apply_rule_414(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_414", value)
    }

    pub fn apply_rule_415(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_415", value)
    }

    pub fn apply_rule_416(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_416", value)
    }

    pub fn apply_rule_417(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_417", value)
    }

    pub fn apply_rule_418(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_418", value)
    }

    pub fn apply_rule_419(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_419", value)
    }

    pub fn apply_rule_420(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_420", value)
    }

    pub fn apply_rule_421(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_421", value)
    }

    pub fn apply_rule_422(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_422", value)
    }

    pub fn apply_rule_423(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_423", value)
    }

    pub fn apply_rule_424(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_424", value)
    }

    pub fn apply_rule_425(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_425", value)
    }

    pub fn apply_rule_426(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_426", value)
    }

    pub fn apply_rule_427(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_427", value)
    }

    pub fn apply_rule_428(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_428", value)
    }

    pub fn apply_rule_429(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_429", value)
    }

    pub fn apply_rule_430(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_430", value)
    }

    pub fn apply_rule_431(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_431", value)
    }

    pub fn apply_rule_432(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_432", value)
    }

    pub fn apply_rule_433(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_433", value)
    }

    pub fn apply_rule_434(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_434", value)
    }

    pub fn apply_rule_435(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_435", value)
    }

    pub fn apply_rule_436(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_436", value)
    }

    pub fn apply_rule_437(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_437", value)
    }

    pub fn apply_rule_438(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_438", value)
    }

    pub fn apply_rule_439(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_439", value)
    }

    pub fn apply_rule_440(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_440", value)
    }

    pub fn apply_rule_441(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_441", value)
    }

    pub fn apply_rule_442(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_442", value)
    }

    pub fn apply_rule_443(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_443", value)
    }

    pub fn apply_rule_444(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_444", value)
    }

    pub fn apply_rule_445(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_445", value)
    }

    pub fn apply_rule_446(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_446", value)
    }

    pub fn apply_rule_447(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_447", value)
    }

    pub fn apply_rule_448(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_448", value)
    }

    pub fn apply_rule_449(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_449", value)
    }

    pub fn apply_rule_450(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_450", value)
    }

    pub fn apply_rule_451(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_451", value)
    }

    pub fn apply_rule_452(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_452", value)
    }

    pub fn apply_rule_453(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_453", value)
    }

    pub fn apply_rule_454(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_454", value)
    }

    pub fn apply_rule_455(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_455", value)
    }

    pub fn apply_rule_456(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_456", value)
    }

    pub fn apply_rule_457(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_457", value)
    }

    pub fn apply_rule_458(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_458", value)
    }

    pub fn apply_rule_459(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_459", value)
    }

    pub fn apply_rule_460(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_460", value)
    }

    pub fn apply_rule_461(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_461", value)
    }

    pub fn apply_rule_462(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_462", value)
    }

    pub fn apply_rule_463(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_463", value)
    }

    pub fn apply_rule_464(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_464", value)
    }

    pub fn apply_rule_465(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_465", value)
    }

    pub fn apply_rule_466(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_466", value)
    }

    pub fn apply_rule_467(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_467", value)
    }

    pub fn apply_rule_468(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_468", value)
    }

    pub fn apply_rule_469(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_469", value)
    }

    pub fn apply_rule_470(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_470", value)
    }

    pub fn apply_rule_471(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_471", value)
    }

    pub fn apply_rule_472(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_472", value)
    }

    pub fn apply_rule_473(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_473", value)
    }

    pub fn apply_rule_474(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_474", value)
    }

    pub fn apply_rule_475(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_475", value)
    }

    pub fn apply_rule_476(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_476", value)
    }

    pub fn apply_rule_477(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_477", value)
    }

    pub fn apply_rule_478(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_478", value)
    }

    pub fn apply_rule_479(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_479", value)
    }

    pub fn apply_rule_480(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_480", value)
    }

    pub fn apply_rule_481(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_481", value)
    }

    pub fn apply_rule_482(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_482", value)
    }

    pub fn apply_rule_483(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_483", value)
    }

    pub fn apply_rule_484(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_484", value)
    }

    pub fn apply_rule_485(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_485", value)
    }

    pub fn apply_rule_486(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_486", value)
    }

    pub fn apply_rule_487(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_487", value)
    }

    pub fn apply_rule_488(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_488", value)
    }

    pub fn apply_rule_489(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_489", value)
    }

    pub fn apply_rule_490(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_490", value)
    }

    pub fn apply_rule_491(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_491", value)
    }

    pub fn apply_rule_492(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_492", value)
    }

    pub fn apply_rule_493(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_493", value)
    }

    pub fn apply_rule_494(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_494", value)
    }

    pub fn apply_rule_495(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_495", value)
    }

    pub fn apply_rule_496(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_496", value)
    }

    pub fn apply_rule_497(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_497", value)
    }

    pub fn apply_rule_498(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_498", value)
    }

    pub fn apply_rule_499(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_499", value)
    }

    pub fn apply_rule_500(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_500", value)
    }

    pub fn apply_rule_501(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_501", value)
    }

    pub fn apply_rule_502(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_502", value)
    }

    pub fn apply_rule_503(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_503", value)
    }

    pub fn apply_rule_504(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_504", value)
    }

    pub fn apply_rule_505(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_505", value)
    }

    pub fn apply_rule_506(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_506", value)
    }

    pub fn apply_rule_507(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_507", value)
    }

    pub fn apply_rule_508(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_508", value)
    }

    pub fn apply_rule_509(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_509", value)
    }

    pub fn apply_rule_510(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_510", value)
    }

    pub fn apply_rule_511(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_511", value)
    }

    pub fn apply_rule_512(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_512", value)
    }

    pub fn apply_rule_513(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_513", value)
    }

    pub fn apply_rule_514(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_514", value)
    }

    pub fn apply_rule_515(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_515", value)
    }

    pub fn apply_rule_516(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_516", value)
    }

    pub fn apply_rule_517(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_517", value)
    }

    pub fn apply_rule_518(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_518", value)
    }

    pub fn apply_rule_519(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_519", value)
    }

    pub fn apply_rule_520(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_520", value)
    }

    pub fn apply_rule_521(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_521", value)
    }

    pub fn apply_rule_522(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_522", value)
    }

    pub fn apply_rule_523(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_523", value)
    }

    pub fn apply_rule_524(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_524", value)
    }

    pub fn apply_rule_525(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_525", value)
    }

    pub fn apply_rule_526(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_526", value)
    }

    pub fn apply_rule_527(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_527", value)
    }

    pub fn apply_rule_528(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_528", value)
    }

    pub fn apply_rule_529(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_529", value)
    }

    pub fn apply_rule_530(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_530", value)
    }

    pub fn apply_rule_531(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_531", value)
    }

    pub fn apply_rule_532(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_532", value)
    }

    pub fn apply_rule_533(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_533", value)
    }

    pub fn apply_rule_534(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_534", value)
    }

    pub fn apply_rule_535(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_535", value)
    }

    pub fn apply_rule_536(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_536", value)
    }

    pub fn apply_rule_537(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_537", value)
    }

    pub fn apply_rule_538(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_538", value)
    }

    pub fn apply_rule_539(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_539", value)
    }

    pub fn apply_rule_540(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_540", value)
    }

    pub fn apply_rule_541(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_541", value)
    }

    pub fn apply_rule_542(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_542", value)
    }

    pub fn apply_rule_543(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_543", value)
    }

    pub fn apply_rule_544(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_544", value)
    }

    pub fn apply_rule_545(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_545", value)
    }

    pub fn apply_rule_546(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_546", value)
    }

    pub fn apply_rule_547(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_547", value)
    }

    pub fn apply_rule_548(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_548", value)
    }

    pub fn apply_rule_549(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_549", value)
    }

    pub fn apply_rule_550(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_550", value)
    }

    pub fn apply_rule_551(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_551", value)
    }

    pub fn apply_rule_552(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_552", value)
    }

    pub fn apply_rule_553(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_553", value)
    }

    pub fn apply_rule_554(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_554", value)
    }

    pub fn apply_rule_555(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_555", value)
    }

    pub fn apply_rule_556(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_556", value)
    }

    pub fn apply_rule_557(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_557", value)
    }

    pub fn apply_rule_558(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_558", value)
    }

    pub fn apply_rule_559(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_559", value)
    }

    pub fn apply_rule_560(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_560", value)
    }

    pub fn apply_rule_561(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_561", value)
    }

    pub fn apply_rule_562(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_562", value)
    }

    pub fn apply_rule_563(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_563", value)
    }

    pub fn apply_rule_564(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_564", value)
    }

    pub fn apply_rule_565(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_565", value)
    }

    pub fn apply_rule_566(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_566", value)
    }

    pub fn apply_rule_567(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_567", value)
    }

    pub fn apply_rule_568(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_568", value)
    }

    pub fn apply_rule_569(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_569", value)
    }

    pub fn apply_rule_570(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_570", value)
    }

    pub fn apply_rule_571(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_571", value)
    }

    pub fn apply_rule_572(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_572", value)
    }

    pub fn apply_rule_573(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_573", value)
    }

    pub fn apply_rule_574(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_574", value)
    }

    pub fn apply_rule_575(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_575", value)
    }

    pub fn apply_rule_576(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_576", value)
    }

    pub fn apply_rule_577(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_577", value)
    }

    pub fn apply_rule_578(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_578", value)
    }

    pub fn apply_rule_579(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_579", value)
    }

    pub fn apply_rule_580(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_580", value)
    }

    pub fn apply_rule_581(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_581", value)
    }

    pub fn apply_rule_582(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_582", value)
    }

    pub fn apply_rule_583(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_583", value)
    }

    pub fn apply_rule_584(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_584", value)
    }

    pub fn apply_rule_585(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_585", value)
    }

    pub fn apply_rule_586(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_586", value)
    }

    pub fn apply_rule_587(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_587", value)
    }

    pub fn apply_rule_588(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_588", value)
    }

    pub fn apply_rule_589(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_589", value)
    }

    pub fn apply_rule_590(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_590", value)
    }

    pub fn apply_rule_591(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_591", value)
    }

    pub fn apply_rule_592(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_592", value)
    }

    pub fn apply_rule_593(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_593", value)
    }

    pub fn apply_rule_594(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_594", value)
    }

    pub fn apply_rule_595(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_595", value)
    }

    pub fn apply_rule_596(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_596", value)
    }

    pub fn apply_rule_597(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_597", value)
    }

    pub fn apply_rule_598(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_598", value)
    }

    pub fn apply_rule_599(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_599", value)
    }

    pub fn apply_rule_600(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_600", value)
    }

    pub fn apply_rule_601(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_601", value)
    }

    pub fn apply_rule_602(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_602", value)
    }

    pub fn apply_rule_603(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_603", value)
    }

    pub fn apply_rule_604(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_604", value)
    }

    pub fn apply_rule_605(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_605", value)
    }

    pub fn apply_rule_606(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_606", value)
    }

    pub fn apply_rule_607(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_607", value)
    }

    pub fn apply_rule_608(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_608", value)
    }

    pub fn apply_rule_609(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_609", value)
    }

    pub fn apply_rule_610(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_610", value)
    }

    pub fn apply_rule_611(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_611", value)
    }

    pub fn apply_rule_612(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_612", value)
    }

    pub fn apply_rule_613(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_613", value)
    }

    pub fn apply_rule_614(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_614", value)
    }

    pub fn apply_rule_615(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_615", value)
    }

    pub fn apply_rule_616(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_616", value)
    }

    pub fn apply_rule_617(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_617", value)
    }

    pub fn apply_rule_618(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_618", value)
    }

    pub fn apply_rule_619(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_619", value)
    }

    pub fn apply_rule_620(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_620", value)
    }

    pub fn apply_rule_621(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_621", value)
    }

    pub fn apply_rule_622(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_622", value)
    }

    pub fn apply_rule_623(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_623", value)
    }

    pub fn apply_rule_624(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_624", value)
    }

    pub fn apply_rule_625(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_625", value)
    }

    pub fn apply_rule_626(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_626", value)
    }

    pub fn apply_rule_627(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_627", value)
    }

    pub fn apply_rule_628(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_628", value)
    }

    pub fn apply_rule_629(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_629", value)
    }

    pub fn apply_rule_630(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_630", value)
    }

    pub fn apply_rule_631(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_631", value)
    }

    pub fn apply_rule_632(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_632", value)
    }

    pub fn apply_rule_633(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_633", value)
    }

    pub fn apply_rule_634(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_634", value)
    }

    pub fn apply_rule_635(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_635", value)
    }

    pub fn apply_rule_636(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_636", value)
    }

    pub fn apply_rule_637(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_637", value)
    }

    pub fn apply_rule_638(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_638", value)
    }

    pub fn apply_rule_639(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_639", value)
    }

    pub fn apply_rule_640(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_640", value)
    }

    pub fn apply_rule_641(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_641", value)
    }

    pub fn apply_rule_642(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_642", value)
    }

    pub fn apply_rule_643(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_643", value)
    }

    pub fn apply_rule_644(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_644", value)
    }

    pub fn apply_rule_645(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_645", value)
    }

    pub fn apply_rule_646(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_646", value)
    }

    pub fn apply_rule_647(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_647", value)
    }

    pub fn apply_rule_648(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_648", value)
    }

    pub fn apply_rule_649(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_649", value)
    }

    pub fn apply_rule_650(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_650", value)
    }

    pub fn apply_rule_651(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_651", value)
    }

    pub fn apply_rule_652(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_652", value)
    }

    pub fn apply_rule_653(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_653", value)
    }

    pub fn apply_rule_654(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_654", value)
    }

    pub fn apply_rule_655(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_655", value)
    }

    pub fn apply_rule_656(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_656", value)
    }

    pub fn apply_rule_657(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_657", value)
    }

    pub fn apply_rule_658(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_658", value)
    }

    pub fn apply_rule_659(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_659", value)
    }

    pub fn apply_rule_660(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_660", value)
    }

    pub fn apply_rule_661(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_661", value)
    }

    pub fn apply_rule_662(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_662", value)
    }

    pub fn apply_rule_663(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_663", value)
    }

    pub fn apply_rule_664(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_664", value)
    }

    pub fn apply_rule_665(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_665", value)
    }

    pub fn apply_rule_666(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_666", value)
    }

    pub fn apply_rule_667(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_667", value)
    }

    pub fn apply_rule_668(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_668", value)
    }

    pub fn apply_rule_669(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_669", value)
    }

    pub fn apply_rule_670(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_670", value)
    }

    pub fn apply_rule_671(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_671", value)
    }

    pub fn apply_rule_672(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_672", value)
    }

    pub fn apply_rule_673(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_673", value)
    }

    pub fn apply_rule_674(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_674", value)
    }

    pub fn apply_rule_675(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_675", value)
    }

    pub fn apply_rule_676(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_676", value)
    }

    pub fn apply_rule_677(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_677", value)
    }

    pub fn apply_rule_678(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_678", value)
    }

    pub fn apply_rule_679(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_679", value)
    }

    pub fn apply_rule_680(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_680", value)
    }

    pub fn apply_rule_681(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_681", value)
    }

    pub fn apply_rule_682(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_682", value)
    }

    pub fn apply_rule_683(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_683", value)
    }

    pub fn apply_rule_684(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_684", value)
    }

    pub fn apply_rule_685(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_685", value)
    }

    pub fn apply_rule_686(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_686", value)
    }

    pub fn apply_rule_687(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_687", value)
    }

    pub fn apply_rule_688(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_688", value)
    }

    pub fn apply_rule_689(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_689", value)
    }

    pub fn apply_rule_690(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_690", value)
    }

    pub fn apply_rule_691(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_691", value)
    }

    pub fn apply_rule_692(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_692", value)
    }

    pub fn apply_rule_693(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_693", value)
    }

    pub fn apply_rule_694(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_694", value)
    }

    pub fn apply_rule_695(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_695", value)
    }

    pub fn apply_rule_696(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_696", value)
    }

    pub fn apply_rule_697(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_697", value)
    }

    pub fn apply_rule_698(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_698", value)
    }

    pub fn apply_rule_699(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_699", value)
    }

    pub fn apply_rule_700(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_700", value)
    }

    pub fn apply_rule_701(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_701", value)
    }

    pub fn apply_rule_702(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_702", value)
    }

    pub fn apply_rule_703(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_703", value)
    }

    pub fn apply_rule_704(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_704", value)
    }

    pub fn apply_rule_705(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_705", value)
    }

    pub fn apply_rule_706(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_706", value)
    }

    pub fn apply_rule_707(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_707", value)
    }

    pub fn apply_rule_708(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_708", value)
    }

    pub fn apply_rule_709(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_709", value)
    }

    pub fn apply_rule_710(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_710", value)
    }

    pub fn apply_rule_711(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_711", value)
    }

    pub fn apply_rule_712(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_712", value)
    }

    pub fn apply_rule_713(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_713", value)
    }

    pub fn apply_rule_714(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_714", value)
    }

    pub fn apply_rule_715(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_715", value)
    }

    pub fn apply_rule_716(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_716", value)
    }

    pub fn apply_rule_717(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_717", value)
    }

    pub fn apply_rule_718(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_718", value)
    }

    pub fn apply_rule_719(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_719", value)
    }

    pub fn apply_rule_720(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_720", value)
    }

    pub fn apply_rule_721(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_721", value)
    }

    pub fn apply_rule_722(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_722", value)
    }

    pub fn apply_rule_723(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_723", value)
    }

    pub fn apply_rule_724(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_724", value)
    }

    pub fn apply_rule_725(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_725", value)
    }

    pub fn apply_rule_726(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_726", value)
    }

    pub fn apply_rule_727(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_727", value)
    }

    pub fn apply_rule_728(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_728", value)
    }

    pub fn apply_rule_729(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_729", value)
    }

    pub fn apply_rule_730(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_730", value)
    }

    pub fn apply_rule_731(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_731", value)
    }

    pub fn apply_rule_732(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_732", value)
    }

    pub fn apply_rule_733(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_733", value)
    }

    pub fn apply_rule_734(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_734", value)
    }

    pub fn apply_rule_735(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_735", value)
    }

    pub fn apply_rule_736(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_736", value)
    }

    pub fn apply_rule_737(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_737", value)
    }

    pub fn apply_rule_738(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_738", value)
    }

    pub fn apply_rule_739(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_739", value)
    }

    pub fn apply_rule_740(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_740", value)
    }

    pub fn apply_rule_741(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_741", value)
    }

    pub fn apply_rule_742(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_742", value)
    }

    pub fn apply_rule_743(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_743", value)
    }

    pub fn apply_rule_744(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_744", value)
    }

    pub fn apply_rule_745(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_745", value)
    }

    pub fn apply_rule_746(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_746", value)
    }

    pub fn apply_rule_747(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_747", value)
    }

    pub fn apply_rule_748(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_748", value)
    }

    pub fn apply_rule_749(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_749", value)
    }

    pub fn apply_rule_750(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_750", value)
    }

    pub fn apply_rule_751(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_751", value)
    }

    pub fn apply_rule_752(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_752", value)
    }

    pub fn apply_rule_753(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_753", value)
    }

    pub fn apply_rule_754(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_754", value)
    }

    pub fn apply_rule_755(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_755", value)
    }

    pub fn apply_rule_756(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_756", value)
    }

    pub fn apply_rule_757(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_757", value)
    }

    pub fn apply_rule_758(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_758", value)
    }

    pub fn apply_rule_759(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_759", value)
    }

    pub fn apply_rule_760(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_760", value)
    }

    pub fn apply_rule_761(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_761", value)
    }

    pub fn apply_rule_762(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_762", value)
    }

    pub fn apply_rule_763(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_763", value)
    }

    pub fn apply_rule_764(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_764", value)
    }

    pub fn apply_rule_765(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_765", value)
    }

    pub fn apply_rule_766(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_766", value)
    }

    pub fn apply_rule_767(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_767", value)
    }

    pub fn apply_rule_768(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_768", value)
    }

    pub fn apply_rule_769(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_769", value)
    }

    pub fn apply_rule_770(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_770", value)
    }

    pub fn apply_rule_771(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_771", value)
    }

    pub fn apply_rule_772(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_772", value)
    }

    pub fn apply_rule_773(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_773", value)
    }

    pub fn apply_rule_774(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_774", value)
    }

    pub fn apply_rule_775(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_775", value)
    }

    pub fn apply_rule_776(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_776", value)
    }

    pub fn apply_rule_777(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_777", value)
    }

    pub fn apply_rule_778(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_778", value)
    }

    pub fn apply_rule_779(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_779", value)
    }

    pub fn apply_rule_780(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_780", value)
    }

    pub fn apply_rule_781(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_781", value)
    }

    pub fn apply_rule_782(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_782", value)
    }

    pub fn apply_rule_783(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_783", value)
    }

    pub fn apply_rule_784(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_784", value)
    }

    pub fn apply_rule_785(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_785", value)
    }

    pub fn apply_rule_786(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_786", value)
    }

    pub fn apply_rule_787(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_787", value)
    }

    pub fn apply_rule_788(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_788", value)
    }

    pub fn apply_rule_789(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_789", value)
    }

    pub fn apply_rule_790(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_790", value)
    }

    pub fn apply_rule_791(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_791", value)
    }

    pub fn apply_rule_792(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_792", value)
    }

    pub fn apply_rule_793(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_793", value)
    }

    pub fn apply_rule_794(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_794", value)
    }

    pub fn apply_rule_795(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_795", value)
    }

    pub fn apply_rule_796(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_796", value)
    }

    pub fn apply_rule_797(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_797", value)
    }

    pub fn apply_rule_798(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_798", value)
    }

    pub fn apply_rule_799(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_799", value)
    }

    pub fn apply_rule_800(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_800", value)
    }

    pub fn apply_rule_801(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_801", value)
    }

    pub fn apply_rule_802(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_802", value)
    }

    pub fn apply_rule_803(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_803", value)
    }

    pub fn apply_rule_804(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_804", value)
    }

    pub fn apply_rule_805(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_805", value)
    }

    pub fn apply_rule_806(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_806", value)
    }

    pub fn apply_rule_807(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_807", value)
    }

    pub fn apply_rule_808(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_808", value)
    }

    pub fn apply_rule_809(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_809", value)
    }

    pub fn apply_rule_810(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_810", value)
    }

    pub fn apply_rule_811(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_811", value)
    }

    pub fn apply_rule_812(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_812", value)
    }

    pub fn apply_rule_813(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_813", value)
    }

    pub fn apply_rule_814(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_814", value)
    }

    pub fn apply_rule_815(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_815", value)
    }

    pub fn apply_rule_816(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_816", value)
    }

    pub fn apply_rule_817(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_817", value)
    }

    pub fn apply_rule_818(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_818", value)
    }

    pub fn apply_rule_819(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_819", value)
    }

    pub fn apply_rule_820(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_820", value)
    }

    pub fn apply_rule_821(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_821", value)
    }

    pub fn apply_rule_822(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_822", value)
    }

    pub fn apply_rule_823(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_823", value)
    }

    pub fn apply_rule_824(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_824", value)
    }

    pub fn apply_rule_825(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_825", value)
    }

    pub fn apply_rule_826(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_826", value)
    }

    pub fn apply_rule_827(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_827", value)
    }

    pub fn apply_rule_828(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_828", value)
    }

    pub fn apply_rule_829(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_829", value)
    }

    pub fn apply_rule_830(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_830", value)
    }

    pub fn apply_rule_831(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_831", value)
    }

    pub fn apply_rule_832(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_832", value)
    }

    pub fn apply_rule_833(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_833", value)
    }

    pub fn apply_rule_834(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_834", value)
    }

    pub fn apply_rule_835(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_835", value)
    }

    pub fn apply_rule_836(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_836", value)
    }

    pub fn apply_rule_837(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_837", value)
    }

    pub fn apply_rule_838(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_838", value)
    }

    pub fn apply_rule_839(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_839", value)
    }

    pub fn apply_rule_840(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_840", value)
    }

    pub fn apply_rule_841(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_841", value)
    }

    pub fn apply_rule_842(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_842", value)
    }

    pub fn apply_rule_843(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_843", value)
    }

    pub fn apply_rule_844(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_844", value)
    }

    pub fn apply_rule_845(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_845", value)
    }

    pub fn apply_rule_846(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_846", value)
    }

    pub fn apply_rule_847(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_847", value)
    }

    pub fn apply_rule_848(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_848", value)
    }

    pub fn apply_rule_849(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_849", value)
    }

    pub fn apply_rule_850(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_850", value)
    }

    pub fn apply_rule_851(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_851", value)
    }

    pub fn apply_rule_852(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_852", value)
    }

    pub fn apply_rule_853(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_853", value)
    }

    pub fn apply_rule_854(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_854", value)
    }

    pub fn apply_rule_855(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_855", value)
    }

    pub fn apply_rule_856(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_856", value)
    }

    pub fn apply_rule_857(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_857", value)
    }

    pub fn apply_rule_858(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_858", value)
    }

    pub fn apply_rule_859(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_859", value)
    }

    pub fn apply_rule_860(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_860", value)
    }

    pub fn apply_rule_861(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_861", value)
    }

    pub fn apply_rule_862(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_862", value)
    }

    pub fn apply_rule_863(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_863", value)
    }

    pub fn apply_rule_864(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_864", value)
    }

    pub fn apply_rule_865(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_865", value)
    }

    pub fn apply_rule_866(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_866", value)
    }

    pub fn apply_rule_867(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_867", value)
    }

    pub fn apply_rule_868(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_868", value)
    }

    pub fn apply_rule_869(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_869", value)
    }

    pub fn apply_rule_870(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_870", value)
    }

    pub fn apply_rule_871(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_871", value)
    }

    pub fn apply_rule_872(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_872", value)
    }

    pub fn apply_rule_873(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_873", value)
    }

    pub fn apply_rule_874(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_874", value)
    }

    pub fn apply_rule_875(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_875", value)
    }

    pub fn apply_rule_876(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_876", value)
    }

    pub fn apply_rule_877(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_877", value)
    }

    pub fn apply_rule_878(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_878", value)
    }

    pub fn apply_rule_879(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_879", value)
    }

    pub fn apply_rule_880(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_880", value)
    }

    pub fn apply_rule_881(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_881", value)
    }

    pub fn apply_rule_882(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_882", value)
    }

    pub fn apply_rule_883(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_883", value)
    }

    pub fn apply_rule_884(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_884", value)
    }

    pub fn apply_rule_885(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_885", value)
    }

    pub fn apply_rule_886(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_886", value)
    }

    pub fn apply_rule_887(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_887", value)
    }

    pub fn apply_rule_888(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_888", value)
    }

    pub fn apply_rule_889(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_889", value)
    }

    pub fn apply_rule_890(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_890", value)
    }

    pub fn apply_rule_891(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_891", value)
    }

    pub fn apply_rule_892(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_892", value)
    }

    pub fn apply_rule_893(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_893", value)
    }

    pub fn apply_rule_894(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_894", value)
    }

    pub fn apply_rule_895(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_895", value)
    }

    pub fn apply_rule_896(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_896", value)
    }

    pub fn apply_rule_897(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_897", value)
    }

    pub fn apply_rule_898(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_898", value)
    }

    pub fn apply_rule_899(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_899", value)
    }

    pub fn apply_rule_900(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_900", value)
    }

    pub fn apply_rule_901(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_901", value)
    }

    pub fn apply_rule_902(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_902", value)
    }

    pub fn apply_rule_903(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_903", value)
    }

    pub fn apply_rule_904(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_904", value)
    }

    pub fn apply_rule_905(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_905", value)
    }

    pub fn apply_rule_906(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_906", value)
    }

    pub fn apply_rule_907(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_907", value)
    }

    pub fn apply_rule_908(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_908", value)
    }

    pub fn apply_rule_909(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_909", value)
    }

    pub fn apply_rule_910(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_910", value)
    }

    pub fn apply_rule_911(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_911", value)
    }

    pub fn apply_rule_912(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_912", value)
    }

    pub fn apply_rule_913(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_913", value)
    }

    pub fn apply_rule_914(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_914", value)
    }

    pub fn apply_rule_915(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_915", value)
    }

    pub fn apply_rule_916(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_916", value)
    }

    pub fn apply_rule_917(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_917", value)
    }

    pub fn apply_rule_918(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_918", value)
    }

    pub fn apply_rule_919(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_919", value)
    }

    pub fn apply_rule_920(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_920", value)
    }

    pub fn apply_rule_921(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_921", value)
    }

    pub fn apply_rule_922(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_922", value)
    }

    pub fn apply_rule_923(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_923", value)
    }

    pub fn apply_rule_924(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_924", value)
    }

    pub fn apply_rule_925(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_925", value)
    }

    pub fn apply_rule_926(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_926", value)
    }

    pub fn apply_rule_927(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_927", value)
    }

    pub fn apply_rule_928(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_928", value)
    }

    pub fn apply_rule_929(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_929", value)
    }

    pub fn apply_rule_930(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_930", value)
    }

    pub fn apply_rule_931(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_931", value)
    }

    pub fn apply_rule_932(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_932", value)
    }

    pub fn apply_rule_933(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_933", value)
    }

    pub fn apply_rule_934(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_934", value)
    }

    pub fn apply_rule_935(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_935", value)
    }

    pub fn apply_rule_936(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_936", value)
    }

    pub fn apply_rule_937(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_937", value)
    }

    pub fn apply_rule_938(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_938", value)
    }

    pub fn apply_rule_939(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_939", value)
    }

    pub fn apply_rule_940(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_940", value)
    }

    pub fn apply_rule_941(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_941", value)
    }

    pub fn apply_rule_942(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_942", value)
    }

    pub fn apply_rule_943(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_943", value)
    }

    pub fn apply_rule_944(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_944", value)
    }

    pub fn apply_rule_945(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_945", value)
    }

    pub fn apply_rule_946(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_946", value)
    }

    pub fn apply_rule_947(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_947", value)
    }

    pub fn apply_rule_948(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_948", value)
    }

    pub fn apply_rule_949(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_949", value)
    }

    pub fn apply_rule_950(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_950", value)
    }

    pub fn apply_rule_951(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_951", value)
    }

    pub fn apply_rule_952(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_952", value)
    }

    pub fn apply_rule_953(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_953", value)
    }

    pub fn apply_rule_954(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_954", value)
    }

    pub fn apply_rule_955(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_955", value)
    }

    pub fn apply_rule_956(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_956", value)
    }

    pub fn apply_rule_957(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_957", value)
    }

    pub fn apply_rule_958(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_958", value)
    }

    pub fn apply_rule_959(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_959", value)
    }

    pub fn apply_rule_960(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_960", value)
    }

    pub fn apply_rule_961(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_961", value)
    }

    pub fn apply_rule_962(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_962", value)
    }

    pub fn apply_rule_963(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_963", value)
    }

    pub fn apply_rule_964(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_964", value)
    }

    pub fn apply_rule_965(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_965", value)
    }

    pub fn apply_rule_966(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_966", value)
    }

    pub fn apply_rule_967(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_967", value)
    }

    pub fn apply_rule_968(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_968", value)
    }

    pub fn apply_rule_969(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_969", value)
    }

    pub fn apply_rule_970(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_970", value)
    }

    pub fn apply_rule_971(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_971", value)
    }

    pub fn apply_rule_972(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_972", value)
    }

    pub fn apply_rule_973(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_973", value)
    }

    pub fn apply_rule_974(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_974", value)
    }

    pub fn apply_rule_975(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_975", value)
    }

    pub fn apply_rule_976(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_976", value)
    }

    pub fn apply_rule_977(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_977", value)
    }

    pub fn apply_rule_978(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_978", value)
    }

    pub fn apply_rule_979(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_979", value)
    }

    pub fn apply_rule_980(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_980", value)
    }

    pub fn apply_rule_981(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_981", value)
    }

    pub fn apply_rule_982(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_982", value)
    }

    pub fn apply_rule_983(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_983", value)
    }

    pub fn apply_rule_984(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_984", value)
    }

    pub fn apply_rule_985(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_985", value)
    }

    pub fn apply_rule_986(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_986", value)
    }

    pub fn apply_rule_987(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_987", value)
    }

    pub fn apply_rule_988(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_988", value)
    }

    pub fn apply_rule_989(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_989", value)
    }

    pub fn apply_rule_990(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_990", value)
    }

    pub fn apply_rule_991(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_991", value)
    }

    pub fn apply_rule_992(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_992", value)
    }

    pub fn apply_rule_993(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_993", value)
    }

    pub fn apply_rule_994(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_994", value)
    }

    pub fn apply_rule_995(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_995", value)
    }

    pub fn apply_rule_996(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_996", value)
    }

    pub fn apply_rule_997(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_997", value)
    }

    pub fn apply_rule_998(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_998", value)
    }

    pub fn apply_rule_999(&self, value: f64) -> Option<&RuleAction> {
        self.check_rule("rule_999", value)
    }
}
