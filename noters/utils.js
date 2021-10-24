function listLocalStorageKeys() {
    return Object.keys(localStorage);
}

function getCurrentTimeMillis() {
    return Date.now();
}

function timeHR(millis) {
    return new Date(millis).toLocaleString();
}