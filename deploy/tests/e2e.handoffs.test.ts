/**
 * Handoffs API E2E Tests
 * Converted from e2e_handoffs_tests.sh
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { httpGet, httpPost, assertJsonField, waitForServer } from './test-utils';

describe('Handoffs API', () => {
  beforeAll(async () => {
    await waitForServer();
  });

  describe('List Handoffs', () => {
    it('should return handoffs list', async () => {
      const resp = await httpGet('/api/handoffs');
      assertJsonField(resp, '.handoffs');
    });

    it('should return valid JSON for handoffs list', async () => {
      const resp = await httpGet('/api/handoffs');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle handoffs list with from_agent filter', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/handoffs?from_agent=agent1');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle handoffs list with limit', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/handoffs?limit=20');
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Create Handoffs', () => {
    it('should create basic handoff', async () => {
      const resp = await httpPost('/api/handoffs', {
        from_agent: 'agent1',
        to_agent: 'agent2',
        context: 'task transfer',
      }, 201);
      assertJsonField(resp, '.id');
    });

    it('should create handoff with full details', async () => {
      const resp = await httpPost('/api/handoffs', {
        from_agent: 'agent1',
        to_agent: 'agent2',
        context: 'full handoff',
        notes: 'Complete context transfer',
        data: { task_id: 'T123' },
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should create minimal handoff', async () => {
      const resp = await httpPost('/api/handoffs', {
        from_agent: 'agent1',
        to_agent: 'agent2',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should list handoffs after create', async () => {
      await httpPost('/api/handoffs', {
        from_agent: 'agent1',
        to_agent: 'agent2',
      }, 201);
      const resp = await httpGet('/api/handoffs');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle concurrent handoff creation', async () => {
      const promises = Array.from({ length: 5 }, (_, i) =>
        httpPost('/api/handoffs', {
          from_agent: 'agent1',
          to_agent: `agent${i + 1}`,
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(5);
    });

    it('should handle sequential handoff creation', async () => {
      for (let i = 0; i < 10; i++) {
        const resp = await httpPost('/api/handoffs', {
          from_agent: 'agent1',
          to_agent: 'agent2',
          context: `seq-${i}`,
        }, 201);
        expect(resp).toBeTruthy();
      }
    });

    it('should handle invalid JSON', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/handoffs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{invalid}',
      });
      expect(response.status).toBe(400);
    });

    it('should handle empty JSON object', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/handoffs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{}',
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should handle handoff with metadata', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/handoffs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          from_agent: 'agent1',
          to_agent: 'agent2',
          metadata: { priority: 'high' },
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should handle handoff with large context', async () => {
      const context = 'x'.repeat(1000);
      const response = await fetch('http://127.0.0.1:18080/api/handoffs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          from_agent: 'agent1',
          to_agent: 'agent2',
          context,
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should handle batch handoff creation', async () => {
      const promises = Array.from({ length: 3 }, (_, i) =>
        httpPost('/api/handoffs', {
          from_agent: 'agent1',
          to_agent: `agent${i + 2}`,
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(3);
    });
  });

  describe('Performance', () => {
    it('should complete handoff list request within 2 seconds', async () => {
      const startTime = Date.now();
      await httpGet('/api/handoffs');
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });
  });
});
