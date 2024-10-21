import React, {useState} from "react";
import {add, mult, greet} from "rusty-fish";

const Game = () => {
  return (
    <div className="w-full flex items-center justify-center pt-20">
      <div className="grid grid-cols-8 gap-0 ">
        {Array.from({length: 64}, (_, i) => (
          <div
            key={i}
            className={`h-[10vh] w-[10vh] ${
              i % 2 === Math.floor(i / 8) % 2 ? "bg-[#f4dcb4]" : "bg-[#b58863]"
            }`}
          >
            <img src="/pieces/b-b.png" />
          </div>
        ))}
      </div>
    </div>
  );
};

export default Game;
