/**
 * @jest-environment jsdom
 */

import axios from "axios";
import Rates from "./Rates";

test("API request works", async () => {
    const resp = await axios(process.env.API_URL);
    const data = resp.data as Rates;

    expect(data.base_code).toMatch("USD");
}, 3600000); // 1 hour timeout lol