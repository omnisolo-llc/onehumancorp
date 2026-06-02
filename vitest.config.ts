import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./setupTests.ts'],
    exclude: [
      '**/node_modules/**',
      '**/e2e/**',
      'src/ui/next/**',
      'bazel-app/**',
      'bazel-bin/**',
      'bazel-out/**',
      'bazel-testlogs/**',
      'bazel-mono/**',
      'bazel-**/**',
      'verification_tests/**'
    ],
  }
})
