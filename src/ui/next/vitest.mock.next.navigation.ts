export const useRouter = () => ({
  push: vi.fn(),
  replace: vi.fn(),
  prefetch: vi.fn(),
});
export const usePathname = () => '';
export const useSearchParams = () => new URLSearchParams();
