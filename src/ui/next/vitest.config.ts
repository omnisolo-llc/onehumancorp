import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      'next/server': path.resolve(__dirname, 'vitest.mock.next.server.ts'),
      'next/navigation': path.resolve(__dirname, 'vitest.mock.next.navigation.ts'),
      'next/link': path.resolve(__dirname, 'vitest.mock.next.link.tsx'),
      'swagger-ui-react/swagger-ui.css': path.resolve(__dirname, 'vitest.mock.swagger.tsx'),
      'swagger-ui-react': path.resolve(__dirname, 'vitest.mock.swagger.tsx'),
      '@': path.resolve(__dirname, './src'),
    },
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./vitest.setup.ts'],
    exclude: ['**/node_modules/**', '**/e2e/**', '**/*.spec.ts'],
  }
})
