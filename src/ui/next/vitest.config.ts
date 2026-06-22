import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./setupTests.ts'],
    exclude: ['**/node_modules/**', '**/dist/**', '**/e2e/**', '**/playwright/**', '**/.next/**'],
    alias: {
      '@': resolve(__dirname, './src'),
    },
    // The following fixes the Object testPath issue on some vitest + jsdom versions
    onConsoleLog: () => {},
  },
})
