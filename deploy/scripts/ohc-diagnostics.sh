#!/bin/bash
set -e

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}      OHC: Interactive Environment Diagnostics        ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

# Interactive prompt to begin
read -p "Start Diagnostics? (y/n) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    echo "Aborted."
    return 1 2>/dev/null || true
fi

echo -e "\n${DIM}[Running Environment Health Checks]${RESET}"

# Check for essential tools
TOOLS=("bazelisk" "docker" "go" "sqlite3")
MISSING_TOOLS=0
for tool in "${TOOLS[@]}"; do
    if command -v $tool >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓ $tool installed${RESET}"
    else
        echo -e "  ${PURPLE}✗ $tool not found${RESET}"
        MISSING_TOOLS=$((MISSING_TOOLS+1))
    fi
done

# Verify memory directory
echo -e "\n${DIM}[Verifying Agent Memory Storage]${RESET}"
if [ -d ".agent-task" ]; then
    echo -e "  ${GREEN}✓ .agent-task directory exists${RESET}"
else
    echo -e "  ${PURPLE}✗ .agent-task directory missing${RESET}"
    read -p "Create it now? (y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        mkdir -p .agent-task/memory .agent-task/status .agent-task/missions
        echo -e "  ${GREEN}✓ Directory created.${RESET}"
    fi
fi

# Verify Standalone DB
echo -e "\n${DIM}[Verifying Standalone Database]${RESET}"
DB_FILE="$HOME/.ohc-local-data/standalone.db"
if [ -f "$DB_FILE" ]; then
    echo -e "  ${GREEN}✓ Local SQLite database exists${RESET}"
else
    echo -e "  ${PURPLE}✗ Local SQLite database not found at $DB_FILE${RESET}"
    echo -e "    ${DIM}(Hint: Launch Standalone Desktop Mode first to create it)${RESET}"
fi


# Verify .env File
echo -e "\n${DIM}[Verifying Environment Configuration]${RESET}"
if [ -f ".env" ]; then
    echo -e "  ${GREEN}✓ .env file exists${RESET}"
    if grep -q "PORT" ".env"; then
        echo -e "  ${GREEN}✓ PORT variable is defined${RESET}"
    else
        echo -e "  ${PURPLE}✗ PORT variable missing in .env${RESET}"
    fi
else
    echo -e "  ${PURPLE}✗ .env file not found${RESET}"
    echo -e "    ${DIM}(Hint: Run Interactive Environment Wizard via option 'e')${RESET}"
    MISSING_TOOLS=$((MISSING_TOOLS+1))
fi

# Check Port Conflicts
echo -e "\n${DIM}[Verifying Network Ports]${RESET}"
PORT_VAL=8080
if [ -f ".env" ] && grep -q "^PORT=" ".env"; then
    PORT_VAL=$(grep "^PORT=" ".env" | cut -d '=' -f2)
fi
if command -v lsof >/dev/null 2>&1; then
    if lsof -i :$PORT_VAL >/dev/null 2>&1; then
        echo -e "  ${PURPLE}✗ Port $PORT_VAL is currently in use!${RESET}"
        echo -e "    ${DIM}(Hint: This will cause 'Failed to launch Cloud Backend' errors)${RESET}"
        MISSING_TOOLS=$((MISSING_TOOLS+1))
    else
        echo -e "  ${GREEN}✓ Port $PORT_VAL is available${RESET}"
    fi
else
    echo -e "  ${DIM}  lsof not installed, skipping port check${RESET}"
fi

# Check Directory Permissions
echo -e "\n${DIM}[Verifying Directory Permissions]${RESET}"
LOCAL_DATA_DIR="$HOME/.ohc-local-data"
if [ -d "$LOCAL_DATA_DIR" ]; then
    if [ -w "$LOCAL_DATA_DIR" ]; then
        echo -e "  ${GREEN}✓ $LOCAL_DATA_DIR is writable${RESET}"
    else
        echo -e "  ${PURPLE}✗ $LOCAL_DATA_DIR is NOT writable!${RESET}"
        MISSING_TOOLS=$((MISSING_TOOLS+1))
    fi
else
    echo -e "  ${DIM}  $LOCAL_DATA_DIR does not exist yet (will be created on first run)${RESET}"
fi

echo -e "\n${BOLD}${BLUE}======================================================${RESET}"
if [ $MISSING_TOOLS -eq 0 ]; then
    echo -e "${BOLD}${GREEN}✓ Interactive Environment Diagnostics Passed${RESET}"
else
    echo -e "${BOLD}${PURPLE}✗ Diagnostics failed due to missing tools.${RESET}"
fi
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""
