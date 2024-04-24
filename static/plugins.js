"use strict";

import { marked } from "https://cdn.jsdelivr.net/npm/marked/lib/marked.esm.js";

const jsonURL = "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/manifest.json";

const parentDiv = document.getElementById("place_cards_here");
const searchBox = document.getElementById("searchbox");

var globalData = null;

function handleJsonData(data) {
    globalData = data;
    const html = buildHtml(data);
    parentDiv.innerHTML = html;
}

function buildHtml(data) {
    let html = "";
    for (let i = 3; i < data.addons.length; i++) {
        const addon = data.addons[i];
        const name = addon.id;
        let description = addon.description;
        let title = addon.name ? addon.name : name[0].toUpperCase() + name.slice(1).replace("_", " ");
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
    var parent_div = document.getElementById("place_cards_here");
    parent_div.innerHTML = "";
    var data = globalData;
    for (var i = 0; i < data["addons"].length; i++) {
        // The following is done to exclude the first 3 unnessecary entries.
        if (i < 3) {
            continue;
        }
        var name = data["addons"][i]['id'];
        name = name;
        var title = name;
        title = title.replace("_", " ");
        title = title.charAt(0).toUpperCase() + title.slice(1);
        var description = data["addons"][i]['description'];
        description = description;
        var thing_that_is_being_typed_in_the_search_box = document.getElementById("searchbox").value;
        if ("name" in data["addons"][i]) {
            title = data["addons"][i]['name'];
            title = title;
        }
        if (title.includes(thing_that_is_being_typed_in_the_search_box) || description.includes(thing_that_is_being_typed_in_the_search_box) || name.includes(thing_that_is_being_typed_in_the_search_box)) {
            parent_div.innerHTML += `<div class="card" style="width: 18rem;">
        <div class="card-body">
          <h5 class="card-title">${title.replace(thing_that_is_being_typed_in_the_search_box, "<span style='background-color:yellow;color:black;'>" + thing_that_is_being_typed_in_the_search_box + "</span>")}</h5>
          <h6 class="card-subtitle mb-2 text-muted">${name.replace(thing_that_is_being_typed_in_the_search_box, "<span style='background-color:yellow;color:black;'>" + thing_that_is_being_typed_in_the_search_box + "</span>")}</h6>
          <p class="card-text markdownContent">${marked.parse(description).replace(thing_that_is_being_typed_in_the_search_box, "<span style='background-color:yellow;color:black;'>" + thing_that_is_being_typed_in_the_search_box + "</span>")}</p>
          <a href="/@plugins/plugin_slug?plugin=${name}" class="card-link btn btn-primary">View plugin</a>
        </div>
        </div>`;
        }
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
}

main();