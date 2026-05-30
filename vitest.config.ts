import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    exclude: ['**/node_modules/**', '**/e2e/**', 'verification_tests/**', 'bazel-app/**', 'bazel-out/**', 'bazel-bin/**'],
  }
})
