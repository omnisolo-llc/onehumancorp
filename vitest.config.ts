import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import tsconfigPaths from 'vite-tsconfig-paths'

export default defineConfig({
  plugins: [react(), tsconfigPaths()],
  test: {
    globals: true,
    environment: 'jsdom',
    exclude: ['**/node_modules/**', '**/e2e/**'],
    setupFiles: ['./src/ui/next/src/tests/setup.ts'],
    alias: {
      'next/link': require.resolve('./src/ui/next/src/tests/next-link-mock.ts'),
      'next/navigation': require.resolve('./src/ui/next/src/tests/next-navigation-mock.ts'),
      'next/server': require.resolve('./src/ui/next/src/tests/next-server-mock.ts')
    }
  }
})
