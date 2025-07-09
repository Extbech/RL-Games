export type EnvironmentType = "TicTacToe" | "Grid";

export type CellState = "Empty" | "X" | "O";

export type TicTacPlayer = "X" | "O";

export type TicTacToeBoard = {
    cells: Array<Array<CellState>>;
    currentPlayer: TicTacPlayer;
}

export type TicTacAction = [number, number];