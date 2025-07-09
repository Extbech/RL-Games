import "./App.css";
import { TicTacToe } from "./components/ticTacToe";

function App() {
  return (
    <div>
      <h1>Reinforcement Learning</h1>
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <TicTacToe />
      </div>
    </div>
  );
}

export default App;
