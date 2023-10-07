/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        "blue-1": "#2D2D37",
        "blue-2": "#212125",
      },
      transitionDelay: {
        "30": "30ms"
      }
    },
  },
  plugins: [],
};
