#!/bin/bash
# OHC Hybrid Agentic OS - Interactive Quickstart Guide

set -e

# Premium aesthetics colors
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
BLUE="\033[38;5;39m"
CYAN="\033[38;5;87m"
GREEN="\033[38;5;120m"
PURPLE="\033[38;5;141m"
YELLOW="\033[38;5;220m"

echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo -e "${BOLD}${CYAN}         OHC: Day One Quickstart Guide                ${RESET}"
echo -e "${BOLD}${BLUE}======================================================${RESET}"
echo ""

echo -e "${BOLD}Welcome to the One Human Corp (OHC) Hybrid Agentic OS!${RESET}"
echo -e "${DIM}This guide will help you understand the architecture and get started.${RESET}"
echo ""

echo -e "${BOLD}${PURPLE}Architecture Modes:${RESET}"
echo -e "  ${YELLOW}1. Standalone Desktop Mode:${RESET} Runs locally with SQLite. Optimized for low resource usage."
echo -e "  ${YELLOW}2. Cloud-Native Mode:${RESET} Multi-tenant, K8s-orchestrated with PostgreSQL & Redis."
echo -e "  ${YELLOW}3. Headless Mode:${RESET} API-only, useful for CI/CD or custom frontends."
echo ""

echo -e "${BOLD}${PURPLE}Directory Structure:${RESET}"
echo -e "  ${GREEN}srcs/server/${RESET}     - Go backend codebase (Agents, Domain, Orchestration)"
echo -e "  ${GREEN}deploy/scripts/${RESET}  - Shell scripts for environments and tools"
echo -e "  ${GREEN}.agent-task/${RESET}     - Agent workspace (missions, memory, status)"
echo -e "  ${GREEN}bazel/${RESET}           - Build configurations"
echo ""

echo -e "${BOLD}${PURPLE}Next Steps:${RESET}"
echo -e "  ${CYAN}Step 1:${RESET} Select option '1' in the CLI to run initial setup."
echo -e "  ${CYAN}Step 2:${RESET} Select option '8' to seed the database with mock data."
echo -e "  ${CYAN}Step 3:${RESET} Select option '3' to launch Standalone Desktop Mode."
echo ""

echo -e "${DIM}Press any key to return to the main menu...${RESET}"
[ -t 0 ] && read -n 1 -s
echo ""
