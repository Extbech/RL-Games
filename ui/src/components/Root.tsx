import { Box } from "@mui/material";
import { Outlet } from "react-router-dom";
import { Navbar } from "./navbar";

export const Root = () => {
  return (
    <Box>
      <Navbar />
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          ml: "200px",
          mt: "100px",
        }}
      >
        <Outlet />
      </Box>
    </Box>
  );
};
