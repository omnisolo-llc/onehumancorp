#!/bin/bash
# OHC Hybrid Agentic OS - Day One Interactive Environment Wizard

set -e

# Premium aesthetics colors
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}     OHC: Interactive Environment Wizard (.env)       ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

ENV_FILE=".env"

# Create a backup if .env already exists
if [ -f "$ENV_FILE" ]; then
    cp "$ENV_FILE" "${ENV_FILE}.bak"
    echo -e "${DIM}[Backup created at ${ENV_FILE}.bak]${RESET}"
else
    touch "$ENV_FILE"
    chmod 0600 "$ENV_FILE"
fi

update_env() {
    local key=$1
    local value=$2
    if grep -q "^${key}=" "$ENV_FILE"; then
        sed -i "s|^${key}=.*|${key}=${value}|" "$ENV_FILE"
    else
        echo "${key}=${value}" >> "$ENV_FILE"
    fi
}

echo -e "${BOLD}Let's configure your Day One environment variables.${RESET}"
echo ""

# 1. Logging Level
update_env "LOG_LEVEL" "info"

# 2. HTTP Port
update_env "PORT" "8080"

# 3. Mode configurations
echo -e "
${BOLD}Environment Mode Settings:${RESET}"
echo "1) Standalone Desktop Mode (Local, SQLite)"
echo "2) Cloud-native Mode (K8s, PostgreSQL)"
read -p "Select mode [1-2] (default: 1): " mode_choice
mode_choice=${mode_choice:-1}

if [ "$mode_choice" = "1" ]; then
    update_env "OHC_MULTITENANT" "false"
    update_env "OHC_SOURCE_MODE" "standalone"
    update_env "MCP_ENABLED" "false"
else
    update_env "OHC_MULTITENANT" "true"
    update_env "OHC_SOURCE_MODE" "cloud"
    update_env "MCP_ENABLED" "true"
fi


# 4. Agent LLM Providers
echo -e "\n${BOLD}LLM Provider Configuration:${RESET}"
read -p "OpenAI API Key (leave blank to skip): " openai_key
if [ -n "$openai_key" ]; then
    update_env "OPENAI_API_KEY" "$openai_key"
fi

read -p "Anthropic API Key (leave blank to skip): " anthropic_key
if [ -n "$anthropic_key" ]; then
    update_env "ANTHROPIC_API_KEY" "$anthropic_key"
fi

# 5. Database Settings (if applicable)
if [ "$mode_choice" = "2" ]; then
    echo -e "\n${BOLD}Cloud Mode Database Configuration:${RESET}"
    read -p "PostgreSQL DATABASE_URL (leave blank to skip): " db_url
    if [ -n "$db_url" ]; then
        update_env "DATABASE_URL" "$db_url"
    fi

    read -p "REDIS_URL (leave blank to skip): " redis_url
    if [ -n "$redis_url" ]; then
        update_env "REDIS_URL" "$redis_url"
    fi
fi

# 6. Advanced Features
# Defaulting to MCP based on mode

# Apply secure permissions to .env
chmod 0600 "$ENV_FILE"

echo -e "\n${GREEN}✓ Setup Complete! Your .env file is configured.${RESET}"
echo -e "${DIM}Note: You can run this wizard again anytime via deploy/scripts/ohc-env-wizard.sh.${RESET}"
echo ""
