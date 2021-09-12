import React from "react";
import ReactDOM from "react-dom";
import App from "./App";
import { initContract } from "./utils";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";

//* This is a workaround to allow fields to be attached to the global window, which are used
//* with NEAR js API
declare global {
  interface Window {
    walletConnection: any;
    nearInitPromise: any;
    accountId: string;
    contract: any;
  }
}

window.nearInitPromise = initContract()
  .then(() => {
    ReactDOM.render(
      <DndProvider backend={HTML5Backend}>
        <App />
      </DndProvider>,
      document.querySelector("#root")
    );
  })
  .catch(console.error);
