import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  safelist: ["bg-white/50", "bg-white/60", "bg-white/65"],
  theme: {
    extend: {},
  },
  plugins: [],
}
export default config
