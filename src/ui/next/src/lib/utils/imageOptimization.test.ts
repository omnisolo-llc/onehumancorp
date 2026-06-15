import { optimizeImage } from './imageOptimization';
import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock browser globals
global.URL.createObjectURL = vi.fn(() => 'blob:test');
global.URL.revokeObjectURL = vi.fn();
// @ts-ignore
global.Worker = undefined; // Force fallback path in tests

describe('imageOptimization', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    it('should return original file if not an image', async () => {
        const file = new File(['text content'], 'test.txt', { type: 'text/plain' });
        const result = await optimizeImage(file, false);
        expect(result).toBe(file);
    });

    it('should return original file if small image', async () => {
        // Less than 50KB
        const smallBuffer = new ArrayBuffer(10 * 1024);
        const file = new File([smallBuffer], 'test.jpg', { type: 'image/jpeg' });
        const result = await optimizeImage(file, false);
        expect(result).toBe(file);
    });
});
