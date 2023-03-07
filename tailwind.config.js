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
        teal : colors.teal,
        red : colors.red
      },
    },
    variants: {
      extend: {},
    },
    plugins: [ require('@tailwindcss/typography')],

  }