window.onload = async function () {
    fadeAudio('./resources/1_1.mp3');

    // Fades out image by 0.025 every 100ms from 1 to 0
    fadeElement("Image", "banner", 1, 0, 0.025, 100, false);

    // Fades in video by 0.05 every 200ms from 0 to 1
    fadeElement("Video", "video", 0, 1, 0.05, 200, true);

    const video = document.getElementById("video");
    try {
        await video.play();
    } catch (err) {
        console.log(err);
    }
}