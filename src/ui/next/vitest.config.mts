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
      '**/bazel-*/**',
      '**/.bazel-*/**',
      'bazel-*',
      '**/node_modules/**',
      '**/dist/**',
      '**/e2e/**',
      '**/external/**',
      '**/.next/**',
      '**/coverage/**',
      // '**/api/**',
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
      'swagger-ui-react/swagger-ui.css': path.resolve(__dirname, './src/mocks/empty.css'),
    },
  },
})
