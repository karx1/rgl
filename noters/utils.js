function listLocalStorageKeys() {
    const arr = Object.keys(localStorage);
    arr.sort((a, b) => {
        const a_num = Number(a);
        const b_num = Number(b);

        if (a_num > b_num) {
            return -1;
        } else if (b_num > a_num) {
            return 1;
        }

        return 0;
    });

    return arr;
}

function getCurrentTimeMillis() {
    return BigInt(Date.now());
}

function timeHR(millis) {
    return new Date(Number(millis)).toLocaleString(); // We have to convert bigint to number explicitly
}