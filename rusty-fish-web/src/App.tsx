import React, {useEffect, useState} from "react";
import logo from "./logo.svg";
import "./App.css";
import init, {add} from "rusty-fish";
import Game from "./pages/Game";
import {DndContext} from "@dnd-kit/core";

function App() {
  const [ans, setAns] = useState(0);
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    init().then(() => {
      setLoaded(true);
    });
  }, []);
  return <div className="">{loaded ? <Game /> : <div>Loading...</div>}</div>;
}

export default App;
