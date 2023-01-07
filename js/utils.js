// increment <= 0.65 (fade in)
// imageOpacity >= 0 (fade out)

function fade(name, to, increment, max, ms, fadeIn) {
    var interval = setInterval(loop, ms);

    var key;
    switch (name) {
        case "Volume":
            key = "volume"; break;
        default:
            key = "opacity";
            name = `${name} ${key}`;
    }

    function loop() {
        var check;
        if (fadeIn) {
            var newValue = +to[key] + increment;
            check = to[key] <= max;
            to[key] = newValue;
        } else {
            check = to[key] >= max;
            to[key] -= increment; 
        }

        if (key == "opacity") {
            to["filter"] = 'alpha(opacity=' + to[key] * 100 + ")";
        }

        if (check) {
            console.log(`${name} is ${to[key]}`);
        } else {
            // Stop fade
            clearInterval(interval);
            interval = false;
        }
    }

    return interval;
}

function fadeElement(name, id, initialOpacity, max, increment, ms, fadeIn) {
    var element = document.getElementById(id);
    console.log(element.style.opacity);
    element.style.opacity = initialOpacity;
    fade(name, element.style, increment, max, ms, fadeIn);
}

function fadeAudio(fileName) {
    var audio = new Audio(fileName);
    audio.loop = true;
    audio.volume = 0;

    // Increments in volume by 0.01 every 200ms up to 0.65
    fade("Volume", audio, 0.01, 0.65, 200, true); audio.play(); 

    return audio
}

function speaker(audio) {
    var speaker = document.getElementById("speaker");
    var noSpeakerIcon = document.getElementById("nospeak");
    var speakerIcon = document.getElementById("speak");

    speaker.onclick = function () {
        if (!audio.paused) {
            speakerIcon.classList.add("invisible");
            noSpeakerIcon.classList.remove("invisible");
            audio.volume = 0;
            audio.pause();
        } else {
            noSpeakerIcon.classList.add("invisible");
            speakerIcon.classList.remove("invisible");
            audio.play();
        }
    }
}