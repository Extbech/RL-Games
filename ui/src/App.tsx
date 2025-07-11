import { ThemeProvider, CssBaseline } from "@mui/material";
import { lightTheme, darkTheme } from "./theme";
import { router } from "./router";
import { RouterProvider } from "react-router-dom";
import { useAppSelector } from "./hooks/useTheme";

export const App = () => {
  const mode = useAppSelector((state) => state.theme.mode);

  return (
    <ThemeProvider theme={mode === "light" ? lightTheme : darkTheme}>
      <CssBaseline />
      <RouterProvider router={router} />
    </ThemeProvider>
  );
};
