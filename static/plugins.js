"use strict";

import { marked } from "https://cdn.jsdelivr.net/npm/marked/lib/marked.esm.js";

const jsonURL = "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/manifest.json";

const parentDiv = document.getElementById("place_cards_here");
const searchBox = document.getElementById("searchbox");

var globalData = null;

function handleJsonData(data) {
    globalData = data;
    parentDiv.innerHTML = buildHtml(data);
    return;
}

function buildHtml(data) {
    let html = "";

    for (let i = 3; i < data.addons.length; i++) {
        const addon = data.addons[i];
        const name = addon.id;
        const description = addon.description;
        const title = addon.name ? addon.name : name[0].toUpperCase() + name.slice(1).replace("_", " ");
        html += `<div class="card" style="width: 18rem;"><div class="card-body">
<h5 class="card-title">${title}</h5>
<h6 class="card-subtitle mb-2 text-muted">${name}</h6>
<p class="card-text markdownContent">${marked.parse(description)}</p>
<a href="/@plugins/plugin_slug?plugin=${name}" class="card-link btn btn-primary">View plugin</a>
</div>
</div>`;
    }

    return html;
}

function handle_typing() {
    let html = "";
    const searchBoxContents = searchBox.value;
    if (searchBoxContents) {
        parentDiv.innerHTML = globalData;
        return;
    }
    var data = globalData;
    for (var i = 3; i < data["addons"].length; i++) {
        const addon = data.addons[i];
        const name = addon.id;
        const description = addon.description;
        const title = addon.name ? addon.name : name[0].toUpperCase() + name.slice(1).replace("_", " ");
        if (title.includes(searchBoxContents) || description.includes(searchBoxContents) || name.includes(searchBoxContents)) {
            html += `<div class="card" style="width: 18rem;">
<div class="card-body">
    <h5 class="card-title">${title.replace(searchBoxContents, "<span style='background-color:yellow;color:black;'>" + searchBoxContents + "</span>")}</h5>
    <h6 class="card-subtitle mb-2 text-muted">${name.replace(searchBoxContents, "<span style='background-color:yellow;color:black;'>" + searchBoxContents + "</span>")}</h6>
    <p class="card-text markdownContent">${marked.parse(description).replace(searchBoxContents, "<span style='background-color:yellow;color:black;'>" + searchBoxContents + "</span>")}</p>
    <a href="/@plugins/plugin_slug?plugin=${name}" class="card-link btn btn-primary">View plugin</a>
</div>
</div>`
        }
        parentDiv.innerHTML = html;
        return;
    }
}

function main() {
    fetch(jsonURL)
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            handleJsonData(data);
        })
        .catch(error => {
            console.error('Error fetching JSON:', error);
        });

    searchBox.addEventListener('input', handle_typing);
    return 0;
}

let _ = main();