import {
  Card,
  CardContent,
  CardActions,
  Button,
  Typography,
  Grid,
} from "@mui/material";
import { Link } from "react-router-dom";

export const Home = () => {
  return (
    <div style={{ padding: 32 }}>
      <Typography variant="h3" gutterBottom>
        Welcome to the Reinforcement Learning Games
      </Typography>
      <Typography variant="subtitle1" gutterBottom>
        Select a game below to get started!
      </Typography>
      <Grid container spacing={4} justifyContent="center" sx={{ marginTop: 2 }}>
        <Grid item xs={12} sm={6} md={4}>
          <Card
            sx={{
              minHeight: 220,
              display: "flex",
              flexDirection: "column",
              justifyContent: "space-between",
              backgroundColor: "#23272b",
              color: "#fff",
            }}
          >
            <CardContent>
              <Typography variant="h5" component="div" gutterBottom>
                Home
              </Typography>
              <Typography variant="body2">
                The main landing page for the Reinforcement Learning Games app.
              </Typography>
            </CardContent>
            <CardActions sx={{ justifyContent: "flex-end" }}>
              <Button
                component={Link}
                to="/"
                size="small"
                color="primary"
                variant="contained"
              >
                Go Home
              </Button>
            </CardActions>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Card
            sx={{
              minHeight: 220,
              display: "flex",
              flexDirection: "column",
              justifyContent: "space-between",
              backgroundColor: "#23272b",
              color: "#fff",
            }}
          >
            <CardContent>
              <Typography variant="h5" component="div" gutterBottom>
                Grid Traversal
              </Typography>
              <Typography variant="body2">
                Play a grid traversal game and see how reinforcement learning
                agents solve mazes and navigate environments.
              </Typography>
            </CardContent>
            <CardActions sx={{ justifyContent: "flex-end" }}>
              <Button
                component={Link}
                to="/GridTraversal"
                size="small"
                color="primary"
                variant="contained"
              >
                Play Grid Traversal
              </Button>
            </CardActions>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <Card
            sx={{
              minHeight: 220,
              display: "flex",
              flexDirection: "column",
              justifyContent: "space-between",
              backgroundColor: "#23272b",
              color: "#fff",
            }}
          >
            <CardContent>
              <Typography variant="h5" component="div" gutterBottom>
                Tic Tac Toe
              </Typography>
              <Typography variant="body2">
                Play Tic Tac Toe against a reinforcement learning agent and see
                how it learns to win!
              </Typography>
            </CardContent>
            <CardActions sx={{ justifyContent: "flex-end" }}>
              <Button
                component={Link}
                to="/TicTacToe"
                size="small"
                color="primary"
                variant="contained"
              >
                Play Tic Tac Toe
              </Button>
            </CardActions>
          </Card>
        </Grid>
      </Grid>
    </div>
  );
};
