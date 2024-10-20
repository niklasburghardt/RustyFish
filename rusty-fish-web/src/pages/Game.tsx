import React from "react";
import {add, mult, greet} from "rusty-fish";

const Game = () => {
  const [num1, setNum1] = React.useState(0);
  const [num2, setNum2] = React.useState(0);
  const [result, setResult] = React.useState(0);

  return (
    <div>
      <input
        type="number"
        value={num1}
        onChange={(e) => setNum1(parseInt(e.target.value))}
      />
      <input
        type="number"
        value={num2}
        onChange={(e) => setNum2(parseInt(e.target.value))}
      />
      <button
        onClick={() => {
          let result = mult(num1, num2);
          setResult(result);
          greet();
        }}
      >
        Add
      </button>
      <div>Result: {result}</div>
    </div>
  );
};

export default Game;
