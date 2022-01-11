function setupDownload(md) {
	const downloadButton = document.getElementById("save");
	downloadButton.setAttribute('href', "data:application/octet-stream;charset=utf-8," + encodeURIComponent(md));
	downloadButton.setAttribute('download', 'markrs.md');
}
