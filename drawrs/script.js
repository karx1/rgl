function test(str) {
    console.log(str);
}

function getCanvas() {
    return document.getElementById("canvas");
}

function getContext() {
    return getCanvas().getContext("2d");
}

function getClientRect() {
    return getCanvas().getBoundingClientRect();
}

function draw(x, y) {
    const context = getContext();

    context.fillStyle = "#000000"; // TODO: Add ability to set color
    context.fillRect(Number(x), Number(y), 4, 4);
}

function getWidth() {
    return getCanvas().width;
}

function getHeight() {
    return getCanvas().height;
}

function clear() {
    const context = getContext();
    const canvas = context.canvas;

    // Store the current transformation
    context.save();

    // Set a default transformation temporarily
    context.setTransform(1, 0, 0, 1, 0, 0);
    context.clearRect(0, 0, canvas.width, canvas.height);

    // Restore saved transformation
    context.restore();
}