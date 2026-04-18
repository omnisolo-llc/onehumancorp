package dashboard

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"sort"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/settings"
)

// wizardStatusResponse describes the current setup state of the platform.
type wizardStatusResponse struct {
	// Configured is true when all required fields have been set.
	Configured bool `json:"configured"`
	// Steps holds per-step completion status.
	Steps wizardSteps `json:"steps"`
}

type wizardSteps struct {
	Server     bool `json:"server"`      // listen_addr and db_path set
	AiProvider bool `json:"ai_provider"` // at least one AI provider enabled
	Centrifuge bool `json:"centrifuge"`  // centrifuge_url set
}

// wizardConfigureRequest carries a partial or complete settings update from
// the wizard UI.
type wizardConfigureRequest struct {
	ListenAddr    string                `json:"listen_addr,omitempty"`
	DBPath        string                `json:"db_path,omitempty"`
	PostgresURL   string                `json:"postgres_url,omitempty"`
	RedisURL      string                `json:"redis_url,omitempty"`
	CentrifugeURL string                `json:"centrifuge_url,omitempty"`
	MinimaxAPIKey string                `json:"minimax_api_key,omitempty"`
	Extras        map[string]string `json:"extras,omitempty"`
	AiProviders   []settings.AiProvider `json:"ai_providers,omitempty"`
}

type wizardBootstrapBusinessRequest struct {
	Prompt     string   `json:"prompt,omitempty"`
	CompanyName string   `json:"company_name,omitempty"`
	Industry   string   `json:"industry,omitempty"`
	CompanySize string   `json:"company_size,omitempty"`
	Goals      []string `json:"goals,omitempty"`
	Deployment string   `json:"deployment_preference,omitempty"`
	AdminName  string   `json:"admin_name,omitempty"`
	AdminEmail string   `json:"admin_email,omitempty"`
}

type businessArchitecturePlan struct {
	TemplateDomain string             `json:"template_domain"`
	CompanyType    string             `json:"company_type"`
	Rationale      string             `json:"rationale"`
	Teams          []businessTeamPlan `json:"teams"`
	Hires          []businessHirePlan `json:"hires"`
}

type businessTeamPlan struct {
	Name    string   `json:"name"`
	Purpose string   `json:"purpose"`
	Roles   []string `json:"roles"`
}

type businessHirePlan struct {
	Name   string `json:"name"`
	Role   string `json:"role"`
	Reason string `json:"reason"`
}

type formationDocument struct {
	Name        string `json:"name"`
	Status      string `json:"status"`
	Description string `json:"description"`
}

type wizardBootstrapBusinessResponse struct {
	Status             string                `json:"status"`
	Summary            string                `json:"summary"`
	MCPServers         []string              `json:"mcp_servers"`
	Architecture       businessArchitecturePlan `json:"architecture"`
	HiredAgents        []orchestration.Agent `json:"hired_agents"`
	FormationDocuments []formationDocument   `json:"formation_documents"`
}

var businessAutomationMCPServers = []string{
	"company-architect-mcp",
	"workforce-hiring-mcp",
	"formation-docs-mcp",
}

// handleWizardStatus returns a JSON summary of whether each wizard step has
// been completed so the Flutter wizard UI can determine which steps to show.
func (s *Server) handleWizardStatus(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	s.mu.RLock()
	cfg := s.settings
	s.mu.RUnlock()

	steps := wizardSteps{
		Server:     cfg.ListenAddr != "" && cfg.DBPath != "",
		AiProvider: hasEnabledProvider(cfg.AiProviders),
		Centrifuge: cfg.CentrifugeURL != "",
	}
	resp := wizardStatusResponse{
		Configured: steps.Server && steps.AiProvider && steps.Centrifuge,
		Steps:      steps,
	}
	writeJSON(w, resp)
}

// handleWizardConfigure applies a partial settings update from the wizard and
// persists it via the settings store.
func (s *Server) handleWizardConfigure(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req wizardConfigureRequest
	dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
	dec.DisallowUnknownFields()
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload: "+err.Error(), http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	cfg := s.settings
	if req.ListenAddr != "" {
		cfg.ListenAddr = req.ListenAddr
	}
	if req.DBPath != "" {
		cfg.DBPath = req.DBPath
	}
	if req.PostgresURL != "" {
		cfg.PostgresURL = req.PostgresURL
	}
	if req.RedisURL != "" {
		cfg.RedisURL = req.RedisURL
	}
	if req.CentrifugeURL != "" {
		cfg.CentrifugeURL = req.CentrifugeURL
	}
	if req.MinimaxAPIKey != "" {
		cfg.MinimaxAPIKey = req.MinimaxAPIKey
		s.hub.SetMinimaxAPIKey(req.MinimaxAPIKey)
	}
	if len(req.Extras) > 0 {
		for k, v := range req.Extras {
			if cfg.Extras == nil {
				cfg.Extras = make(map[string]string)
			}
			cfg.Extras[k] = v
		}
	}
	if len(req.AiProviders) > 0 {
		cfg.AiProviders = req.AiProviders
	}
	s.settings = cfg
	s.mu.Unlock()

	_ = s.hub.SettingsStore().Update(cfg)

	steps := wizardSteps{
		Server:     cfg.ListenAddr != "" && cfg.DBPath != "",
		AiProvider: hasEnabledProvider(cfg.AiProviders),
		Centrifuge: cfg.CentrifugeURL != "",
	}
	writeJSON(w, wizardStatusResponse{
		Configured: steps.Server && steps.AiProvider && steps.Centrifuge,
		Steps:      steps,
	})
}

// hasEnabledProvider returns true if at least one AiProvider is enabled.
func hasEnabledProvider(providers []settings.AiProvider) bool {
	for _, p := range providers {
		if p.Enabled {
			return true
		}
	}
	return false
}

func (s *Server) handleWizardBootstrapBusiness(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req wizardBootstrapBusinessRequest
	dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
	dec.DisallowUnknownFields()
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload: "+err.Error(), http.StatusBadRequest)
		return
	}
	if strings.TrimSpace(req.Prompt) == "" && strings.TrimSpace(req.CompanyName) == "" {
		http.Error(w, "prompt or company_name is required", http.StatusBadRequest)
		return
	}

	s.mu.RLock()
	cfg := s.settings
	s.mu.RUnlock()
	if !hasEnabledProvider(cfg.AiProviders) {
		http.Error(w, "configure at least one enabled AI provider first", http.StatusPreconditionFailed)
		return
	}

	plan := s.planBusinessArchitecture(req)
	hiredAgents := s.bootstrapBusinessAgents(plan, req)
	documents := buildFormationDocuments(req, plan)
	s.persistBusinessBootstrapMetadata(req, plan)

	summary := fmt.Sprintf(
		"Planned a %s using %d backend MCP servers, hired %d agents, and drafted %d formation documents.",
		plan.CompanyType,
		len(businessAutomationMCPServers),
		len(hiredAgents),
		len(documents),
	)
	writeJSON(w, wizardBootstrapBusinessResponse{
		Status:             "created",
		Summary:            summary,
		MCPServers:         append([]string(nil), businessAutomationMCPServers...),
		Architecture:       plan,
		HiredAgents:        hiredAgents,
		FormationDocuments: documents,
	})
}

func (s *Server) planBusinessArchitecture(req wizardBootstrapBusinessRequest) businessArchitecturePlan {
	brief := businessBrief(req)
	templateDomain := resolveBusinessTemplate(brief)
	companyType := companyTypeLabel(templateDomain)
	rationale := businessTemplateRationale(templateDomain, brief)

	switch templateDomain {
	case "digital_marketing_agency":
		return businessArchitecturePlan{
			TemplateDomain: templateDomain,
			CompanyType:    companyType,
			Rationale:      rationale,
			Teams: []businessTeamPlan{
				{Name: "Client Delivery", Purpose: "Own staging engagements, client communication, and service quality.", Roles: []string{"MARKETING_MANAGER", "DESIGNER"}},
				{Name: "Demand Generation", Purpose: "Create outbound growth loops, partnerships, and local lead generation.", Roles: []string{"GROWTH_AGENT", "CONTENT_STRATEGIST", "SEO_SPECIALIST"}},
				{Name: "Revenue Intelligence", Purpose: "Track campaign, referral, and utilization metrics for rapid iteration.", Roles: []string{"ANALYTICS_ENGINEER"}},
			},
			Hires: []businessHirePlan{
				{Name: "Studio Operations Lead", Role: string(domain.RoleMarketingManager), Reason: "Owns client delivery, quoting, and scheduling for staging jobs."},
				{Name: "Lead Staging Designer", Role: string(domain.RoleDesigner), Reason: "Translates vacant spaces into sell-ready design concepts and install plans."},
				{Name: "Growth Lead", Role: string(domain.RoleGrowthAgent), Reason: "Builds realtor partnerships and keeps the lead pipeline full."},
				{Name: "Content Strategist", Role: string(domain.RoleContentStrategist), Reason: "Produces before/after campaigns, listing collateral, and case studies."},
				{Name: "Analytics Engineer", Role: string(domain.RoleAnalyticsEngineer), Reason: "Measures channel ROI, close rates, and referral performance."},
			},
		}
	case "accounting_firm":
		return businessArchitecturePlan{
			TemplateDomain: templateDomain,
			CompanyType:    companyType,
			Rationale:      rationale,
			Teams: []businessTeamPlan{
				{Name: "Finance Operations", Purpose: "Keep the books, payroll, and client cash visibility accurate from day one.", Roles: []string{"CFO", "BOOKKEEPER", "PAYROLL_MANAGER"}},
				{Name: "Compliance", Purpose: "Prepare filings and maintain regulatory correctness.", Roles: []string{"TAX_SPECIALIST", "AUDIT_MANAGER"}},
			},
			Hires: []businessHirePlan{
				{Name: "Finance Lead", Role: string(domain.RoleCFO), Reason: "Sets up financial controls and board-level reporting."},
				{Name: "Bookkeeper", Role: string(domain.RoleBookkeeper), Reason: "Maintains the operating ledger and monthly close."},
				{Name: "Tax Specialist", Role: string(domain.RoleTaxSpecialist), Reason: "Prepares business tax registrations and recurring filings."},
				{Name: "Payroll Manager", Role: string(domain.RolePayrollManager), Reason: "Owns payroll and worker onboarding compliance."},
			},
		}
	default:
		return businessArchitecturePlan{
			TemplateDomain: templateDomain,
			CompanyType:    companyType,
			Rationale:      rationale,
			Teams: []businessTeamPlan{
				{Name: "Product", Purpose: "Turn the founding idea into scoped deliverables and launch priorities.", Roles: []string{"PRODUCT_MANAGER", "DESIGNER"}},
				{Name: "Engineering", Purpose: "Build the core platform, automation, and internal operations tooling.", Roles: []string{"ENGINEERING_DIRECTOR", "SOFTWARE_ENGINEER", "QA_TESTER", "SECURITY_ENGINEER"}},
				{Name: "Go-To-Market", Purpose: "Drive positioning, customer feedback, and launch execution.", Roles: []string{"MARKETING_MANAGER"}},
			},
			Hires: []businessHirePlan{
				{Name: "Product Manager", Role: string(domain.RoleProductManager), Reason: "Turns the founder brief into milestones and operating priorities."},
				{Name: "Engineering Director", Role: string(domain.RoleEngineeringDirector), Reason: "Owns system architecture and technical delivery."},
				{Name: "Software Engineer", Role: string(domain.RoleSoftwareEngineer), Reason: "Builds core workflows and internal automation."},
				{Name: "QA Tester", Role: string(domain.RoleQATester), Reason: "Protects launch quality and validates customer-facing flows."},
				{Name: "Security Engineer", Role: string(domain.RoleSecurityEngineer), Reason: "Keeps the launch secure and compliant."},
				{Name: "Marketing Manager", Role: string(domain.RoleMarketingManager), Reason: "Translates product readiness into demand generation."},
			},
		}
	}
}

func (s *Server) bootstrapBusinessAgents(plan businessArchitecturePlan, req wizardBootstrapBusinessRequest) []orchestration.Agent {
	now := time.Now().UTC()
	org := s.applyBusinessTemplate(plan.TemplateDomain, strings.TrimSpace(req.CompanyName), strings.TrimSpace(req.AdminName), now)
	existing := s.hub.Agents()
	roleCounts := make(map[string]int, len(plan.Hires))
	for _, agent := range existing {
		if agentInOrg(agent, org.ID) {
			roleCounts[agent.Role]++
		}
	}

	hiredAgents := make([]orchestration.Agent, 0, len(plan.Hires))
	for _, hire := range plan.Hires {
		if _, ok := s.roleProfileCache[hire.Role]; !ok {
			continue
		}
		roleCounts[hire.Role]++
		slug := strings.ToLower(strings.ReplaceAll(hire.Role, "_", "-"))
		agent := orchestration.Agent{
			ID:             fmt.Sprintf("%s-bootstrap-%s-%d", org.ID, slug, roleCounts[hire.Role]),
			Name:           hire.Name,
			Role:           hire.Role,
			OrganizationID: org.ID,
			Status:         orchestration.StatusIdle,
			ProviderType:   "builtin",
			Region:         strings.ToLower(strings.TrimSpace(req.Deployment)),
		}
		if agent.Region == "" {
			agent.Region = "cloud"
		}
		s.hub.RegisterAgent(agent)
		hiredAgents = append(hiredAgents, agent)
	}

	if len(hiredAgents) > 0 {
		participants := []string{org.CEOID}
		for _, agent := range hiredAgents {
			participants = append(participants, agent.ID)
		}
		meetingID := fmt.Sprintf("%s-bootstrap-%d", org.ID, now.UnixNano())
		agenda := fmt.Sprintf("Launch %s and assign the first operating responsibilities.", firstNonEmpty(strings.TrimSpace(req.CompanyName), org.Name))
		s.hub.OpenMeetingWithAgenda(meetingID, agenda, participants)
	}

	sort.Slice(hiredAgents, func(i, j int) bool {
		return hiredAgents[i].Role < hiredAgents[j].Role
	})
	return hiredAgents
}

func (s *Server) applyBusinessTemplate(templateDomain, companyName, adminName string, now time.Time) domain.Organization {
	currentOrgID := s.org.ID
	ceoName := firstNonEmpty(adminName, existingCEOName(s.org), "Human CEO")
	displayName := firstNonEmpty(companyName, s.org.Name)

	var org domain.Organization
	switch templateDomain {
	case "digital_marketing_agency":
		org = domain.NewDigitalMarketingAgency(currentOrgID, displayName, ceoName, now)
	case "accounting_firm":
		org = domain.NewAccountingFirm(currentOrgID, displayName, ceoName, now)
	default:
		org = domain.NewSoftwareCompany(currentOrgID, displayName, ceoName, now)
	}

	cache := make(map[string]domain.RoleProfile, len(org.RoleProfiles))
	for _, profile := range org.RoleProfiles {
		cache[string(profile.Role)] = profile
	}

	s.mu.Lock()
	s.org = org
	s.roleProfileCache = cache
	s.mu.Unlock()
	return org
}

func (s *Server) persistBusinessBootstrapMetadata(req wizardBootstrapBusinessRequest, plan businessArchitecturePlan) {
	s.mu.Lock()
	cfg := s.settings
	if cfg.Extras == nil {
		cfg.Extras = make(map[string]string)
	}
	if companyName := strings.TrimSpace(req.CompanyName); companyName != "" {
		cfg.Extras["company_name"] = companyName
	}
	if industry := strings.TrimSpace(req.Industry); industry != "" {
		cfg.Extras["industry"] = industry
	}
	if prompt := strings.TrimSpace(req.Prompt); prompt != "" {
		cfg.Extras["business_prompt"] = prompt
	}
	cfg.Extras["company_template_domain"] = plan.TemplateDomain
	cfg.Extras["company_type"] = plan.CompanyType
	cfg.Extras["deployment_preference"] = strings.TrimSpace(req.Deployment)
	cfg.Extras["admin_name"] = strings.TrimSpace(req.AdminName)
	cfg.Extras["admin_email"] = strings.TrimSpace(req.AdminEmail)
	s.settings = cfg
	s.mu.Unlock()
	_ = s.hub.SettingsStore().Update(cfg)
}

func buildFormationDocuments(req wizardBootstrapBusinessRequest, plan businessArchitecturePlan) []formationDocument {
	companyName := firstNonEmpty(strings.TrimSpace(req.CompanyName), "New Company")
	stateHint := "the target state"
	if strings.Contains(strings.ToLower(req.Prompt), "california") {
		stateHint = "California"
	}
	return []formationDocument{
		{Name: "Articles of Organization", Status: "drafted", Description: fmt.Sprintf("Drafted formation packet for %s to register in %s.", companyName, stateHint)},
		{Name: "Operating Agreement", Status: "drafted", Description: fmt.Sprintf("Prepared ownership, governance, and operating rules for the %s launch team.", strings.ToLower(plan.CompanyType))},
		{Name: "EIN & Compliance Checklist", Status: "ready_for_review", Description: "Compiled the tax registration, licensing, and banking checklist for the new company."},
	}
}

func businessBrief(req wizardBootstrapBusinessRequest) string {
	parts := []string{
		strings.TrimSpace(req.Prompt),
		strings.TrimSpace(req.CompanyName),
		strings.TrimSpace(req.Industry),
		strings.Join(req.Goals, " "),
	}
	return strings.ToLower(strings.Join(parts, " "))
}

func resolveBusinessTemplate(brief string) string {
	switch {
	case containsAny(brief, "tax", "bookkeeping", "payroll", "audit", "accounting", "cpa"):
		return "accounting_firm"
	case containsAny(brief, "marketing", "agency", "staging", "real estate", "home", "listing", "brand", "lead generation", "seo"):
		return "digital_marketing_agency"
	default:
		return "software_company"
	}
}

func businessTemplateRationale(templateDomain, brief string) string {
	switch templateDomain {
	case "digital_marketing_agency":
		if containsAny(brief, "staging", "real estate", "home") {
			return "The brief is service-led and design-heavy, so the planner selected a delivery + growth architecture that fits a real-estate staging company."
		}
		return "The brief is growth and client-delivery oriented, so the planner selected the digital marketing agency operating model."
	case "accounting_firm":
		return "The brief centers on finance and compliance work, so the planner selected the accounting firm operating model."
	default:
		return "The brief is best served by a product-and-engineering-centric launch team, so the planner selected the software company operating model."
	}
}

func companyTypeLabel(templateDomain string) string {
	switch templateDomain {
	case "digital_marketing_agency":
		return "Client Services Company"
	case "accounting_firm":
		return "Accounting Firm"
	default:
		return "Software Company"
	}
}

func existingCEOName(org domain.Organization) string {
	if member, ok := org.MemberByID(org.CEOID); ok {
		return member.Name
	}
	return ""
}

func containsAny(input string, needles ...string) bool {
	for _, needle := range needles {
		if strings.Contains(input, needle) {
			return true
		}
	}
	return false
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		if strings.TrimSpace(value) != "" {
			return strings.TrimSpace(value)
		}
	}
	return ""
}

// handleWizardOnboardingVerify performs a diagnostic verification of the environment variables
// and connection requirements for Day One onboarding.
func (s *Server) handleWizardOnboardingVerify(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}

	var diagnostics []map[string]interface{}
	allHealthy := true

	if mode == "cloud" {
		dbUrl := os.Getenv("DATABASE_URL")
		if dbUrl == "" {
			allHealthy = false
			diagnostics = append(diagnostics, map[string]interface{}{
				"check":   "DATABASE_URL",
				"status":  "missing",
				"message": "DATABASE_URL is required in cloud mode",
			})
		} else {
			diagnostics = append(diagnostics, map[string]interface{}{
				"check":   "DATABASE_URL",
				"status":  "ok",
				"message": "DATABASE_URL is configured",
			})
		}

		redisUrl := os.Getenv("REDIS_URL")
		if redisUrl == "" {
			allHealthy = false
			diagnostics = append(diagnostics, map[string]interface{}{
				"check":   "REDIS_URL",
				"status":  "missing",
				"message": "REDIS_URL is required in cloud mode",
			})
		} else {
			diagnostics = append(diagnostics, map[string]interface{}{
				"check":   "REDIS_URL",
				"status":  "ok",
				"message": "REDIS_URL is configured",
			})
		}
	} else {
		diagnostics = append(diagnostics, map[string]interface{}{
			"check":   "OHC_STANDALONE",
			"status":  "ok",
			"message": "Standalone mode active",
		})
	}

	respStatus := "healthy"
	if !allHealthy {
		respStatus = "degraded"
	}

	resp := map[string]interface{}{
		"status":      respStatus,
		"mode":        mode,
		"diagnostics": diagnostics,
	}
	writeJSON(w, resp)
}
