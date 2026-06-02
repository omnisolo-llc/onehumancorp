import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    setupFiles: ['./vitest.setup.ts'],
    globals: true,
    exclude: [
      '**/node_modules/**',
      '**/dist/**',
      '**/e2e/**',
      '**/.next/**',
      '**/coverage/**',
      '**/api/**', // API routes require Next.js specific testing utilities or E2E tests
    ],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
    },
  },
  resolve: {
    preserveSymlinks: true,
    alias: {
      '@': path.resolve(__dirname, './src'),
      'next/server': path.resolve(__dirname, '__mocks__/next/server.ts'),
      'next/navigation': path.resolve(__dirname, '__mocks__/next/navigation.ts'),
      'next/link': path.resolve(__dirname, '__mocks__/next/link.ts'),
      'dompurify': path.resolve(__dirname, '__mocks__/dompurify.ts'),
      'swagger-ui-react/swagger-ui.css': path.resolve(__dirname, '__mocks__/swagger-ui-react.css.ts'),
      'swagger-ui-react': path.resolve(__dirname, '__mocks__/swagger-ui-react.ts')
    },
  },
})
