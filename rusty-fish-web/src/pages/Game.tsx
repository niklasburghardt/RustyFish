import React, { useState } from "react";
import {add, mult, greet, find_largest_prime} from "rusty-fish";

const Game = () => {
  const [num1, setNum1] = React.useState(0);
  const [num2, setNum2] = React.useState(0);
  const [result, setResult] = React.useState(0);

  const [finished, setFinished] = useState(true);

  const countToBillion = () => {
    console.log("Counting to a billion...");
    let i = 0;
    for (i = 0; i < 10_000_000_000; i++) {
      i += 1;
    }
    console.log("Finished counting to a billion");
  }

  const findPrime = () => {
    console.log("Finding largest prime...");
    let prime = find_largest_prime(1_000_000);
    console.log("Largest prime is: ", prime);
  }

  return (
    <div>
      <button onClick={countToBillion}>Typescript</button>
      <button onClick={findPrime}>Rust</button>
      <button onClick={greet}>Greet</button>
   </div>
  );
};

export default Game;
