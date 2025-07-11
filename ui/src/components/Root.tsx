import { Box } from "@mui/material";
import { Outlet } from "react-router-dom";
import { Navbar } from "./navbar";

export const Root = () => {
  return (
    <Box>
      <Navbar />
      <Box
        sx={{
          width: "100%",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          mt: "100px",
        }}
      >
        <Outlet />
      </Box>
    </Box>
  );
};
