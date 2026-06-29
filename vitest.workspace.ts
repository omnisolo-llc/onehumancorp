import { defineWorkspace } from 'vitest/config';

export default defineWorkspace([
  {
    test: {
      name: 'unit',
      environment: 'jsdom',
      setupFiles: ['./setupTests.ts'],
    },
    esbuild: {
      jsx: 'react-jsx',
    }
  }
]);
