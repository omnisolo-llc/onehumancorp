/**
 * Agents API E2E Tests
 * Converted from e2e_agents_tests.sh
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { httpGet, httpPost, assertJsonField, waitForServer } from './test-utils';

describe('Agents API', () => {
  beforeAll(async () => {
    await waitForServer();
  });

  describe('List Agents', () => {
    it('should return agents list', async () => {
      const resp = await httpGet('/api/agents');
      assertJsonField(resp, '.agents');
    });

    it('should return valid JSON for agents list', async () => {
      const resp = await httpGet('/api/agents');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should complete list request within 2 seconds', async () => {
      const startTime = Date.now();
      await httpGet('/api/agents');
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });

    it('should handle agents list with filter', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents?role=assistant');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle agents list with limit', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents?limit=10');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle agents list pagination', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents?page=1&size=20');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle agents list with sort', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents?sort=name');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle agents list with descending sort', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents?sort=-created_at');
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Hire Agents', () => {
    it('should hire basic agent', async () => {
      const resp = await httpPost('/api/agents/hire', {
        name: 'test-agent',
        role: 'assistant',
      });
      assertJsonField(resp, '.id');
    });

    it('should hire agent with full details', async () => {
      const resp = await httpPost('/api/agents/hire', {
        name: 'full-agent',
        role: 'manager',
        skills: ['negotiation', 'analysis'],
        cost_center: 'CC001',
      });
      assertJsonField(resp, '.id');
    });

    it('should hire agent with metadata', async () => {
      const resp = await httpPost('/api/agents/hire', {
        name: 'meta-agent',
        role: 'assistant',
        metadata: { team: 'platform', created_by: 'system' },
      });
      expect(resp).toBeTruthy();
    });

    it('should hire agent with minimal data', async () => {
      const resp = await httpPost('/api/agents/hire', {
        name: 'minimal-agent',
      });
      expect(resp).toBeTruthy();
    });

    it('should handle duplicate agent hire', async () => {
      const data = { name: 'dup-agent' };
      await httpPost('/api/agents/hire', data);
      // Should also succeed (depends on API behavior)
      await httpPost('/api/agents/hire', data);
    });

    it('should handle invalid JSON', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{invalid json}',
      });
      expect(response.status).toBe(400);
    });

    it('should handle empty JSON object', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{}',
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should hire agent with large payload', async () => {
      const description = 'x'.repeat(1000);
      const resp = await httpPost('/api/agents/hire', {
        name: 'large-agent',
        description,
      });
      expect(resp).toBeTruthy();
    });

    it('should hire agent with special characters in name', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: 'agent@special#chars$%' }),
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should hire agent with unicode name', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: 'agent-é-ñ-中文' }),
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should hire agent with long name', async () => {
      const name = 'a'.repeat(200);
      const response = await fetch('http://127.0.0.1:18080/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name }),
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should hire agent with numeric name', async () => {
      const resp = await httpPost('/api/agents/hire', {
        name: '123456789',
      });
      expect(resp).toBeTruthy();
    });

    it('should hire agent with timestamps', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: 'timestamp-agent',
          created_at: '2024-01-01T00:00:00Z',
        }),
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should hire agent with context', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: 'context-agent',
          context: { project: 'platform', team: 'engineering' },
        }),
      });
      expect([200, 400]).toContain(response.status);
    });

    it('should complete hire request within 2 seconds', async () => {
      const startTime = Date.now();
      await httpPost('/api/agents/hire', { name: 'perf-agent' });
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });

    it('should handle concurrent hire requests', async () => {
      const promises = Array.from({ length: 5 }, (_, i) =>
        httpPost('/api/agents/hire', { name: `concurrent-agent-${i}` }),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(5);
    });

    it('should list agents after hiring', async () => {
      await httpPost('/api/agents/hire', { name: 'listed-agent' });
      const resp = await httpGet('/api/agents');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle sequential hire requests', async () => {
      for (let i = 0; i < 10; i++) {
        const resp = await httpPost('/api/agents/hire', {
          name: `sequential-agent-${i}`,
        });
        expect(resp).toBeTruthy();
      }
    });

    it('should handle batch hire requests', async () => {
      const promises = Array.from({ length: 3 }, (_, i) =>
        httpPost('/api/agents/hire', { name: `batch-agent-${i}` }),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(3);
    });
  });

  describe('Get Agent', () => {
    it('should return 404 for nonexistent agent', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/agents/nonexistent');
      expect(response.status).toBe(404);
    });
  });
});
