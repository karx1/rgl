function reset_cards() {
	const cards = document.querySelectorAll(".flip.disabled");

	cards.forEach(card => {
		card.classList.toggle("flip");
		card.classList.toggle("disabled");
	});
}
