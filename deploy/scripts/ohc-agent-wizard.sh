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
echo -e "  1) Operations (The Manager)"
echo -e "  2) Customer Success (The Ambassador)"
echo -e "  3) Marketing (The Promoter)"
echo -e "  4) Sales (The Closer)"
echo -e "  5) Finance (The Accountant)"
echo -e "  6) Legal (The Counsel)"
echo -e "  7) Advisory (The Strategist)"
read -p "Select a role [1-7] (default: 1): " role_choice
agent_role="Operations"
case $role_choice in
    2) agent_role="Customer Success" ;;
    3) agent_role="Marketing" ;;
    4) agent_role="Sales" ;;
    5) agent_role="Finance" ;;
    6) agent_role="Legal" ;;
    7) agent_role="Advisory" ;;
esac
echo -e "\n${BOLD}Provider Type:${RESET}"
echo -e "  1) OpenAI"
echo -e "  2) MiniMax"
echo -e "  3) Anthropic"
echo -e "  4) Ollama"
echo -e "  5) OpenAI-Compatible"
read -p "Select a provider type [1-5] (default: 1): " provider_choice
provider_type="openai"
case $provider_choice in
    2) provider_type="minimax" ;;
    3) provider_type="anthropic" ;;
    4) provider_type="ollama" ;;
    5) provider_type="openai-compatible" ;;
esac

echo -e "\n${BOLD}Model (optional):${RESET}"
read -p "Enter a model name (leave blank for provider default): " agent_model

PORT=${PORT:-8080}
API_URL="http://127.0.0.1:${PORT}/api/agents/hire"
echo -e "\n${DIM}[Hiring ${agent_name} as ${agent_role} via ${API_URL}...]${RESET}"

PAYLOAD="{\"name\": \"${agent_name}\", \"role\": \"${agent_role}\", \"providerType\": \"${provider_type}\""
if [ -n "$agent_model" ]; then
    PAYLOAD="${PAYLOAD}, \"model\": \"${agent_model}\""
fi
PAYLOAD="${PAYLOAD}}"

RESPONSE=$(curl -s -w "\n%{http_code}" -X POST -H "Content-Type: application/json" -d "$PAYLOAD" "$API_URL" || echo "failed")
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
