import { AppBar, Toolbar, Typography, Button, IconButton } from "@mui/material";
import { Link } from "react-router-dom";
import { Brightness4, Brightness7 } from "@mui/icons-material";
import { useAppSelector, useAppDispatch } from "../hooks/useTheme";
import { toggleTheme } from "../store/themeSlice";

export const Navbar = () => {
  const mode = useAppSelector((state) => state.theme.mode);
  const dispatch = useAppDispatch();

  const handleToggleTheme = () => {
    dispatch(toggleTheme());
  };

  return (
    <AppBar
      position="fixed"
      sx={{ zIndex: (theme) => theme.zIndex.drawer + 1, height: "64px" }}
    >
      <Toolbar>
        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          Reinforcement Learning
        </Typography>
        <Button color="inherit" component={Link} to="/IntelliFit">
          Home
        </Button>
        <Button color="inherit" component={Link} to="/IntelliFit/services">
          Grid
        </Button>
        <Button color="inherit" component={Link} to="/IntelliFit/login">
          Tic Tac
        </Button>
        <IconButton edge="end" color="inherit" onClick={handleToggleTheme}>
          {mode === "light" ? <Brightness4 /> : <Brightness7 />}
        </IconButton>
      </Toolbar>
    </AppBar>
  );
};
