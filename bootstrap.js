import init from "./pkg/client.js";

let aud = document.createElement("audio");
aud.innerHTML = `<source src="chime/chime-1.mp3" type="audio/mpeg">
                  <source src="chime/chime-1.ogg" type="audio/ogg">`;
window.play_hit_audio = function() {
  aud.play();
  aud = document.createElement("audio");
  let nb = Math.floor(Math.random() * 3);
  aud.innerHTML = `<source src="chime/chime-${nb}.mp3" type="audio/mpeg">
                    <source src="chime/chime-${nb}.ogg" type="audio/ogg">`;
}

init();
