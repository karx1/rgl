import React from "react";
import ReactDOM from "react-dom";

import Exchanger from "./Exchanger";

function App() {
    return <>
        <h1>ConCurrency</h1>
        <Exchanger />
    </>
}

ReactDOM.render(<App />, document.getElementById("root"));