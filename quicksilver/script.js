function get_token() {
    let params = new URLSearchParams(document.location.search);
    let token = params.get("deck");
    return token;
}

function get_edit_token() {
    let params = new URLSearchParams(document.location.search);
    let token = params.get("edit");
    return token;
}

function set_location(l) {
    window.location = l; 
}
