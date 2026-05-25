import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./vitest.setup.ts'],
    exclude: ['**/node_modules/**', '**/e2e/**', '**/bazel-app/**', 'bazel-app/**', 'bazel-*/**', '**/verification_tests/**'],
    alias: {
      'next/link': path.resolve(__dirname, './__mocks__/next/link.tsx'),
      'ink-spinner': path.resolve(__dirname, './__mocks__/ink-spinner.tsx'),
      'ink-text-input': path.resolve(__dirname, './__mocks__/ink-text-input.tsx')
    }
  }
})
