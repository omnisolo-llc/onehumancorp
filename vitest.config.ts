import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ["./setupTests.ts"],
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
  },
})
