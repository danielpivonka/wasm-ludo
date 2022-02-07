export const copyToClipboard = (text) => {
  try {
    navigator.clipboard.writeText(text);
  } catch (err) {
    console.error(err);
  }
};
