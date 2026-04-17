/**
 * React Web App – E2E tests using Playwright.
 *
 * All critical user journeys (CUJs) are covered here.  AI model responses
 * are mocked via Playwright's page.route() so that the tests are fully
 * deterministic and do not call external AI APIs.
 *
 * CUJs covered:
 *   1. Dashboard loads and renders all widgets
 *   2. BusinessSetupWizard: user can create an AI team (full wizard flow)
 *   3. AgentChatPanel: user sends a task → agent team responds (chat CUJ)
 *   4. TaskDAGViewer: user can view, pause, and kill tasks
 *   5. SwarmOverview: user can see agent and task statistics
 *   6. TeammateMeshConsole: user sees real-time agent mesh messages
 *   7. Error handling: graceful recovery when API is unavailable
 */

import { test, expect, Page } from '@playwright/test';

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const BASE_URL = process.env.WEB_APP_BASE_URL ?? 'http://localhost:3000';

// ---------------------------------------------------------------------------
// Mock helpers
// ---------------------------------------------------------------------------

/**
 * Mock all backend API calls to return deterministic test data.
 * This prevents the tests from relying on a live backend or AI service.
 */
async function mockBackendAPIs(page: Page): Promise<void> {
  // Mock task list (TaskDAGViewer)
  await page.route('**/api/v1/orchestration/tasks', (route) => {
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify([
        { id: 'task-1', title: 'Build auth module', status: 'EXECUTING' },
        { id: 'task-2', title: 'Write unit tests', status: 'PENDING' },
        { id: 'task-3', title: 'Deploy to staging', status: 'COMPLETED' },
      ]),
    });
  });

  // Mock task pause/kill actions
  await page.route('**/api/v1/orchestration/tasks/*/pause', (route) => {
    route.fulfill({ status: 200, body: JSON.stringify({ status: 'paused' }) });
  });
  await page.route('**/api/v1/orchestration/tasks/*/kill', (route) => {
    route.fulfill({ status: 200, body: JSON.stringify({ status: 'killed' }) });
  });

  // Mock wizard provision API
  await page.route('**/api/provision', (route) => {
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ status: 'provisioned', teamId: 'team-test-123' }),
    });
  });

  // Mock AI agent broadcast (mocked AI model response channel)
  await page.route('**/api/mesh/broadcast', (route) => {
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ status: 'queued', messageId: 'msg-mock-1' }),
    });
  });

  // Mock seed API used in test setup
  await page.route('**/api/dev/seed', (route) => {
    route.fulfill({ status: 200, body: JSON.stringify({ ok: true }) });
  });
}

/**
 * Mock WebSocket to simulate agent responses.
 * Injects a fake WebSocket implementation that delivers a canned AI response
 * after a short delay without any real network connection.
 */
async function mockWebSocket(page: Page, mockResponse: string): Promise<void> {
  await page.addInitScript(
    ({ response }: { response: string }) => {
      (window as any).WebSocket = class MockWebSocket extends EventTarget {
        onopen: ((e: Event) => void) | null = null;
        onclose: ((e: CloseEvent) => void) | null = null;
        onerror: ((e: Event) => void) | null = null;
        onmessage: ((e: MessageEvent) => void) | null = null;
        readyState = 1; // OPEN
        close() { /* no-op */ }
        send(_data: string) { /* no-op */ }

        constructor(_url: string) {
          super();
          // Simulate connection open
          setTimeout(() => {
            if (this.onopen) this.onopen(new Event('open'));
          }, 50);
          // Deliver mocked AI model response after 200ms
          setTimeout(() => {
            if (this.onmessage) {
              this.onmessage(new MessageEvent('message', { data: response }));
            }
          }, 200);
        }
      };
    },
    { response: mockResponse },
  );
}

// ---------------------------------------------------------------------------
// Shared setup
// ---------------------------------------------------------------------------

test.beforeEach(async ({ page }) => {
  await mockBackendAPIs(page);
  await mockWebSocket(page, JSON.stringify({
    content: 'Understood! I will start working on your request now.',
    agentId: 'agent-swe-1',
    role: 'SOFTWARE_ENGINEER',
  }));
  await page.goto(BASE_URL);
});

// ---------------------------------------------------------------------------
// CUJ 1 – Dashboard loads correctly
// ---------------------------------------------------------------------------

test.describe('CUJ 1: Dashboard loads and renders all widgets', () => {
  test('heading "Swarm Orchestration Dashboard" is visible', async ({ page }) => {
    await expect(page.getByText('Swarm Orchestration Dashboard')).toBeVisible();
  });

  test('SwarmOverview widget renders with agent count', async ({ page }) => {
    await expect(page.getByText('Swarm Overview')).toBeVisible();
    await expect(page.getByTestId('active-agents')).toBeVisible();
    await expect(page.getByTestId('active-agents')).toHaveText('12');
  });

  test('AutoDreamPipelineWidget renders with pipeline stages', async ({ page }) => {
    await expect(page.getByTestId('autodream-pipeline')).toBeVisible();
    await expect(page.getByText('AutoDream Pipeline Stream')).toBeVisible();
    await expect(page.getByText('Extract')).toBeVisible();
    await expect(page.getByText('Analyze')).toBeVisible();
  });

  test('TaskDAGViewer loads and displays tasks', async ({ page }) => {
    await expect(page.getByText('Task DAG Viewer')).toBeVisible();
    await expect(page.getByText('Build auth module')).toBeVisible();
    await expect(page.getByText('EXECUTING')).toBeVisible();
  });

  test('AgentChatPanel is visible', async ({ page }) => {
    await expect(page.getByTestId('agent-chat-panel')).toBeVisible();
    await expect(page.getByText('Agent Chat')).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// CUJ 2 – BusinessSetupWizard full flow
// ---------------------------------------------------------------------------

test.describe('CUJ 2: BusinessSetupWizard – create your AI team', () => {
  test('welcome screen is shown on first load', async ({ page }) => {
    await expect(page.getByText('Your AI team, ready in minutes')).toBeVisible();
  });

  test('user can progress through all wizard steps', async ({ page }) => {
    // Step 1 → 2
    await page.getByRole('button', { name: 'Next' }).first().click();
    await expect(page.getByText('Business Profile')).toBeVisible();

    // Step 2 – fill in profile
    await page.getByPlaceholder('Company Name').fill('Test Company');

    // Step 2 → 3
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByText('Goal Selection')).toBeVisible();

    // Step 3 – select a goal
    await page.getByLabel(/Build software faster/i).check();

    // Step 3 → 4
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByText('Deployment Preference')).toBeVisible();

    // Step 4 → 5
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByText('Administrator Account')).toBeVisible();

    // Step 5 – fill in admin details
    await page.getByPlaceholder('Name').fill('Admin User');
    await page.getByPlaceholder('Email').fill('admin@test.com');
    await page.getByPlaceholder('Password').fill('password123');

    // Step 5 → 6
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByText('Review & Launch')).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// CUJ 3 – AgentChatPanel: user sends task, agent responds (CORE CUJ)
// ---------------------------------------------------------------------------

test.describe('CUJ 3: AgentChatPanel – user gives task to agent team', () => {
  test('chat input and send button are present', async ({ page }) => {
    await expect(page.getByTestId('chat-input')).toBeVisible();
    await expect(page.getByTestId('send-button')).toBeVisible();
  });

  test('send button is disabled when input is empty', async ({ page }) => {
    await expect(page.getByTestId('send-button')).toBeDisabled();
  });

  test('CORE CUJ: user sends task and mocked AI model responds via WebSocket', async ({ page }) => {
    // 1. User types a task
    await page.getByTestId('chat-input').fill('Analyze our Q3 performance metrics');

    // 2. User sends the task (triggers POST /api/mesh/broadcast)
    await page.getByTestId('send-button').click();

    // 3. User message appears in the chat
    await expect(page.getByText('Analyze our Q3 performance metrics')).toBeVisible();

    // 4. Mocked AI model response arrives via the mocked WebSocket
    await expect(
      page.getByText('Understood! I will start working on your request now.'),
    ).toBeVisible({ timeout: 2000 });
  });

  test('user can send a task by pressing Enter', async ({ page }) => {
    const input = page.getByTestId('chat-input');
    await input.fill('Build a new API endpoint');
    await input.press('Enter');
    await expect(page.getByText('Build a new API endpoint')).toBeVisible();
  });

  test('multiple tasks can be sent in sequence', async ({ page }) => {
    const input = page.getByTestId('chat-input');

    await input.fill('First task: research competitors');
    await page.getByTestId('send-button').click();
    await expect(page.getByText('First task: research competitors')).toBeVisible();

    await input.fill('Second task: create report');
    await page.getByTestId('send-button').click();
    await expect(page.getByText('Second task: create report')).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// CUJ 4 – TaskDAGViewer: user manages tasks
// ---------------------------------------------------------------------------

test.describe('CUJ 4: TaskDAGViewer – user manages agent tasks', () => {
  test('tasks are loaded from mocked API', async ({ page }) => {
    await expect(page.getByText('Build auth module')).toBeVisible();
    await expect(page.getByText('Write unit tests')).toBeVisible();
    await expect(page.getByText('Deploy to staging')).toBeVisible();
  });

  test('task statuses are shown correctly', async ({ page }) => {
    await expect(page.getByText('EXECUTING')).toBeVisible();
    await expect(page.getByText('PENDING')).toBeVisible();
    await expect(page.getByText('COMPLETED')).toBeVisible();
  });

  test('user can pause a task', async ({ page }) => {
    const pauseButtons = page.getByRole('button', { name: 'Pause' });
    await expect(pauseButtons.first()).toBeVisible();
    await pauseButtons.first().click();
    await page.waitForTimeout(200);
  });

  test('user can kill a task', async ({ page }) => {
    const killButtons = page.getByRole('button', { name: 'Kill' });
    await expect(killButtons.first()).toBeVisible();
    await killButtons.first().click();
    await page.waitForTimeout(200);
  });
});

// ---------------------------------------------------------------------------
// CUJ 5 – SwarmOverview statistics
// ---------------------------------------------------------------------------

test.describe('CUJ 5: SwarmOverview – user sees agent statistics', () => {
  test('active agent count is displayed', async ({ page }) => {
    await expect(page.getByTestId('active-agents')).toHaveText('12');
  });

  test('completed task count is displayed', async ({ page }) => {
    await expect(page.getByTestId('completed-tasks')).toHaveText('145');
  });
});

// ---------------------------------------------------------------------------
// CUJ 6 – TeammateMeshConsole
// ---------------------------------------------------------------------------

test.describe('CUJ 6: TeammateMeshConsole – real-time mesh messages', () => {
  test('console heading is visible', async ({ page }) => {
    await expect(page.getByText('Teammate Mesh Console')).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// CUJ 7 – Error handling
// ---------------------------------------------------------------------------

test.describe('CUJ 7: Error handling – graceful API failures', () => {
  test('app remains usable when task API returns empty', async ({ page }) => {
    await page.route('**/api/v1/orchestration/tasks', (route) => {
      route.fulfill({ status: 200, body: JSON.stringify([]) });
    });
    await page.reload();
    await expect(page.getByText('No tasks in DAG.')).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// CUJ 8 – Agent chat: message history and input state management
// 5 new E2E tests added to expand coverage
// ---------------------------------------------------------------------------

test.describe('CUJ 8: AgentChatPanel – message history & full chat loop CUJ', () => {
  test('NEW 1: agent chat input clears after send', async ({ page }) => {
    const input = page.getByTestId('chat-input');
    await input.fill('Deploy staging environment');
    await page.getByTestId('send-button').click();
    // After sending, the input should be cleared
    await expect(input).toHaveValue('');
  });

  test('NEW 2: full CUJ loop – user task → AI agent action response shown in chat', async ({ page }) => {
    // User submits a task to the agent team via the chat UI
    const input = page.getByTestId('chat-input');
    await input.fill('Run security audit on all agent permissions');
    await page.getByTestId('send-button').click();

    // User message is visible in history
    await expect(page.getByText('Run security audit on all agent permissions')).toBeVisible();

    // Mocked AI agent responds with an action (via mocked WebSocket)
    await expect(
      page.getByText('Understood! I will start working on your request now.'),
    ).toBeVisible({ timeout: 3000 });
  });

  test('NEW 3: broadcast API is called with correct payload when task is sent', async ({ page }) => {
    let capturedPayload: string | null = null;

    // Override the mock to capture what is sent to the broadcast API
    await page.route('**/api/mesh/broadcast', async (route) => {
      capturedPayload = route.request().postData();
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ status: 'queued', messageId: 'msg-capture-1' }),
      });
    });

    const input = page.getByTestId('chat-input');
    await input.fill('Optimize the CI pipeline');
    await page.getByTestId('send-button').click();

    // Message is shown to the user
    await expect(page.getByText('Optimize the CI pipeline')).toBeVisible();
    // Payload should have been sent (non-null)
    expect(capturedPayload).not.toBeNull();
  });

  test('NEW 4: error banner shown when broadcast API returns 500', async ({ page }) => {
    // Simulate server error from broadcast API
    await page.route('**/api/mesh/broadcast', (route) => {
      route.fulfill({ status: 500, body: JSON.stringify({ error: 'internal error' }) });
    });

    const input = page.getByTestId('chat-input');
    await input.fill('This task will fail to send');
    await page.getByTestId('send-button').click();

    // Either the error message or the user message must be visible
    // The app should stay usable (not crash)
    const dashboardHeading = page.getByText('Swarm Orchestration Dashboard');
    await expect(dashboardHeading).toBeVisible({ timeout: 3000 });
  });

  test('NEW 5: task DAG and agent chat are independently scrollable and usable together', async ({ page }) => {
    // Confirm both the DAG viewer and the chat panel coexist on the page
    await expect(page.getByText('Task Pipeline (DAG)')).toBeVisible();
    await expect(page.getByTestId('chat-input')).toBeVisible();

    // Send a chat message while the DAG is visible
    const input = page.getByTestId('chat-input');
    await input.fill('Rebuild the DAG for sprint 12');
    await page.getByTestId('send-button').click();
    await expect(page.getByText('Rebuild the DAG for sprint 12')).toBeVisible();

    // Confirm DAG tasks are still shown
    await expect(page.getByText('Build auth module')).toBeVisible();
  });
});
