#!/bin/bash
# OHC Hybrid Agentic OS - Day One Agent Provisioning Wizard

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
YELLOW="\033[38;5;220m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      OHC: Interactive Agent Provisioning Wizard      ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

PORT=${PORT:-8080}
API_URL="http://127.0.0.1:${PORT}/api/agents/hire"
HEALTH_URL="http://127.0.0.1:${PORT}/healthz"

echo -e "${DIM}Checking if OHC backend is running on port ${PORT}...${RESET}"
if ! curl -s -f "$HEALTH_URL" > /dev/null; then
    echo -e "${PURPLE}✗ OHC Backend is not reachable at ${HEALTH_URL}.${RESET}"
    echo -e "${DIM}Please ensure the server is running (e.g., using 'Launch Quick Start (Standalone)').${RESET}"
    exit 1
fi

echo -e "${GREEN}✓ Backend is reachable.${RESET}"
echo ""

echo -e "${DIM}Let's hire your first AI Agent for the OHC Swarm!${RESET}"
echo ""

while true; do
    read -p "Enter a name for your agent (e.g., Nova, Jules): " agent_name
    if [ -z "$agent_name" ]; then
        echo -e "${PURPLE}Name cannot be empty. Please try again.${RESET}"
    else
        break
    fi
done

echo -e "\n${BOLD}Available Roles:${RESET}"
echo -e "  1) Software Engineer"
echo -e "  2) QA Tester"
echo -e "  3) Security Engineer"
echo -e "  4) AI News Collector"
read -p "Select a role [1-4] (default: 1): " role_choice

agent_role="SOFTWARE_ENGINEER"
role_display="Software Engineer"
case $role_choice in
    2) agent_role="QA_TESTER"; role_display="QA Tester" ;;
    3) agent_role="SECURITY_ENGINEER"; role_display="Security Engineer" ;;
    4) agent_role="AI_NEWS_COLLECTOR"; role_display="AI News Collector" ;;
    *) agent_role="SOFTWARE_ENGINEER"; role_display="Software Engineer" ;;
esac

echo -e "\n${BOLD}Provider Type:${RESET}"
echo -e "  1) Cloud (OpenAI/Anthropic/Gemini)"
echo -e "  2) Local (Standalone)"
read -p "Select a provider type [1-2] (default: 1): " provider_choice

provider_type="cloud"
if [ "$provider_choice" == "2" ]; then
    provider_type="local"
fi

echo -e "\n${DIM}[Hiring ${agent_name} as ${role_display} via ${API_URL}...]${RESET}"

# Post request using the correct enum string mapping
PAYLOAD="{\"name\": \"${agent_name}\", \"role\": \"${agent_role}\", \"providerType\": \"${provider_type}\"}"

RESPONSE=$(curl -s -w "\n%{http_code}" -X POST -H "Content-Type: application/json" -d "$PAYLOAD" "$API_URL" || echo "failed")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" == "200" ] || [ "$HTTP_CODE" == "201" ]; then
    echo -e "${GREEN}✓ Agent hired successfully!${RESET}"
    echo -e "${DIM}Response: ${BODY}${RESET}"
elif [ "$HTTP_CODE" == "failed" ]; then
    echo -e "${PURPLE}✗ Failed to connect to OHC Backend on port ${PORT}.${RESET}"
else
    echo -e "${PURPLE}✗ Failed to hire agent. Server returned HTTP ${HTTP_CODE}.${RESET}"
    echo -e "${DIM}Response: ${BODY}${RESET}"
fi

echo ""
