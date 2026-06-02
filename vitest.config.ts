import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./setupTests.ts'],
    exclude: ['**/node_modules/**', '**/e2e/**', '**/verification_tests/**'],
  },
  resolve: {
    alias: {
      'next/server': path.resolve(__dirname, 'src/ui/next/__mocks__/next/server.ts'),
      'next/navigation': path.resolve(__dirname, 'src/ui/next/__mocks__/next/navigation.ts'),
      'next/link': path.resolve(__dirname, 'src/ui/next/__mocks__/next/link.ts'),
      'dompurify': path.resolve(__dirname, 'src/ui/next/__mocks__/dompurify.ts'),
      'swagger-ui-react/swagger-ui.css': path.resolve(__dirname, 'src/ui/next/__mocks__/swagger-ui-react.css.ts'),
      'swagger-ui-react': path.resolve(__dirname, 'src/ui/next/__mocks__/swagger-ui-react.ts')
    }
  }
})
