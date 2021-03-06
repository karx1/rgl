import React, { useEffect, useRef, useState } from "react";
import axios from "axios";
import Rates from "./Rates";



function Exchanger() {
    let [rates, setRates] = useState({} as Rates);
    let [inputOne, setInputOne] = useState("");
    let [selectedOne, setSelectedOne] = useState("USD");
    let [selectedTwo, setSelectedTwo] = useState("EUR");
    let [result, setResult] = useState("0.00");
    const isFirstRender = useRef(true);

    useEffect(() => {
        const saved = Number(localStorage.getItem("saved"));
        if (Math.floor(Date.now() / 1000) - saved > 86400) {
            // If it's been more tha 24 hours, fetch new rates
            // The timeout is 24 hours because that's how often the exchangerate-api refreshes its data
            axios.get(process.env.API_URL).then(resp => {
                setRates(resp.data);
                localStorage.setItem("saved", Math.floor(Date.now() / 1000).toString()); // convert milliseconds to seconds
                localStorage.setItem("rates", JSON.stringify(resp.data));
            });
        } else {
            // Otherwise, just grab them from localstorage
            const data = JSON.parse(localStorage.getItem("rates"));
            setRates(data);
        }
    }, []);

    function handleSelectedOne(event: any) {
        setSelectedOne(event.target.value);
    }

    function handleInputOne(event: any) {
        setInputOne(event.target.value);
    }

    function handleSelectedTwo(event: any) {
        setSelectedTwo(event.target.value);
    }

    useEffect(() => {
        if (isFirstRender.current) {
            // Do nothing if it's the first render
            // This will ensure this effect will only fire on state updates, not the initial state
            isFirstRender.current = false;
            return;
        }
        let inp = Number(inputOne);
        if (Number.isNaN(inp)) {
            alert("Input must be a number!");
            return;
        }

        let inpCurrency = selectedOne;
        let outCurrency = selectedTwo;

        // First, convert the input to USD
        let exchange_rate = rates.conversion_rates[inpCurrency];
        let usd = inp / exchange_rate;

        // Then, convert the USD to the chosen output currency
        exchange_rate = rates.conversion_rates[outCurrency];
        setResult((usd * exchange_rate).toFixed(2)); // Round to 2 decimal places
    }, [rates, inputOne, selectedOne, selectedTwo]); // Only run this effect if one of these changed

    function handleSwapClick() {
        const temp1 = selectedOne;
        setSelectedOne(selectedTwo);
        setSelectedTwo(temp1);

        const temp2 = inputOne;
        setInputOne(result);
        setResult(temp2);
    }

    if (Object.keys(rates).length) {
        return <>
            <div className="grid-container">
                <input type="text" value={inputOne} placeholder="0" onChange={handleInputOne} />

                <input type="text" value={result} readOnly />
                <select value={selectedOne} onChange={handleSelectedOne}>
                    {/* Map currency names to dropdown values */}
                    {Object.keys(rates.conversion_rates).sort((a, b) => a.localeCompare(b)).map((currency, index) => <option key={index}>{currency}</option>)}
                </select>
                <select value={selectedTwo} onChange={handleSelectedTwo}>
                    {Object.keys(rates.conversion_rates).sort((a, b) => a.localeCompare(b)).map((currency, index) => <option key={index}>{currency}</option>)}
                </select>

            </div>

            <br />
            <div style={{ display: "flex", justifyContent: "center" }}>
                <button onClick={handleSwapClick}>Swap</button>
            </div>
        </>
    } else {
        return <h1>Loading...</h1>
    }
}

export default Exchanger;