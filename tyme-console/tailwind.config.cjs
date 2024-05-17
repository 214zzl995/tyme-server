/** @type {import('tailwindcss').Config}*/
const { withMaterialColors } = require('tailwind-material-colors');

const config = {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
      },
      gridTemplateRows: {
        '12': 'repeat(12, minmax(0, 1fr))',
        'layout': '200px minmax(900px, 1fr) 100px',
      },
      gridRowStart: {
        '7': '7',
        '8': '8',
        '9': '9',
        '10': '10',
        '11': '11',
        '12': '12',
        '13': '13',
      },
      gridRow: {
        'span-8': 'span 8 / span 8',
        'span-9': 'span 9 / span 9',
        'span-10': 'span 10 / span 10',
        'span-11': 'span 11 / span 11',
        'span-12': 'span 12 / span 12',
        'span-16': 'span 16 / span 16',
      },
      height: {
        'chat': 'calc(100vh - 7rem)',
        'main': 'calc(100vh - 4rem)',
        'md-main': 'calc(100vh - 5rem)',
      },
    }
  }
};

// https://tailwind-material-colors-docs.vercel.app/
module.exports = withMaterialColors(config, {
  primary: '#415F91',
  secondary: '#565F71',
  tertiary: '#705575',
  error: "#BA1A1A",
  background: "#F9F9FF",
},
  {
    scheme: 'content',
    contrast: 0,
  });
