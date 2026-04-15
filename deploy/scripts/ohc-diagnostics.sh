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
RUNTIME_DIR="${OHC_RUNTIME_DIR:-.ohc/runtime}"
MEMORY_DIR="${OHC_MEMORY_DIR:-${RUNTIME_DIR}/memory}"
STATUS_DIR="${OHC_STATUS_DIR:-${RUNTIME_DIR}/status}"
if [ -d "${RUNTIME_DIR}" ]; then
    echo -e "  ${GREEN}✓ ${RUNTIME_DIR} exists${RESET}"
else
    echo -e "  ${PURPLE}✗ ${RUNTIME_DIR} missing${RESET}"
    read -p "Create it now? (y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        mkdir -p "${MEMORY_DIR}" "${STATUS_DIR}"
        echo -e "  ${GREEN}✓ Directory created.${RESET}"
    fi
fi

# Verify Standalone DB
echo -e "\n${DIM}[Verifying Standalone Database]${RESET}"
DB_FILE="${RUNTIME_DIR}/swarm.db"
if [ -f "$DB_FILE" ]; then
    echo -e "  ${GREEN}✓ Local SQLite database exists${RESET}"
else
    echo -e "  ${PURPLE}✗ Local SQLite database not found at $DB_FILE${RESET}"
    echo -e "    ${DIM}(Hint: Launch Standalone Desktop Mode first to create it)${RESET}"
fi

echo -e "\n${BOLD}${BLUE}======================================================${RESET}"
if [ $MISSING_TOOLS -eq 0 ]; then
    echo -e "${BOLD}${GREEN}✓ Interactive Environment Diagnostics Passed${RESET}"
else
    echo -e "${BOLD}${PURPLE}✗ Diagnostics failed due to missing tools.${RESET}"
fi
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""
