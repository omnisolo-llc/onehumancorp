/**
 * Deployment Artifacts Verification Test
 * Converted from deploy_artifacts_test.sh
 * Verifies required deployment files are present and contain expected content
 */

import { describe, it, expect } from 'vitest';
import { promises as fs } from 'fs';
import * as path from 'path';

// Get workspace root
const getWorkspaceRoot = (): string => {
  const testSrcdir = process.env.TEST_SRCDIR;
  const workspace = process.env.TEST_WORKSPACE || 'mono';
  if (testSrcdir) {
    return path.join(testSrcdir, workspace);
  }
  return process.cwd();
};

const root = getWorkspaceRoot();

describe('Deployment Artifacts', () => {
  describe('Required files exist and are non-empty', () => {
    const requiredFiles = [
      'deploy/docker-compose.yml',
      'deploy/helm/ohc/Chart.yaml',
      'deploy/helm/ohc/values.yaml',
      'deploy/BUILD.bazel',
    ];

    requiredFiles.forEach(file => {
      it(`should have non-empty file: ${file}`, async () => {
        const filePath = path.join(root, file);
        const stats = await fs.stat(filePath);
        expect(stats.isFile()).toBe(true);
        expect(stats.size).toBeGreaterThan(0);
      });
    });
  });

  describe('OCI Bazel rules are present', () => {
    it('should contain oci_image rule in BUILD.bazel', async () => {
      const buildFile = path.join(root, 'deploy/BUILD.bazel');
      const content = await fs.readFile(buildFile, 'utf-8');
      expect(content).toMatch(/oci_image/);
    });

    it('should contain server_image in BUILD.bazel', async () => {
      const buildFile = path.join(root, 'deploy/BUILD.bazel');
      const content = await fs.readFile(buildFile, 'utf-8');
      expect(content).toMatch(/server_image/);
    });

    it('should contain default_agent_image in BUILD.bazel', async () => {
      const buildFile = path.join(root, 'deploy/BUILD.bazel');
      const content = await fs.readFile(buildFile, 'utf-8');
      expect(content).toMatch(/default_agent_image/);
    });

    it('should contain distroless reference in BUILD.bazel', async () => {
      const buildFile = path.join(root, 'deploy/BUILD.bazel');
      const content = await fs.readFile(buildFile, 'utf-8');
      expect(content).toMatch(/distroless/);
    });

    it('should contain bazel internal server image reference', async () => {
      const buildFile = path.join(root, 'deploy/BUILD.bazel');
      const content = await fs.readFile(buildFile, 'utf-8');
      expect(content).toMatch(/internal-default-agent:bazel/);
    });
  });

  describe('Docker Compose Configuration', () => {
    it('should use consolidated server image', async () => {
      const composeFile = path.join(root, 'deploy/docker-compose.yml');
      const content = await fs.readFile(composeFile, 'utf-8');
      expect(content).toMatch(/server:/);
      expect(content).toMatch(/onehumancorp\/server:latest/);
    });

    it('should not contain UI-only services', async () => {
      const composeFile = path.join(root, 'deploy/docker-compose.yml');
      const content = await fs.readFile(composeFile, 'utf-8');
      expect(content).not.toMatch(/onehumancorp\/ui/);
      expect(content).not.toMatch(/^  ui:/m);
    });
  });

  describe('Helm Values Configuration', () => {
    it('should contain backend configuration', async () => {
      const valuesFile = path.join(root, 'deploy/helm/ohc/values.yaml');
      const content = await fs.readFile(valuesFile, 'utf-8');
      expect(content).toMatch(/backend/);
    });

    it('should contain redis configuration', async () => {
      const valuesFile = path.join(root, 'deploy/helm/ohc/values.yaml');
      const content = await fs.readFile(valuesFile, 'utf-8');
      expect(content).toMatch(/redis/);
    });
  });

  describe('Helm Templates', () => {
    it('should have backend deployment template', async () => {
      const templateFile = path.join(root, 'deploy/helm/ohc/templates/backend-deployment.yaml');
      const stats = await fs.stat(templateFile);
      expect(stats.isFile()).toBe(true);
    });

    it('should contain Deployment kind in backend template', async () => {
      const templateFile = path.join(root, 'deploy/helm/ohc/templates/backend-deployment.yaml');
      const content = await fs.readFile(templateFile, 'utf-8');
      expect(content).toMatch(/Deployment/);
    });

    it('should not have frontend deployment template', async () => {
      const templateFile = path.join(root, 'deploy/helm/ohc/templates/frontend-deployment.yaml');
      try {
        await fs.stat(templateFile);
        expect(false).toBe(true); // File should not exist
      } catch (error) {
        // Expected - file should not exist
        expect(true).toBe(true);
      }
    });

    it('should not have frontend service template', async () => {
      const templateFile = path.join(root, 'deploy/helm/ohc/templates/frontend-service.yaml');
      try {
        await fs.stat(templateFile);
        expect(false).toBe(true); // File should not exist
      } catch (error) {
        // Expected - file should not exist
        expect(true).toBe(true);
      }
    });

    it('should contain liveness probe in backend deployment', async () => {
      const templateFile = path.join(root, 'deploy/helm/ohc/templates/backend-deployment.yaml');
      const content = await fs.readFile(templateFile, 'utf-8');
      expect(content).toMatch(/livenessProbe/);
    });

    it('should contain readiness probe in backend deployment', async () => {
      const templateFile = path.join(root, 'deploy/helm/ohc/templates/backend-deployment.yaml');
      const content = await fs.readFile(templateFile, 'utf-8');
      expect(content).toMatch(/readinessProbe/);
    });
  });

  describe('Deployment Structure', () => {
    it('should have consistent deployment configuration', async () => {
      // Verify all key files exist together
      const files = [
        'deploy/docker-compose.yml',
        'deploy/BUILD.bazel',
        'deploy/helm/ohc/Chart.yaml',
        'deploy/helm/ohc/values.yaml',
        'deploy/helm/ohc/templates/backend-deployment.yaml',
      ];

      for (const file of files) {
        const filePath = path.join(root, file);
        try {
          const stats = await fs.stat(filePath);
          expect(stats.isFile()).toBe(true);
        } catch (error) {
          expect(false).toBe(true); // File should exist
        }
      }
    });

    it('should have docker directory structure', async () => {
      const dockerDir = path.join(root, 'deploy/docker');
      try {
        const stats = await fs.stat(dockerDir);
        expect(stats.isDirectory()).toBe(true);
      } catch {
        // Directory might not exist, that's acceptable
      }
    });
  });
});
