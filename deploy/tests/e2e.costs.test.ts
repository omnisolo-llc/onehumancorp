/**
 * Costs API E2E Tests
 * Converted from e2e_costs_tests.sh
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { httpGet, httpPost, assertJsonField, waitForServer } from './test-utils';

describe('Costs API', () => {
  beforeAll(async () => {
    await waitForServer();
  });

  describe('List Costs', () => {
    it('should return costs list', async () => {
      const resp = await httpGet('/api/costs');
      assertJsonField(resp, '.costs');
    });

    it('should return valid JSON for costs list', async () => {
      const resp = await httpGet('/api/costs');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle costs list with department filter', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/costs?department=engineering');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle costs list with limit', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/costs?limit=20');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle costs list pagination', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/costs?page=1&size=10');
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Query Costs', () => {
    it('should query costs with basic parameters', async () => {
      const resp = await httpPost('/api/costs', {
        period: 'month',
        year: 2024,
        month: 12,
      });
      assertJsonField(resp, '.total');
    });

    it('should query costs with full parameters', async () => {
      const resp = await httpPost('/api/costs', {
        period: 'month',
        year: 2024,
        month: 12,
        department: 'engineering',
        cost_center: 'CC001',
      });
      expect(resp).toBeTruthy();
    });

    it('should query costs with minimal parameters', async () => {
      const resp = await httpPost('/api/costs', {
        period: 'month',
      });
      expect(resp).toBeTruthy();
    });

    it('should list costs after query', async () => {
      await httpPost('/api/costs', { period: 'month' });
      const resp = await httpGet('/api/costs');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle concurrent cost queries', async () => {
      const promises = Array.from({ length: 5 }, (_, i) =>
        httpPost('/api/costs', {
          period: 'month',
          year: 2024,
          month: (i % 12) + 1,
        }),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(5);
    });

    it('should handle sequential cost queries', async () => {
      for (let i = 0; i < 10; i++) {
        const resp = await httpPost('/api/costs', {
          period: 'month',
          year: 2024,
          month: (i % 12) + 1,
        });
        expect(resp).toBeTruthy();
      }
    });

    it('should handle invalid JSON', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/costs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{invalid}',
      });
      expect(response.status).toBe(400);
    });

    it('should handle empty JSON object', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/costs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{}',
      });
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Performance', () => {
    it('should complete cost query within 2 seconds', async () => {
      const startTime = Date.now();
      await httpPost('/api/costs', { period: 'month' });
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });

    it('should complete cost list request within 2 seconds', async () => {
      const startTime = Date.now();
      await httpGet('/api/costs');
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });
  });
});
