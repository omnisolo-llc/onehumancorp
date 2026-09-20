import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

// API handlers execute in Node, not a WebView. Keep the complete test discovery
// partitioned across environments; no test allowlist and no disabled assertions.
const serverTests = ['src/app/api/**/*.test.ts', 'src/**/*.source.test.ts', 'scripts/**/*.test.mjs'];
const excluded = [
  '**/node_modules/**', '**/dist/**', '**/e2e/**', '**/external/**',
  '**/.next/**', '**/coverage/**',
];

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    projects: [
      {
        extends: true,
        test: {
          name: 'server', environment: 'node',
          include: serverTests, exclude: excluded,
          setupFiles: ['./vitest.server.setup.ts'],
        },
      },
      {
        extends: true,
        test: {
          name: 'browser', environment: 'jsdom',
          exclude: [...excluded, ...serverTests],
          setupFiles: ['./vitest.setup.ts'],
        },
      },
    ],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
    },
    alias: {
      '^swagger-ui-react.*css$': path.resolve(__dirname, './src/mocks/empty.css'),
    },
  },
  resolve: {
    preserveSymlinks: true,
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
})
