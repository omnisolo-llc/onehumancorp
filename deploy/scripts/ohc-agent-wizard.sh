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

# Loop for agent name until a non-empty name is provided
agent_name=""
while [ -z "$agent_name" ]; do
    read -p "Enter a name for your agent (e.g., Nova, Jules): " agent_name
    if [ -z "$agent_name" ]; then
        echo -e "${PURPLE}Name cannot be empty. Please try again.${RESET}"
    fi
done

echo -e "\n${BOLD}Available Roles:${RESET}"
echo -e "  1) Software Engineer"
echo -e "  2) QA Automation"
echo -e "  3) SRE/DevOps"
echo -e "  4) Data Analyst"
echo -e "  5) Custom Role"
read -p "Select a role [1-5] (default: 1): " role_choice
agent_role="Software Engineer"
case $role_choice in
    2) agent_role="QA Automation" ;;
    3) agent_role="SRE/DevOps" ;;
    4) agent_role="Data Analyst" ;;
    5)
       read -p "Enter custom role name: " custom_role
       if [ -n "$custom_role" ]; then
           agent_role="$custom_role"
       else
           echo -e "${PURPLE}Custom role cannot be empty, defaulting to Software Engineer.${RESET}"
       fi
       ;;
esac

echo -e "\n${BOLD}Provider Type:${RESET}"
echo -e "  1) Cloud (OpenAI/Anthropic)"
echo -e "  2) Local (Standalone)"
read -p "Select a provider type [1-2] (default: 1): " provider_choice
provider_type="cloud"
if [ "$provider_choice" == "2" ]; then
    provider_type="local"
fi

echo -e "\n${BOLD}Model Selection:${RESET}"
model_name=""
if [ "$provider_type" == "cloud" ]; then
    echo -e "  1) gpt-4o"
    echo -e "  2) gpt-4o-mini"
    echo -e "  3) claude-3-5-sonnet-20240620"
    echo -e "  4) Custom"
    read -p "Select a model [1-4] (default: 1): " model_choice
    case $model_choice in
        2) model_name="gpt-4o-mini" ;;
        3) model_name="claude-3-5-sonnet-20240620" ;;
        4) read -p "Enter custom model name: " custom_model; model_name="${custom_model:-gpt-4o}" ;;
        *) model_name="gpt-4o" ;;
    esac
else
    echo -e "  1) llama-3-8b-instruct"
    echo -e "  2) mistral-7b-instruct-v0.2"
    echo -e "  3) Custom"
    read -p "Select a model [1-3] (default: 1): " model_choice
    case $model_choice in
        2) model_name="mistral-7b-instruct-v0.2" ;;
        3) read -p "Enter custom model name: " custom_model; model_name="${custom_model:-llama-3-8b-instruct}" ;;
        *) model_name="llama-3-8b-instruct" ;;
    esac
fi

PORT=${PORT:-8080}
API_URL="http://127.0.0.1:${PORT}/api/agents/hire"

echo -e "\n${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      Agent Summary                                   ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}Name:${RESET}         $agent_name"
echo -e "${BOLD}Role:${RESET}         $agent_role"
echo -e "${BOLD}Provider:${RESET}     $provider_type"
echo -e "${BOLD}Model:${RESET}        $model_name"
echo -e "${BOLD}${BLUE}======================================================${RESET}"

read -p "Do you want to proceed with hiring this agent? [Y/n]: " confirm
if [[ "$confirm" =~ ^[nN] ]]; then
    echo -e "${PURPLE}Agent hiring aborted.${RESET}"
    exit 0
fi

echo -e "\n${DIM}[Hiring ${agent_name} as ${agent_role} via ${API_URL}...]${RESET}"

# Send POST request
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST -H "Content-Type: application/json" -d "{\"name\": \"${agent_name}\", \"role\": \"${agent_role}\", \"providerType\": \"${provider_type}\", \"model\": \"${model_name}\"}" "$API_URL" || echo "failed")

HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" == "200" ] || [ "$HTTP_CODE" == "201" ]; then
    echo -e "${GREEN}✓ Agent hired successfully!${RESET}"
    if command -v jq &> /dev/null; then
        echo -e "${DIM}Response:${RESET}"
        echo "$BODY" | jq .
    else
        echo -e "${DIM}Response: ${BODY}${RESET}"
    fi
elif [ "$HTTP_CODE" == "failed" ]; then
    echo -e "${PURPLE}✗ Failed to connect to OHC Backend on port ${PORT}.${RESET}"
    echo -e "${DIM}Please ensure the server is running (e.g., using 'Launch Standalone Desktop Mode').${RESET}"
else
    echo -e "${PURPLE}✗ Failed to hire agent. Server returned HTTP ${HTTP_CODE}.${RESET}"
    if command -v jq &> /dev/null; then
        echo -e "${DIM}Response:${RESET}"
        echo "$BODY" | jq .
    else
        echo -e "${DIM}Response: ${BODY}${RESET}"
    fi
fi
echo ""
