import { Box } from "@mui/material";
import { TicTacQ } from "./TicTacQ";
import { TicTacDQN } from "./TicTacDQN";

export const TicTacToe = () => {
  return (
    <Box
      style={{
        width: "60%",
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        justifyContent: "space-around",
        padding: "24px",
      }}
    >
      <TicTacQ />
      <TicTacDQN />
    </Box>
  );
};
