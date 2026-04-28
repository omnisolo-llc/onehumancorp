/**
 * Approvals API E2E Tests
 * Converted from e2e_approvals_tests.sh
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { httpGet, httpPost, assertJsonField, waitForServer } from './test-utils';

describe('Approvals API', () => {
  beforeAll(async () => {
    await waitForServer();
  });

  describe('List Approvals', () => {
    it('should return approvals list', async () => {
      const resp = await httpGet('/api/approvals');
      assertJsonField(resp, '.approvals');
    });

    it('should return valid JSON for approvals list', async () => {
      const resp = await httpGet('/api/approvals');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle approvals list with status filter', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/approvals?status=pending');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle approvals list with limit', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/approvals?limit=20');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle approvals list pagination', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/approvals?page=1&size=10');
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Request Approval', () => {
    it('should request basic approval', async () => {
      const resp = await httpPost('/api/approvals/request', {
        title: 'Approval Request',
        requester: 'agent1',
        type: 'spending',
      }, 201);
      assertJsonField(resp, '.id');
    });

    it('should request approval with full details', async () => {
      const resp = await httpPost('/api/approvals/request', {
        title: 'Full Approval',
        requester: 'agent1',
        approver: 'agent2',
        type: 'contract',
        amount: 5000,
        currency: 'USD',
        justification: 'Business need',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should request approval with minimal data', async () => {
      const resp = await httpPost('/api/approvals/request', {
        title: 'Quick Approval',
        requester: 'agent1',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should request approval with multiple approvers', async () => {
      const resp = await httpPost('/api/approvals/request', {
        title: 'Multi Approval',
        requester: 'agent1',
        approvers: ['agent2', 'agent3', 'agent4'],
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should request approval with deadline', async () => {
      const resp = await httpPost('/api/approvals/request', {
        title: 'Deadline Approval',
        requester: 'agent1',
        deadline: '2024-12-31T23:59:59Z',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should request approval with budget', async () => {
      const resp = await httpPost('/api/approvals/request', {
        title: 'Budget Approval',
        requester: 'agent1',
        amount: 10000,
        budget_code: 'ENG-2024',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should handle invalid JSON', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/approvals/request', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{invalid}',
      });
      expect(response.status).toBe(400);
    });

    it('should handle empty JSON object', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/approvals/request', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{}',
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should list approvals after request', async () => {
      await httpPost('/api/approvals/request', {
        title: 'Tracked Approval',
        requester: 'agent1',
      }, 201);
      const resp = await httpGet('/api/approvals');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle concurrent approval requests', async () => {
      const promises = Array.from({ length: 5 }, (_, i) =>
        httpPost('/api/approvals/request', {
          title: `concurrent-approval-${i}`,
          requester: 'agent1',
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(5);
    });

    it('should handle sequential approval requests', async () => {
      for (let i = 0; i < 10; i++) {
        const resp = await httpPost('/api/approvals/request', {
          title: `sequential-approval-${i}`,
          requester: 'agent1',
        }, 201);
        expect(resp).toBeTruthy();
      }
    });

    it('should handle long title', async () => {
      const title = 'x'.repeat(500);
      const response = await fetch('http://127.0.0.1:18080/api/approvals/request', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ title, requester: 'agent1' }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });
  });

  describe('Decide Approval', () => {
    it('should approve request', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/approvals/decide', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          approval_id: 'test-approval-1',
          decision: 'approved',
          approver: 'agent2',
          comments: 'Looks good',
        }),
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should reject request', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/approvals/decide', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          approval_id: 'test-approval-2',
          decision: 'rejected',
          approver: 'agent2',
          comments: 'Needs more info',
        }),
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should mark request as pending', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/approvals/decide', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          approval_id: 'test-approval-3',
          decision: 'pending',
          approver: 'agent2',
        }),
      });
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Performance', () => {
    it('should complete approval request within 2 seconds', async () => {
      const startTime = Date.now();
      await httpPost('/api/approvals/request', {
        title: 'Perf Test Approval',
        requester: 'agent1',
      }, 201);
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });
  });
});
