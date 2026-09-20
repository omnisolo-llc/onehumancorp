import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import globals from 'globals';

export default tseslint.config(
  {
    ignores: [
      '**/node_modules/**', '**/target/**', '**/.next/**', '**/dist/**',
      '**/coverage/**', '**/playwright-report/**', '**/test-results/**',
      '**/bazel-*/**', '**/next_out/**', '**/gen/**', '**/.cache/**',
      'site/**',
      '**/*.tsbuildinfo', '**/next-env.d.ts',
    ],
  },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['**/*.{js,cjs,mjs,ts,tsx}'],
    languageOptions: { globals: { ...globals.node, ...globals.browser } },
  },
  {
    files: ['**/*.{test,spec}.{js,mjs,ts,tsx}', '**/vitest.setup.{ts,tsx}'],
    languageOptions: { globals: globals.vitest },
  },
  {
    files: ['**/*.{js,cjs,mjs}'],
    rules: { '@typescript-eslint/no-require-imports': 'off' },
  },
);
