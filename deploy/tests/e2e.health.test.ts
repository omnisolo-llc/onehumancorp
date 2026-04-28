/**
 * Health Endpoint E2E Tests
 * Converted from e2e_health_tests.sh
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { httpGet, assertJsonField, waitForServer, sleep } from './test-utils';

describe('Health Endpoints', () => {
  beforeAll(async () => {
    await waitForServer();
  });

  describe('Liveness Checks', () => {
    it('should return liveness status as ok', async () => {
      const resp = await httpGet('/healthz');
      assertJsonField(resp, '.status', 'ok');
    });

    it('should return 200 status code for liveness', async () => {
      const response = await fetch('http://127.0.0.1:18080/healthz');
      expect(response.status).toBe(200);
    });

    it('should handle repeated liveness checks', async () => {
      for (let i = 0; i < 5; i++) {
        const response = await fetch('http://127.0.0.1:18080/healthz');
        expect(response.status).toBe(200);
      }
    });

    it('should handle concurrent liveness checks', async () => {
      const promises = Array.from({ length: 10 }, () =>
        fetch('http://127.0.0.1:18080/healthz'),
      );
      const responses = await Promise.all(promises);
      responses.forEach(resp => {
        expect(resp.status).toBe(200);
      });
    });

    it('should complete liveness check within 1 second', async () => {
      const startTime = Date.now();
      const response = await fetch('http://127.0.0.1:18080/healthz');
      const duration = Date.now() - startTime;
      expect(response.status).toBe(200);
      expect(duration).toBeLessThan(1000);
    });

    it('should handle sequential liveness checks', async () => {
      for (let i = 0; i < 20; i++) {
        const response = await fetch('http://127.0.0.1:18080/healthz');
        expect(response.status).toBe(200);
        await sleep(100);
      }
    });

    it('should accept liveness check with query parameters', async () => {
      const response = await fetch('http://127.0.0.1:18080/healthz?foo=bar');
      expect(response.status).toBe(200);
    });
  });

  describe('Readiness Checks', () => {
    it('should return readiness status as ready', async () => {
      const resp = await httpGet('/readyz');
      assertJsonField(resp, '.status', 'ready');
    });

    it('should return 200 status code for readiness', async () => {
      const response = await fetch('http://127.0.0.1:18080/readyz');
      expect(response.status).toBe(200);
    });

    it('should handle concurrent readiness checks', async () => {
      const promises = Array.from({ length: 10 }, () =>
        fetch('http://127.0.0.1:18080/readyz'),
      );
      const responses = await Promise.all(promises);
      responses.forEach(resp => {
        expect(resp.status).toBe(200);
      });
    });

    it('should complete readiness check within 1 second', async () => {
      const startTime = Date.now();
      const response = await fetch('http://127.0.0.1:18080/readyz');
      const duration = Date.now() - startTime;
      expect(response.status).toBe(200);
      expect(duration).toBeLessThan(1000);
    });

    it('should accept readiness check with query parameters', async () => {
      const response = await fetch('http://127.0.0.1:18080/readyz?foo=bar');
      expect(response.status).toBe(200);
    });
  });

  describe('Mixed Endpoint Tests', () => {
    it('should handle both health and readiness endpoints', async () => {
      const healthResp = await fetch('http://127.0.0.1:18080/healthz');
      const readyResp = await fetch('http://127.0.0.1:18080/readyz');
      expect(healthResp.status).toBe(200);
      expect(readyResp.status).toBe(200);
    });

    it('should return non-empty response for health endpoint', async () => {
      const resp = await httpGet('/healthz');
      expect(resp.length).toBeGreaterThan(0);
    });

    it('should return valid JSON from health endpoint', async () => {
      const resp = await httpGet('/healthz');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should return 404 for invalid health endpoint', async () => {
      const response = await fetch('http://127.0.0.1:18080/health');
      expect(response.status).toBe(404);
    });

    it('should not allow POST to health endpoint', async () => {
      const response = await fetch('http://127.0.0.1:18080/healthz', {
        method: 'POST',
      });
      expect([405, 400]).toContain(response.status); // Method Not Allowed or Bad Request
    });

    it('should handle timeout gracefully', async () => {
      const controller = new AbortController();
      const timeout = setTimeout(() => controller.abort(), 5000);
      try {
        const response = await fetch('http://127.0.0.1:18080/healthz', {
          signal: controller.signal,
        });
        expect(response.status).toBe(200);
      } finally {
        clearTimeout(timeout);
      }
    });
  });

  describe('Response Format', () => {
    it('should return valid JSON from health endpoint', async () => {
      const resp = await httpGet('/healthz');
      const parsed = JSON.parse(resp);
      expect(parsed).toHaveProperty('status');
    });

    it('should return valid JSON from readiness endpoint', async () => {
      const resp = await httpGet('/readyz');
      const parsed = JSON.parse(resp);
      expect(parsed).toHaveProperty('status');
    });
  });
});
