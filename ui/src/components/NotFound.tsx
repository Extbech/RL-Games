import { Box, Button, Typography } from "@mui/material";
import { Link } from "react-router-dom";
export const NotFound = () => {
  return (
    <Box
      sx={{
        height: "90vh",
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Typography variant="h3" gutterBottom>
        Oops!
      </Typography>
      <Typography variant="h5" gutterBottom>
        Sorry, an unexpected error has occured.
      </Typography>
      <Typography variant="h6" sx={{ mb: 5 }}>
        Not Found
      </Typography>
      <Button variant="contained" component={Link} to="/">
        Return Home
      </Button>
    </Box>
  );
};
