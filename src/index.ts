// Copyright (c) 2022 Sungbae Jeong
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

import init, { Voca, rand } from "@pkg/vrot.js";

function readFilesAsText(file: File) {
    return new Promise((resolve, reject) => {
        let fr = new FileReader();
        fr.onload = () => { resolve(fr.result); };
        fr.onerror = () => { reject(fr); };
        fr.readAsText(file);
    });
}

function main(values: unknown[]) {
    let yamlStr = "";
    for (var value of values) {
        yamlStr += value;
    }
    let vocas = Voca.new(yamlStr) as any[];
    const vocasLen = vocas.length;
    let mainVoca = document.querySelector("#main-voca") as Element;
    let knownButton = document.querySelector("#known-word") as Element;
    let unknownButton = document.querySelector("#unknown-word") as Element;
    let answer = document.querySelector("#voca-answer") as HTMLDivElement;

    let idx = rand(vocasLen);
    mainVoca.textContent = vocas[idx].word;

    knownButton.addEventListener("click", () => {
        answer.style.display = "none";
        idx = rand(vocasLen);
        mainVoca.textContent = vocas[idx].word;
    });
    unknownButton.addEventListener("click", () => unknownButtonHandler(vocas, idx, answer));
}

function unknownButtonHandler(vocas: any[], vocaIdx: number, answer: HTMLDivElement) {
    let prevAnswer = document.querySelector("#prev-answer") as HTMLButtonElement;
    let nextAnswer = document.querySelector("#next-answer") as HTMLButtonElement;

    let infoLen = vocas[vocaIdx].info.length;
    let idx = 0;

    nextAnswer.style.display = infoLen <= 1 ? "none" : "block";
    showAnswer(vocas, vocaIdx, answer, idx);

    prevAnswer.addEventListener("click", () => {
        if (idx > 0) {
            idx -= 1;
        }

        showAnswer(vocas, vocaIdx, answer, idx);

        if (idx <= 0) {
            prevAnswer.style.display = "none";
        }
        if (idx + 1 < infoLen) {
            nextAnswer.style.display = "block";
        }
    });

    nextAnswer.addEventListener("click", () => {
        if (idx + 1 < infoLen) {
            idx += 1;
        }

        showAnswer(vocas, vocaIdx, answer, idx);

        if (idx > 0) {
            prevAnswer.style.display = "block";
        }
        if (idx + 1 >= infoLen) {
            nextAnswer.style.display = "none";
        }
    });
}

function showAnswer(vocas: any[], vocaIdx: number, answer: HTMLDivElement, idx: number) {
    let vocaMeaning = document.querySelector("#voca-meaning") as HTMLSpanElement;
    let vocaSynos = document.querySelectorAll(".synos") as NodeListOf<HTMLSpanElement>;
    let vocaSynosText = document.querySelector("#voca-synos") as HTMLSpanElement;
    let vocaExample = document.querySelectorAll(".example") as NodeListOf<HTMLSpanElement>;
    let vocaExampleText = document.querySelector("#voca-example") as HTMLSpanElement;

    // change voca answers
    vocaMeaning.textContent = vocas[vocaIdx].info[idx].meaning;
    if (typeof (vocas[vocaIdx].info[0].synos) !== "undefined") {
        for (var item of vocaSynos) {
            item.style.display = "block";
        }
        vocaSynosText.textContent = vocas[vocaIdx].info[idx].synos.join(", ");
    } else {
        for (var item of vocaSynos) {
            item.style.display = "none";
        }
    }
    if (typeof (vocas[vocaIdx].info[idx].example) !== "undefined") {
        for (var item of vocaExample) {
            item.style.display = "block";
        }
        vocaExampleText.textContent = vocas[vocaIdx].info[idx].example;
    } else {
        for (var item of vocaExample) {
            item.style.display = "none";
        }
    }
    // show answer
    answer.style.display = "block";
}

// Main actor
const runWasm = async () => {
    await init();
    const fileSelector = document.querySelector("#yaml-reader") as Element;
    fileSelector.addEventListener('change', (event) => {
        let fileList = event.target as HTMLInputElement;
        let readers = [];
        for (const file of fileList.files as FileList) {
            readers.push(readFilesAsText(file));
        }
        Promise.all(readers).then(main);
    });
};
runWasm();