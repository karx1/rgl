function get_token() {
    let params = new URLSearchParams(document.location.search);
    let token = params.get("deck");
    return token;
}
