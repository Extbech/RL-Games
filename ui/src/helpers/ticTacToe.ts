import type { TicTacAction, TicTacToeBoard } from "../types/api";

export const initTicTacToeBoard = (): TicTacToeBoard => {
    return {
        cells: Array(3)
        .fill(null)
        .map(() => Array(3).fill("Empty")),
        player: "X",
        done: false,
    };
}

export const predictTicTacToe = async (board: TicTacToeBoard): Promise<TicTacAction> => {
    // Make a GET request to the server to predict the next move
    const response = await fetch(`http://localhost:8000/predict/TicTacToe?state=${JSON.stringify(board)}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        }
    });
    if (!response.ok) {
        throw new Error("Network response was not ok");
    }
    const result = await response.json();
    return result;
};