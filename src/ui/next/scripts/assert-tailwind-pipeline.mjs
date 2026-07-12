import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { pathToFileURL } from 'node:url'

const dependencySections = [
  'dependencies',
  'devDependencies',
  'optionalDependencies',
  'peerDependencies',
]

export function assertTailwindPipeline(packageJson, postcssConfig) {
  assert.match(
    packageJson.devDependencies?.tailwindcss ?? '',
    /^\^?3\./,
    'Tailwind 3 must be installed in devDependencies',
  )
  for (const section of dependencySections) {
    assert.equal(
      packageJson[section]?.['@tailwindcss/postcss'],
      undefined,
      `Tailwind 4 PostCSS must not be installed in ${section}`,
    )
  }
  assert.equal(
    packageJson.postcss,
    undefined,
    'package.json must not compete with postcss.config.mjs',
  )
  assert.ok(
    postcssConfig !== null &&
      typeof postcssConfig === 'object' &&
      !Array.isArray(postcssConfig),
    'postcss.config.mjs must default-export a configuration object',
  )
  assert.ok(
    postcssConfig.plugins !== null &&
      typeof postcssConfig.plugins === 'object' &&
      !Array.isArray(postcssConfig.plugins),
    'postcss.config.mjs default export must contain a plugins object',
  )
  assert.ok(
    Object.hasOwn(postcssConfig.plugins, 'tailwindcss'),
    'postcss.config.mjs plugins must include tailwindcss',
  )
  assert.ok(
    Object.hasOwn(postcssConfig.plugins, 'autoprefixer'),
    'postcss.config.mjs plugins must include autoprefixer',
  )
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const packageJson = JSON.parse(
    await readFile(new URL('../package.json', import.meta.url), 'utf8'),
  )
  const { default: postcssConfig } = await import(
    new URL('../postcss.config.mjs', import.meta.url)
  )

  assertTailwindPipeline(packageJson, postcssConfig)
  process.stdout.write('Tailwind/PostCSS pipeline is coherent.\n')
}
