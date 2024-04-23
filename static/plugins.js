"use strict";

import { marked } from "https://cdn.jsdelivr.net/npm/marked/lib/marked.esm.js";

const jsonURL = "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/manifest.json";

var global_data = ""; // This gets populated by the function handle_json_data

function handle_json_data(data) {
    console.log(data);
    for (var i = 0; i < data["addons"].length; i++) {
        // The following is done to exclude the first 3 unnessecary entries.
        if (i < 3) {
            continue;
        }
        var name = JSON.stringify(data["addons"][i]['id']);
        name = name.replace("\"", "").replace("\"", "");
        var title = name;
        title = title.replace("_", " ");
        title = title.charAt(0).toUpperCase() + title.slice(1);
        var description = JSON.stringify(data["addons"][i]['description']);
        description = description.replace("\"", "").replace("\"", "");
        var parent_div = document.getElementById("place_cards_here");
        parent_div.innerHTML += `<div class="card" style="width: 18rem;">
        <div class="card-body">
          <h5 class="card-title">${title}</h5>
          <h6 class="card-subtitle mb-2 text-muted">${name}</h6>
          <p class="card-text markdownContent">${marked.parse(description)}</p>
          <a href="/@plugins/${name}" class="card-link btn btn-primary">View plugin</a>
        </div>
        </div>`;
    }
    global_data = data;
}


function handle_typing() {
    var parent_div = document.getElementById("place_cards_here");
    parent_div.innerHTML = "";
    var data = global_data;
    for (var i = 0; i < data["addons"].length; i++) {
        // The following is done to exclude the first 3 unnessecary entries.
        if (i < 3) {
            continue;
        }
        var name = JSON.stringify(data["addons"][i]['id']);
        name = name.replace("\"", "").replace("\"", "");
        var title = name;
        title = title.replace("_", " ");
        title = title.charAt(0).toUpperCase() + title.slice(1);
        var description = JSON.stringify(data["addons"][i]['description']);
        description = description.replace("\"", "").replace("\"", "");
        var thing_that_is_being_typed_in_the_search_box = document.getElementById("searchbox").value;
        if (title.includes(thing_that_is_being_typed_in_the_search_box) || description.includes(thing_that_is_being_typed_in_the_search_box) || name.includes(thing_that_is_being_typed_in_the_search_box)) {
            parent_div.innerHTML += `<div class="card" style="width: 18rem;">
        <div class="card-body">
          <h5 class="card-title">${title.replace(thing_that_is_being_typed_in_the_search_box, "<span style='background-color:yellow;color:black;'>" + thing_that_is_being_typed_in_the_search_box + "</span>")}</h5>
          <h6 class="card-subtitle mb-2 text-muted">${name.replace(thing_that_is_being_typed_in_the_search_box, "<span style='background-color:yellow;color:black;'>" + thing_that_is_being_typed_in_the_search_box + "</span>")}</h6>
          <p class="card-text markdownContent">${marked.parse(description).replace(thing_that_is_being_typed_in_the_search_box, "<span style='background-color:yellow;color:black;'>" + thing_that_is_being_typed_in_the_search_box + "</span>")}</p>
          <a href="/@plugins/${name}" class="card-link btn btn-primary">View plugin</a>
        </div>
        </div>`;
        }
    }
}

function main() {
    fetch(jsonURL).then(function (response) {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        return response.json();
    }).then(function (data) {
        handle_json_data(JSON.parse(JSON.stringify(data)));
    }).catch(function (error) {
        console.error('Error fetching JSON:', error);
    });

    document.getElementById("searchbox").oninput = function(){handle_typing()};

    return 0;
}

let _ = main();
