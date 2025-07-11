/// Tic Tac Toe
export type EnvironmentType = "TicTacToe" | "Grid";

export type CellState = "Empty" | "X" | "O";

export type TicTacPlayer = "X" | "O";

export type TicTacToeBoard = {
    cells: Array<Array<CellState>>;
    player: TicTacPlayer;
    done: boolean;
}

export type TicTacAction = [number, number];

/// Grid
export type Direction = "Up" | "Down" | "Left" | "Right";
export type GridBoard = Array<Array<Direction>>;
