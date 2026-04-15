#!/bin/bash
echo -e "\033[1m\033[38;5;39m======================================================\033[0m"
echo -e "\033[1m\033[38;5;87m      OHC: Interactive Swarm Status Viewer            \033[0m"
echo -e "\033[1m\033[38;5;39m======================================================\033[0m"
echo ""
REPO="${OHC_GITHUB_REPO:-onehumancorp/mono}"

if command -v gh >/dev/null 2>&1; then
    echo -e "\033[1mOpen GitHub Issues:\033[0m"
    gh issue list --repo "$REPO" --state open --limit 10 || true
    echo ""
    echo -e "\033[1mRecently Closed GitHub Issues:\033[0m"
    gh issue list --repo "$REPO" --state closed --limit 5 || true
else
    echo -e "\033[1mTask Tracking Source:\033[0m GitHub Issues"
    echo "Install GitHub CLI to query issues from the terminal: https://cli.github.com/"
    echo "Repository: $REPO"
fi
echo ""
