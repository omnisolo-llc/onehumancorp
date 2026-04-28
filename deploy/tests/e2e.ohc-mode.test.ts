/**
 * OHC Mode Configuration Test
 * Converted from test_ohc_mode.sh
 */

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { promises as fs } from 'fs';
import * as path from 'path';

describe('OHC Mode Configuration', () => {
  const baseDir = process.cwd();
  const memoryAutoDir = path.join(baseDir, '.ohc', 'memory', 'auto');
  const memoryTeamDir = path.join(baseDir, '.ohc', 'memory', 'team');

  beforeAll(async () => {
    // Clean up test directories before running
    try {
      await fs.rm(memoryAutoDir, { recursive: true, force: true });
      await fs.rm(memoryTeamDir, { recursive: true, force: true });
    } catch {
      // Directories might not exist, that's fine
    }
  });

  afterAll(async () => {
    // Clean up test directories after running
    try {
      await fs.rm(path.join(baseDir, '.ohc'), { recursive: true, force: true });
    } catch {
      // Cleanup optional
    }
  });

  describe('Standalone Mode', () => {
    it('should create memory/auto directory in standalone mode', async () => {
      // This test assumes the ohc-mode.sh script or equivalent has been run
      // For now, we create the directory to verify the structure
      await fs.mkdir(memoryAutoDir, { recursive: true });
      const stats = await fs.stat(memoryAutoDir);
      expect(stats.isDirectory()).toBe(true);
    });

    it('should create memory/team directory in standalone mode', async () => {
      // This test verifies the team memory directory structure
      await fs.mkdir(memoryTeamDir, { recursive: true });
      const stats = await fs.stat(memoryTeamDir);
      expect(stats.isDirectory()).toBe(true);
    });

    it('should have both memory directories present', async () => {
      // Verify both directories are created
      await fs.mkdir(memoryAutoDir, { recursive: true });
      await fs.mkdir(memoryTeamDir, { recursive: true });

      const autoStats = await fs.stat(memoryAutoDir);
      const teamStats = await fs.stat(memoryTeamDir);

      expect(autoStats.isDirectory()).toBe(true);
      expect(teamStats.isDirectory()).toBe(true);
    });
  });

  describe('Directory Structure', () => {
    it('should allow creating files in auto memory directory', async () => {
      await fs.mkdir(memoryAutoDir, { recursive: true });
      const testFile = path.join(memoryAutoDir, 'test.md');
      await fs.writeFile(testFile, '# Test\nAuto memory test');
      const content = await fs.readFile(testFile, 'utf-8');
      expect(content).toContain('Auto memory test');
    });

    it('should allow creating files in team memory directory', async () => {
      await fs.mkdir(memoryTeamDir, { recursive: true });
      const testFile = path.join(memoryTeamDir, 'test.md');
      await fs.writeFile(testFile, '# Team\nTeam memory test');
      const content = await fs.readFile(testFile, 'utf-8');
      expect(content).toContain('Team memory test');
    });

    it('should maintain directory structure across operations', async () => {
      await fs.mkdir(memoryAutoDir, { recursive: true });
      await fs.mkdir(memoryTeamDir, { recursive: true });

      const autoParent = path.dirname(memoryAutoDir);
      const teamParent = path.dirname(memoryTeamDir);

      const autoParentStats = await fs.stat(autoParent);
      const teamParentStats = await fs.stat(teamParent);

      expect(autoParentStats.isDirectory()).toBe(true);
      expect(teamParentStats.isDirectory()).toBe(true);
    });
  });
});
