/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'victoria': {
          '50': '#f4f3fb',
          '100': '#e6e4f5',
          '200': '#d2cfee',
          '300': '#b2ade3',
          '400': '#8f86d4',
          '500': '#7a69c8',
          '600': '#6f56ba',
          '700': '#674baa',
          '800': '#5b438d',
          '900': '#49396f',
          '950': '#302645',
        }
      }
    }
  },
  plugins: []
};
