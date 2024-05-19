/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ["./templates/**/*.jinja"],
  theme: {
    extend: {
      fontFamily: {
        'sans': ['"IBM Plex Sans"', ...defaultTheme.fontFamily.sans]
      },
      colors: {
        'champagne-pink': '#F2DFCE',
        'old-lace': '#FFF1E0',
        'floral-white': '#FFF9F5',
        'metallic-seaweed': '#0D7680'
      }
    },
  },
  safelist: [
    {
      pattern: /col-(start|end)-[1-5]/
    },
  ]
}

