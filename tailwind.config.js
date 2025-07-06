/** @type {import('tailwindcss').Config} */
const colors = require('tailwindcss/colors')
module.exports = {
  mode: 'jit',
  darkMode: 'class',
  purge: [
    "src/**/*.rs"
  ],
  theme: {
    extend: {
      typography: (theme) => ({
        DEFAULT: {
          css: {
            'code': {
              backgroundColor: 'rgba(0, 0, 0, 0.1)',
              opacity: 0.5,
              padding: '0.2rem 0.4rem',
              borderRadius: '0.25rem',
              fontWeight: 'normal',
              fontSize: '0.875rem',
            },
            'code::before': { content: 'none' },
            'code::after': { content: 'none' },
          },
        },
      }),
    },
    colors: {
      transparent: 'transparent',
      current: 'currentColor',
      black: colors.black,
      white: colors.white,
      gray: colors.gray,
      emerald: colors.emerald,
      indigo: colors.indigo,
      yellow: colors.yellow,
      teal: colors.teal,
      red: colors.red,
      cyan: colors.cyan,
      sky: colors.sky
    },
  },
  variants: {
    extend: {},
  },
  plugins: [require('@tailwindcss/typography')],

}