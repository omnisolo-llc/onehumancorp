# Saga Coordinator Architecture

## Overview
The Saga Coordinator manages distributed transactions across AI Agents. It follows the Orchestrator pattern.

## Database Schema
The state is tracked in PostgreSQL:
- `saga_instances`: Tracks overall saga progress (e.g., BookServiceWithDepositSaga).
- `saga_steps`: Tracks individual steps (e.g., BookCalendar, ProcessPayment).

## Workflow
1. An Agent or API initiates a Saga.
2. The Coordinator logs the start state in the DB.
3. Steps execute sequentially (or via job queues in production).
4. If a step fails, the Coordinator automatically triggers `Compensate()` for previously successful steps.

## Statuses
- `PENDING`
- `IN_PROGRESS`
- `COMPLETED`
- `FAILED`
- `COMPENSATING`
- `COMPENSATED`
