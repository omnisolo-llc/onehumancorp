#!/bin/bash
echo -e "\033[1m\033[38;5;39m======================================================\033[0m"
echo -e "\033[1m\033[38;5;87m      OHC: Interactive Swarm Status Viewer            \033[0m"
echo -e "\033[1m\033[38;5;39m======================================================\033[0m"
echo ""
echo -e "\033[1mActive Missions:\033[0m"
for file in .agent-task/missions/*.md; do
    if grep -q "status:.*IN_PROGRESS" "$file" 2>/dev/null || grep -q "status:.*\"IN_PROGRESS\"" "$file" 2>/dev/null; then
        agent=$(grep -i "agent:" "$file" | head -n 1 | sed 's/[^a-zA-Z0-9 ]//g' | sed 's/agent//gi' | xargs)
        echo -e "  \033[38;5;120m[IN PROGRESS]\033[0m ${file} (Agent: ${agent:-Unknown})"
    fi
done
echo ""
echo -e "\033[1mRecently Completed Missions:\033[0m"
count=0
for file in $(ls -t .agent-task/missions/*.md 2>/dev/null); do
    if grep -q "status:.*DONE" "$file" 2>/dev/null || grep -q "status:.*\"DONE\"" "$file" 2>/dev/null; then
        agent=$(grep -i "agent:" "$file" | head -n 1 | sed 's/[^a-zA-Z0-9 ]//g' | sed 's/agent//gi' | xargs)
        echo -e "  \033[38;5;39m[DONE]\033[0m ${file} (Agent: ${agent:-Unknown})"
        count=$((count+1))
        if [ "$count" -ge 5 ]; then break; fi
    fi
done
echo ""
