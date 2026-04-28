/**
 * Skills API E2E Tests
 * Converted from e2e_skills_tests.sh
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { httpGet, httpPost, assertJsonField, waitForServer } from './test-utils';

describe('Skills API', () => {
  beforeAll(async () => {
    await waitForServer();
  });

  describe('List Skills', () => {
    it('should return skills list', async () => {
      const resp = await httpGet('/api/skills');
      assertJsonField(resp, '.skills');
    });

    it('should return valid JSON for skills list', async () => {
      const resp = await httpGet('/api/skills');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle skills list with category filter', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/skills?category=programming');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle skills list with limit', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/skills?limit=20');
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Import Skills', () => {
    it('should import basic skill', async () => {
      const resp = await httpPost('/api/skills/import', {
        name: 'python',
        level: 'expert',
        category: 'programming',
      }, 201);
      assertJsonField(resp, '.id');
    });

    it('should import skill with full details', async () => {
      const resp = await httpPost('/api/skills/import', {
        name: 'project-management',
        level: 'advanced',
        category: 'management',
        description: 'Experienced in agile and waterfall',
        years_experience: 5,
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should import minimal skill', async () => {
      const resp = await httpPost('/api/skills/import', {
        name: 'communication',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should list skills after import', async () => {
      await httpPost('/api/skills/import', {
        name: 'negotiation',
      }, 201);
      const resp = await httpGet('/api/skills');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle concurrent skill import', async () => {
      const promises = Array.from({ length: 5 }, (_, i) =>
        httpPost('/api/skills/import', {
          name: `skill-${i}`,
          level: 'expert',
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(5);
    });

    it('should handle sequential skill import', async () => {
      for (let i = 0; i < 10; i++) {
        const resp = await httpPost('/api/skills/import', {
          name: `sequential-skill-${i}`,
        }, 201);
        expect(resp).toBeTruthy();
      }
    });

    it('should handle invalid JSON', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/skills/import', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{invalid}',
      });
      expect(response.status).toBe(400);
    });

    it('should handle empty JSON object', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/skills/import', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{}',
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should import skill with proficiency level', async () => {
      const resp = await httpPost('/api/skills/import', {
        name: 'typescript',
        level: 'intermediate',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should import skill with years of experience', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/skills/import', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: 'kubernetes',
          years_experience: 3,
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should handle batch skill import', async () => {
      const promises = Array.from({ length: 3 }, (_, i) =>
        httpPost('/api/skills/import', {
          name: `batch-skill-${i}`,
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(3);
    });
  });

  describe('Performance', () => {
    it('should complete skills list request within 2 seconds', async () => {
      const startTime = Date.now();
      await httpGet('/api/skills');
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });
  });
});
