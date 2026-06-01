import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['src/ui/next/vitest.setup.ts'],
    exclude: ['**/*.spec.ts', '**/node_modules/**', '**/e2e/**', '**/bazel-out/**', '**/_tmp/**'],
  }
})
