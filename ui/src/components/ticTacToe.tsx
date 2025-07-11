import { useState } from "react";
import Typography from "@mui/material/Typography";
import type { CellState, TicTacToeBoard } from "../types/api";
import { initTicTacToeBoard, predictTicTacToe } from "../helpers/ticTacToe";

export const TicTacToe = () => {
  const [data, setData] = useState<TicTacToeBoard>(initTicTacToeBoard());

  const tileClickHandler = async (
    rowIndex: number,
    cellIndex: number,
    cell: CellState
  ) => {
    if (cell === "Empty") {
      const newCells = data.cells.map((r, rIndex) =>
        r.map((c, cIndex) =>
          rIndex === rowIndex && cIndex === cellIndex ? data.current_player : c
        )
      );
      const newData: TicTacToeBoard = {
        cells: newCells,
        current_player: data.current_player === "X" ? "O" : "X",
      };
      setData(newData);
      console.log("New Board State:", newData);
      const agent_prediction = await predictTicTacToe(newData);
      console.log("Agent prediction: ", agent_prediction);
    }
  };
  if (!data) {
    return (
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Typography variant="h6" sx={{ color: "white" }}>
          Loading data
        </Typography>
      </div>
    );
  }
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        padding: "16px",
      }}
    >
      <Typography variant="h4" sx={{ color: "white", marginBottom: "16px" }}>
        Tic Tac Toe
      </Typography>
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(3, 1fr)",
          gap: "8px",
        }}
      >
        {data.cells.map((row, rowIndex) =>
          row.map((cell, cellIndex) => (
            <div
              key={`${rowIndex}-${cellIndex}`}
              style={{
                width: "100px",
                height: "100px",
                display: "flex",
                justifyContent: "center",
                alignItems: "center",
                backgroundColor: "#282c34",
                color: "white",
                fontSize: "24px",
                borderRadius: "4px",
                cursor: "pointer",
              }}
              onClick={async () =>
                await tileClickHandler(rowIndex, cellIndex, cell)
              }
            >
              {cell}
            </div>
          ))
        )}
      </div>
      <button
        style={{
          marginTop: "16px",
          padding: "8px 16px",
          fontSize: "16px",
          cursor: "pointer",
        }}
        onClick={() => {
          setData(initTicTacToeBoard());
        }}
      >
        Reset Game
      </button>
    </div>
  );
};
