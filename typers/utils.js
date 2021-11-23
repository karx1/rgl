function processText(value) {
    const value_array = value.split('');
    let errors = 0;

    let quoteSpanArray = document.getElementById("quote").querySelectorAll('span');
    quoteSpanArray.forEach((char, index) => {
        let typedChar = value_array[index];

        switch (typedChar) {
            case undefined:
                char.classList.remove('correct');
                char.classList.remove('incorrect');
                break;
            case char.innerText:
                char.classList.add('correct');
                char.classList.remove('incorrect');
                break;
            default:
                char.classList.remove('correct');
                char.classList.add('incorrect');
                errors++;
                break;
        }
    });

    return errors;
}

function reset() {
    let quoteSpanArray = document.getElementById("quote").querySelectorAll('span');
    quoteSpanArray.forEach((char, _) => {
        char.classList.remove('correct');
        char.classList.remove('incorrect');
    })
}

function reload(event) {
    window.location.reload();
}