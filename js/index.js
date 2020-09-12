const piano = {
  A0: 27.5,
  "A♯0/B♭0": 29.13524,
  B0: 30.86771,
  C1: 32.7032,
  "C♯1/D♭": 34.64783,
  D1: 36.7081,
  "D♯1/E♭1": 38.89087,
  E1: 41.20344,
  F1: 43.65353,
  "F♯1/G♭": 46.2493,
  G1: 48.99943,
  "G♯1/A♭": 51.91309,
  A1: 55.0,
  "A♯1/B♭1": 58.27047,
  B1: 61.73541,
  C2: 65.40639,
  "C♯2/D♭2": 69.29566,
  D2: 73.41619,
  "D♯2/E♭2": 77.78175,
  E2: 82.40689,
  F2: 87.30706,
  "F♯2/G♭2": 92.49861,
  G2: 97.99886,
  "G♯2/A♭2": 103.8262,
  A2: 110.0,
  "A♯2/B♭2": 116.5409,
  B2: 123.4708,
  C3: 130.8128,
  "C♯3/D♭3": 138.5913,
  D3: 146.8324,
  "D♯3/E♭3": 155.5635,
  E3: 164.8138,
  F3: 174.6141,
  "F♯3/G♭3": 184.9972,
  G3: 195.9977,
  "G♯3/A♭3": 207.6523,
  A3: 220.0,
  "A♯3/B♭3": 233.0819,
  B3: 246.9417,
  C4: 261.6256,
  "C♯4/D♭4": 277.1826,
  D4: 293.6648,
  "D♯4/E♭4": 311.127,
  E4: 329.6276,
  F4: 349.2282,
  "F♯4/G♭4": 369.9944,
  G4: 391.9954,
  "G♯4/A♭4": 415.3047,
  A4: 440.0,
  "A♯4/B♭4": 466.1638,
  B4: 493.8833,
  C5: 523.2511,
  "C♯5/D♭5": 554.3653,
  D5: 587.3295,
  "D♯5/E♭5": 622.254,
  E5: 659.2551,
  F5: 698.4565,
  "F♯5/G♭5": 739.9888,
  G5: 783.9909,
  "G♯5/A♭5": 830.6094,
  A5: 880.0,
  "A♯5/B♭5": 932.3275,
  B5: 987.7666,
  C6: 1046.502,
  "C♯6/D♭6": 1108.731,
  D6: 1174.659,
  "D♯6/E♭6": 1244.508,
  E6: 1318.51,
  F6: 1396.913,
  "F♯6/G♭6": 1479.978,
  G6: 1567.982,
  "G♯6/A♭6": 1661.219,
  A6: 1760.0,
  "A♯6/B♭6": 1864.655,
  B6: 1975.533,
  C7: 2093.005,
};

import("../pkg")
  .then((rust_module) => {
    let fm = null;

    const play_button = document.getElementById("play");
    play_button.addEventListener("click", (event) => {
      if (fm === null) {
        play_button.innerHTML = "⏸";
        fm = new rust_module.FmOsc();
        fm.set_note(50);
        fm.set_fm_frequency(2);
        fm.set_fm_amount(0);
        fm.set_gain(0.8);
      } else {
        play_button.innerHTML = "▶️";
        fm.free();
        fm = null;
      }
    });
    const pianoDiv = document.getElementById("piano");
    pianoDiv.style.display = "flex";
    pianoDiv.style.flexDirection = "row";
    pianoDiv.style.position = "relative";

    Object.entries(piano)
      .sort(([, pA], [, pB]) => {
        return pA < pB;
      })
      .map(([key, value], i) => {
        let isBlackKey = key.includes("/");
        let div = document.createElement("kbd");

        // div.style.border = "-1px inset black";
        div.style.boxShadow = "inset 0px 0px 1px black";

        if (!isBlackKey) {
          div.style.width = "10px";
          div.style.height = "80px";
          // div.style.transform = "translateY(10px)";
          const [note, num] = key;
          div.innerText = note;
        } else {
          div.style.width = "10px";
          div.style.height = "80px";
          const [sharp, flat] = key.split("/");
          const [note, sign, num] = sharp;
          // div.innerText = note + sign;
          div.style.background = "lightgrey";
        }
        let ignore = false;
        div.addEventListener("mousedown", () => {
          ignore = !ignore;
          if (div.style.background !== "red") {
            div.style.background = "red";
          } else {
            div.style.background = "inherit";
          }
        });
        div.addEventListener("mouseover", () => {
          if (fm && !ignore) {
            fm.set_primary_frequency(parseInt(value));
            // fm.set_note(parseInt(i + 12));
            // fm.set_fm_frequency(parseFloat(value));
          }
        });
        pianoDiv.appendChild(div);
      });

    // document.body.addEventListener("mousemove", (event) => {
    //   console.log(event.clientX, event.clientY);
    //   if (fm) {
    //     fm.set_note(parseInt(event.clientX / 2));
    //     fm.set_fm_frequency(parseFloat(event.clientX));
    //   }
    // })
  })
  .catch(console.error);
