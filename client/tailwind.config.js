const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      colors: {
        primary: colors.indigo,
      },
    },
  },
  plugins: [],
};
