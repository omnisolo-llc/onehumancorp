import { vi } from 'vitest';
export const useRouter = vi.fn().mockReturnValue({
  push: vi.fn(),
  replace: vi.fn(),
  prefetch: vi.fn(),
  back: vi.fn(),
  forward: vi.fn()
});
export const useParams = vi.fn().mockReturnValue({});
export const usePathname = vi.fn().mockReturnValue('/');