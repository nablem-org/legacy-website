function loadIn() {
    setTimeout(function () {
        var loadNextVideo = document.getElementById("loadNext");

        fadeElement("Load Video", "loadNext", 1, 0, 0.1, 50, false);

        setTimeout(function () {
            loadNextVideo.classList.add("invisible");
            //
            var body = document.getElementById("body");
            body.classList.remove("invisible");
            //
            fadeElement("Body", "body", 0, 1, 0.1, 50, true);
        }, 500);
    }, 3000);
}

function checkIfLoad() {
    // Checks if a cookie called load has been stored in the browser
    // If it does, doesn't load the transition.
    const cookieValue = document.cookie
        .split('; ')
        .find((row) => row.startsWith('load='))
        ?.split('=')[1];

    if (cookieValue !== undefined) {
        var body = document.getElementById("body");
        var loadNextVideo = document.getElementById("loadNext");
        loadNextVideo.classList.add("invisible");
        body.classList.remove("invisible");
    } else {
        // Otherwise, adds the cookie
        console.log("Added Load Cookie!");
        document.cookie = "load=true; SameSite=strict; Secure";
        loadIn();
    }
}

window.onload = async function () {
    checkIfLoad();
    speaker(fadeAudio('../resources/1_1.mp3'));

    // Fades out image by 0.025 every 100ms from 1 to 0
    fadeElement("Image", "banner", 1, 0, 0.025, 100, false);

    // Fades in video by 0.05 every 200ms from 0 to 1
    fadeElement("Video", "video", 0, 1, 0.05, 200, true);
}