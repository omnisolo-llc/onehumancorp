/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
  transform: {
    '^.+\\.(ts|tsx)$': ['ts-jest', {
      tsconfig: {
        target: 'es2022',
        moduleResolution: 'node16',
        jsx: 'react',
        ignoreDeprecations: '6.0',
        rootDir: '.'
      }
    }]
  }
};
