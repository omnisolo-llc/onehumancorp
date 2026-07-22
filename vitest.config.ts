import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  resolve: {
    preserveSymlinks: true,
    alias: {
      '@': path.resolve(__dirname, './src/ui/next/src'),
    },
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: [path.resolve(__dirname, './src/ui/next/vitest.setup.ts')],
    include: [
      'src/ui/next/src/**/*.test.{ts,tsx}',
      'src/ui/tauri/src/**/*.test.{ts,tsx}',
    ],
    exclude: [
      '**/node_modules/**',
      '**/bazel-out/**',
      '**/bazel-bin/**',
      '**/bazel-mono/**',
      '**/bazel-testlogs/**',
      '**/bazel-workspace/**',
      '**/target/**',
      '**/.git/**',
      '**/.cache/**',
    ],
    cache: false,
    alias: {
      '^swagger-ui-react.*css$': path.resolve(__dirname, './src/ui/next/src/mocks/empty.css'),
    },
  },
})
