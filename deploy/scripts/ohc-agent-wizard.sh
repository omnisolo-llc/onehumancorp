#!/bin/bash
# OHC Hybrid Agentic OS - Day One Agent Provisioning Wizard
set -e
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      OHC: Interactive Agent Provisioning Wizard      ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""
echo -e "${DIM}Let's hire your first AI Agent for the OHC Swarm!${RESET}"
echo ""
read -p "Enter a name for your agent (e.g., Nova, Jules): " agent_name
if [ -z "$agent_name" ]; then
    echo -e "${PURPLE}Name cannot be empty. Aborting.${RESET}"
    exit 1
fi
echo -e "\n${BOLD}Available Roles:${RESET}"
echo -e "  1) The Manager"
echo -e "  2) The Promoter"
echo -e "  3) The Salesperson"
echo -e "  4) The Ambassador"
echo -e "  5) The Accountant"
echo -e "  6) The Protector"
echo -e "  7) The Advisor"
read -p "Select a role [1-7] (default: 1): " role_choice
agent_role="The Manager"
case $role_choice in
    2) agent_role="The Promoter" ;;
    3) agent_role="The Salesperson" ;;
    4) agent_role="The Ambassador" ;;
    5) agent_role="The Accountant" ;;
    6) agent_role="The Protector" ;;
    7) agent_role="The Advisor" ;;
esac
echo -e "\n${BOLD}Provider Type:${RESET}"
echo -e "  1) Cloud (OpenAI/Anthropic)"
echo -e "  2) Local (Standalone)"
read -p "Select a provider type [1-2] (default: 1): " provider_choice
provider_type="cloud"
if [ "$provider_choice" == "2" ]; then
    provider_type="local"
fi
PORT=${PORT:-8080}
API_URL="http://127.0.0.1:${PORT}/api/agents/hire"
echo -e "\n${DIM}[Hiring ${agent_name} as ${agent_role} via ${API_URL}...]${RESET}"
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST -H "Content-Type: application/json" -d "{\"name\": \"${agent_name}\", \"role\": \"${agent_role}\", \"providerType\": \"${provider_type}\"}" "$API_URL" || echo "failed")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')
if [ "$HTTP_CODE" == "200" ] || [ "$HTTP_CODE" == "201" ]; then
    echo -e "${GREEN}✓ Agent hired successfully!${RESET}"
    echo -e "${DIM}Response: ${BODY}${RESET}"
elif [ "$HTTP_CODE" == "failed" ]; then
    echo -e "${PURPLE}✗ Failed to connect to OHC Backend on port ${PORT}.${RESET}"
    echo -e "${DIM}Please ensure the server is running (e.g., using 'Launch Standalone Desktop Mode').${RESET}"
else
    echo -e "${PURPLE}✗ Failed to hire agent. Server returned HTTP ${HTTP_CODE}.${RESET}"
    echo -e "${DIM}Response: ${BODY}${RESET}"
fi
echo ""
