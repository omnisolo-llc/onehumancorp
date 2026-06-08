import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./setupTests.ts'],
    exclude: [
      '**/node_modules/**',
      '**/.bazel/**',
      '**/.cache/bazel/**',
      '**/bazel-*/**',
      '**/bazel-out/**',
      '**/bazel-bin/**',
      '**/bazel-app/**',
      '**/e2e/**',
      '**/verification_tests/**',
      '**/external/**',
    ],
  }
})
