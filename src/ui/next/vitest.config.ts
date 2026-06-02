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
    alias: [
      { find: /^@\/(.*)/, replacement: path.resolve(__dirname, './src/$1') },
      { find: 'next/navigation', replacement: path.resolve(__dirname, './src/__mocks__/next/navigation.ts') },
      { find: 'next/link', replacement: path.resolve(__dirname, './src/__mocks__/next/link.ts') },
      { find: 'dompurify', replacement: path.resolve(__dirname, './src/__mocks__/dompurify.ts') },
      { find: 'swagger-ui-react/swagger-ui.css', replacement: path.resolve(__dirname, './src/__mocks__/swagger-ui-react.css') },
      { find: 'swagger-ui-react', replacement: path.resolve(__dirname, './src/__mocks__/swagger-ui-react.ts') },
    ]
  },
  css: {
    postcss: {
      plugins: [], // Disable PostCSS for tests to avoid tailwindcss dependency error on mock css
    }
  }
})
