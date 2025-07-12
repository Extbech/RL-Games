import { AppBar, Toolbar, Typography, Button, Stack } from "@mui/material";
import { Link } from "react-router-dom";

export const Navbar = () => {
  return (
    <AppBar
      position="fixed"
      sx={{ zIndex: (theme) => theme.zIndex.drawer + 1, height: "64px" }}
    >
      <Toolbar>
        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          Reinforcement Learning
        </Typography>
        <Stack direction="row" spacing={2}>
          <Button
            variant="contained"
            component={Link}
            to="/"
            sx={{
              backgroundColor: "#ff9800",
              color: "#fff",
              boxShadow: 3,
              fontWeight: "bold",
              transition: "transform 0.2s, background 0.2s",
              "&:hover": {
                backgroundColor: "#fb8c00",
                transform: "scale(1.08)",
                boxShadow: 6,
              },
            }}
          >
            Home
          </Button>
          <Button
            variant="contained"
            component={Link}
            to="/GridTraversal"
            sx={{
              backgroundColor: "#ff9800",
              color: "#fff",
              boxShadow: 3,
              fontWeight: "bold",
              transition: "transform 0.2s, background 0.2s",
              "&:hover": {
                backgroundColor: "#fb8c00",
                transform: "scale(1.08)",
                boxShadow: 6,
              },
            }}
          >
            Grid Traversal
          </Button>
          <Button
            variant="contained"
            component={Link}
            to="/TicTacToe"
            sx={{
              backgroundColor: "#ff9800",
              color: "#fff",
              boxShadow: 3,
              fontWeight: "bold",
              transition: "transform 0.2s, background 0.2s",
              "&:hover": {
                backgroundColor: "#fb8c00",
                transform: "scale(1.08)",
                boxShadow: 6,
              },
            }}
          >
            Tic Tac Toe
          </Button>
        </Stack>
      </Toolbar>
    </AppBar>
  );
};
