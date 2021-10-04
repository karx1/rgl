import React, { useEffect, useState } from "react";
import axios from "axios";
import Rates from "./Rates";



function Exchanger() {
    let [rates, setRates] = useState({} as Rates);
    let [inputOne, setInputOne] = useState("0");
    let [selectedOne, setSelectedOne] = useState("USD");
    let [selectedTwo, setSelectedTwo] = useState("EUR");
    let [result, setResult] = useState("0.00");

    useEffect(() => {
        let saved = Number(localStorage.getItem("saved"));
        if (Math.floor(Date.now() / 1000) - saved > 3600) {
            axios.get(process.env.API_URL).then(resp => {
                setRates(resp.data);
                localStorage.setItem("saved", Math.floor(Date.now() / 1000).toString());
                localStorage.setItem("rates", JSON.stringify(resp.data));
            });
        } else {
            let data = JSON.parse(localStorage.getItem("rates"));
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

    function handleConvertClick() {
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
    }

    if (Object.keys(rates).length) {
        return <>
            <div className="grid-container">
                <input type="text" defaultValue={inputOne} onChange={handleInputOne} />
                <input type="text" value={result} readOnly />
                <select defaultValue={selectedOne} onChange={handleSelectedOne}>
                    {Object.keys(rates.conversion_rates).map((currency, index) => <option key={index}>{currency}</option>)}
                </select>
                <select defaultValue={selectedTwo} onChange={handleSelectedTwo}>
                    {Object.keys(rates.conversion_rates).map((currency, index) => <option key={index}>{currency}</option>)}
                </select>

            </div>

            <br />
            <div style={{ display: "flex", justifyContent: "center" }}>
                <button onClick={handleConvertClick}>Convert!</button>
            </div>
        </>
    } else {
        return <h1>Loading...</h1>
    }
}

export default Exchanger;