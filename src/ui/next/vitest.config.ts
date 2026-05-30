import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import * as path from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./vitest.setup.ts'],
    exclude: ['**/node_modules/**', '**/e2e/**', '**/*.spec.ts'],
    alias: {
      'next/server': path.resolve(__dirname, './__mocks__/next/server.js'),
      'next/link': path.resolve(__dirname, './__mocks__/next/link.js'),
    }
  },
  resolve: {
    alias: {
      'next/server': path.resolve(__dirname, './__mocks__/next/server.js'),
      'next/link': path.resolve(__dirname, './__mocks__/next/link.js'),
    }
  }
})
