import React from "react";
import ReactDOM from "react-dom";

import Exchanger from "./Exchanger";

function App() {
    return <>
        <div style={{display: "flex", justifyContent: "center"}}>
            <h1>ConCurrency</h1>
        </div>
        <div className="wrapper">
            <Exchanger />
        </div>
    </>
}

ReactDOM.render(<App />, document.getElementById("root"));