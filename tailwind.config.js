/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: 'jit',
    purge: [
      "src/**/*.rs"
    ],
    theme: {
      extend: {},
    },
    variants: {
      extend: {},
    },
    plugins: [ require('@tailwindcss/typography')],

  }