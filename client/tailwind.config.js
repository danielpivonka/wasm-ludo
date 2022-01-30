const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      colors: {
        primary: colors.indigo,
      },
      gridTemplateColumns: {
        board: "40% 20% 40%",
        game: "25% 50% 25%",
      },
      gridTemplateRows: {
        board: "40% 20% 40%",
      },
      padding: {
        full: "100%",
        "1/2": "50%",
      },
      borderWidth: {
        "1/2": "50%",
        full: "100%",
      },
    },
  },
  plugins: [],
};
