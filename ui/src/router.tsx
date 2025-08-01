import { createBrowserRouter } from "react-router-dom";
import "./index.css";
import { Root } from "./components/Root";
import { GridTraversal } from "./components/GridTraversal";
import { NotFound } from "./components/NotFound";
import { TicTacToe } from "./components/ticTacToe";
import { Home } from "./components/home";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <NotFound />,
    children: [
      {
        path: "/",
        element: <Home />,
      },
      {
        path: "/GridTraversal",
        element: <GridTraversal />,
      },
      {
        path: "/TicTacToe",
        element: <TicTacToe />,
      },
    ],
  },
]);
