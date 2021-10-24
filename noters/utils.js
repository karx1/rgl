function listLocalStorageKeys() {
    return Object.keys(localStorage);
}

function getCurrentTimeMillis() {
    return BigInt(Date.now());
}

function timeHR(millis) {
    return new Date(Number(millis)).toLocaleString(); // We have to convert bigint to number explicitly
}