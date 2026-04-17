/** @type {import('jest').Config} */
module.exports = {
  testEnvironment: 'jsdom',
  setupFilesAfterFramework: [],
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
  transform: {
    '^.+\\.(ts|tsx)$': [
      'ts-jest',
      {
        tsconfig: {
          jsx: 'react',
          esModuleInterop: true,
          allowSyntheticDefaultImports: true,
          isolatedModules: true,
        },
        diagnostics: false,
      },
    ],
  },
  moduleNameMapper: {
    '\\.(css|less|scss|sass)$': '<rootDir>/test/__mocks__/styleMock.cjs',
  },
  testMatch: ['<rootDir>/test/**/*.test.{ts,tsx}', '<rootDir>/test/**/*.spec.{ts,tsx}'],
  testPathIgnorePatterns: ['/node_modules/', '/e2e/'],
};
