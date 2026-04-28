/**
 * Snapshots API E2E Tests
 * Converted from e2e_snapshots_tests.sh
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { httpGet, httpPost, assertJsonField, waitForServer } from './test-utils';

describe('Snapshots API', () => {
  beforeAll(async () => {
    await waitForServer();
  });

  describe('List Snapshots', () => {
    it('should return snapshots list', async () => {
      const resp = await httpGet('/api/snapshots');
      assertJsonField(resp, '.snapshots');
    });

    it('should return valid JSON for snapshots list', async () => {
      const resp = await httpGet('/api/snapshots');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle snapshots list with tag filter', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/snapshots?tag=important');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle snapshots list with limit', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/snapshots?limit=20');
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Create Snapshots', () => {
    it('should create basic snapshot', async () => {
      const resp = await httpPost('/api/snapshots/create', {
        name: 'snapshot-1',
        description: 'Test snapshot',
      }, 201);
      assertJsonField(resp, '.id');
    });

    it('should create snapshot with full details', async () => {
      const resp = await httpPost('/api/snapshots/create', {
        name: 'full-snapshot',
        description: 'Full snapshot with metadata',
        tags: ['important', 'backup'],
        retention_days: 90,
        metadata: { version: '1.0', env: 'prod' },
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should create minimal snapshot', async () => {
      const resp = await httpPost('/api/snapshots/create', {
        name: 'minimal-snapshot',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should list snapshots after create', async () => {
      await httpPost('/api/snapshots/create', {
        name: 'listed-snapshot',
      }, 201);
      const resp = await httpGet('/api/snapshots');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle concurrent snapshot creation', async () => {
      const promises = Array.from({ length: 5 }, (_, i) =>
        httpPost('/api/snapshots/create', {
          name: `concurrent-snapshot-${i}`,
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(5);
    });

    it('should handle sequential snapshot creation', async () => {
      for (let i = 0; i < 10; i++) {
        const resp = await httpPost('/api/snapshots/create', {
          name: `sequential-snapshot-${i}`,
        }, 201);
        expect(resp).toBeTruthy();
      }
    });

    it('should handle invalid JSON', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/snapshots/create', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{invalid}',
      });
      expect(response.status).toBe(400);
    });

    it('should handle empty JSON object', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/snapshots/create', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{}',
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should create snapshot with tags', async () => {
      const resp = await httpPost('/api/snapshots/create', {
        name: 'tagged-snapshot',
        tags: ['production', 'critical'],
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should create snapshot with retention policy', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/snapshots/create', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: 'retained-snapshot',
          retention_days: 30,
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should create snapshot with environment metadata', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/snapshots/create', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: 'env-snapshot',
          metadata: { environment: 'staging', timestamp: '2024-01-01T00:00:00Z' },
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should handle snapshot with large description', async () => {
      const description = 'x'.repeat(1000);
      const response = await fetch('http://127.0.0.1:18080/api/snapshots/create', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: 'large-snapshot',
          description,
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should handle batch snapshot creation', async () => {
      const promises = Array.from({ length: 3 }, (_, i) =>
        httpPost('/api/snapshots/create', {
          name: `batch-snapshot-${i}`,
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(3);
    });
  });

  describe('Performance', () => {
    it('should complete snapshots list request within 2 seconds', async () => {
      const startTime = Date.now();
      await httpGet('/api/snapshots');
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });
  });
});
