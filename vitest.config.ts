import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    exclude: ['**/node_modules/**', '**/e2e/**', 'bazel-*/**', 'verification_tests/**'],
    setupFiles: ['./vitest-setup.ts'],
  },
  resolve: {
    alias: {
      'next/link': require.resolve('./src/ui/next/test-utils/next-link-mock.js'),
      'next/navigation': require.resolve('./src/ui/next/test-utils/next-navigation-mock.js'),
      'next/server': require.resolve('./src/ui/next/test-utils/next-server-mock.js')
    }
  }
})
