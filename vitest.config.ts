import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    setupFiles: ['./vitest.setup.ts'],
    environment: 'jsdom',
    exclude: ['**/node_modules/**', '**/e2e/**', '**/bazel-*/**', '**/verification_tests/**'],
  }
})
