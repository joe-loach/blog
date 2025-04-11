const isDark = () => {
  return (
    localStorage.theme === "dark" ||
    (!("theme" in localStorage) &&
      window.matchMedia("(prefers-color-scheme: dark)").matches)
  );
};

const updateTheme = () => {
  document.documentElement.classList.toggle("dark", isDark());
};

const setTheme = (theme) => {
  localStorage.theme = theme;
  updateTheme();
};

const toggleTheme = () => {
  if (isDark()) {
    setTheme("");
  } else {
    setTheme("dark");
  }
};

updateTheme();