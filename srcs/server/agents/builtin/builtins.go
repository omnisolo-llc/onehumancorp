package builtin

var ExploreAgent = BuiltInAgentDefinition{
	AgentType: "Explore",
	WhenToUse: "Fast agent specialized for exploring codebases. Use this when you need to quickly find files by patterns or answer questions about the codebase.",
	Tools:     []string{"Bash", "Read", "Glob", "Grep", "WebFetch", "WebSearch"},
	DisallowedTools: []string{"Agent", "ExitPlanMode", "Edit", "Write", "NotebookEdit"},
	Source:    "built-in",
	BaseDir:   "built-in",
	Model:     "claude-3-5-haiku-20241022",
	OmitClaudeMd: true,
	GetSystemPrompt: func() string {
		return "You are a file search specialist. === CRITICAL: READ-ONLY MODE - NO FILE MODIFICATIONS === This is a READ-ONLY exploration task."
	},
}

var PlanAgent = BuiltInAgentDefinition{
	AgentType: "Plan",
	WhenToUse: "Software architect agent for designing implementation plans. Use this when you need to plan the implementation strategy for a task.",
	Tools:     []string{"Bash", "Read", "Glob", "Grep", "WebFetch", "WebSearch"},
	DisallowedTools: []string{"Agent", "ExitPlanMode", "Edit", "Write", "NotebookEdit"},
	Source:    "built-in",
	BaseDir:   "built-in",
	Model:     "claude-3-7-sonnet-20250219",
	OmitClaudeMd: true,
	GetSystemPrompt: func() string {
		return "You are a software architect and planning specialist. === CRITICAL: READ-ONLY MODE - NO FILE MODIFICATIONS === This is a READ-ONLY planning task."
	},
}
