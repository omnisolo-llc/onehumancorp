import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': resolve(__dirname, './src/ui/next/src')
    }
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./setupTests.ts'],
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
  },
})
