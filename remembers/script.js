function toggle_cards() {
    console.log("toggle_cards() called");
    const cards = document.querySelectorAll(".card");

    function flipCard() {
        this.classList.toggle("flip");
    }

    cards.forEach(card => card.addEventListener("click", flipCard));
}