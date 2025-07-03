import { useEffect, useState } from "react";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";
import Typography from "@mui/material/Typography";

type Direction = "Up" | "Down" | "Left" | "Right";
type QTable = Array<Array<Direction>>;
type Inner = {
  position: Array<number>
}

export const QTable = () => {
  const [data, setData] = useState<QTable | null>(null);

  const fetchData = async () => {
    try {
      const response = await fetch("http://localhost:8000/predict_all");
      if (!response.ok) {
        throw new Error("Network response was not ok");
      }
      let result: Array<[Inner, Direction]> = await response.json();
      console.log(result);
      let qTable: Array<Array<Direction>> = [];
      let temp: Array<Direction> = [];
      let prevRow = 0;
      
      for (let i = 0; i < result.length; i++) {
        if (i === result.length - 1) {
          temp.push(result[i][1]);
          qTable.push(temp);
          break;
        }
        if (result[i][0].position[0] != prevRow) {
          console.log("This was hit at", prevRow, result[i][0].position[0]);
          qTable.push(temp);
          temp = [];
          prevRow = result[i][0].position[0];
        }
        temp.push(result[i][1]);
      }
      console.log("Qtable formatting lul", qTable);
      setData(qTable);
    } catch (error) {
      console.error("There was a problem with the fetch operation:", error);
    }
  };

  useEffect(() => {
    // Fetch data from server on page load.
    fetchData();
  }, []);

  const renderDirection = (direction: Direction) => {
    switch (direction) {
      case "Up":
        return "⬆️";
      case "Down":
        return "⬇️";
      case "Left":
        return "⬅️";
      case "Right":
        return "➡️";
      default:
        return "";
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
          Loading Q-Table...
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
      {data && (
        <>
          <Typography variant="h4" gutterBottom sx={{ color: "white" }}>
            Q-Table
          </Typography>
          <TableContainer
            component={Paper}
            sx={{
              borderRadius: "8px",
              boxShadow: 3,
              marginBottom: "20px",
            }}
          >
            <Table
              sx={{ minWidth: 750, backgroundColor: "#424242" }}
              aria-label="q-table"
            >
              <TableBody>
                {data.length > 0 &&
                  (() => {
                    const midRow = Math.floor(data.length / 2);
                    const midCol = Math.floor(data[0].length / 2);
                    return data.map((row, rowIndex) => (
                      <TableRow key={rowIndex}>
                        {row.map((direction, colIndex) => {
                          const isCheckpoint =
                            rowIndex === midRow && colIndex === midCol;
                          return (
                            <TableCell
                              key={colIndex}
                              align="center"
                              sx={{
                                color: "white",
                                border: "1px solid white",
                                backgroundColor: isCheckpoint
                                  ? "#4caf50"
                                  : "inherit",
                                fontWeight: isCheckpoint ? "bold" : "normal",
                                fontSize: "20px",
                                padding: "16px",
                              }}
                            >
                              {isCheckpoint ? "🏁" : renderDirection(direction)}
                            </TableCell>
                          );
                        })}
                      </TableRow>
                    ));
                  })()}
              </TableBody>
            </Table>
          </TableContainer>
        </>
      )}
    </div>
  );
};
