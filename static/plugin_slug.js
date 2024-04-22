"use strict";

import { marked } from "https://cdn.jsdelivr.net/npm/marked/lib/marked.esm.js";

const jsonURL = "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/manifest.json";

function handle_json_data(data) {
    var valid_page = false;
    console.log(data);
    var name = "";
    var description = "";
    for (var i = 0; i < data["addons"].length; i++) {
        // The following is done to exclude the first 3 unnessecary entries.
        if (i < 3) {
            continue;
        }
        name = JSON.stringify(data["addons"][i]['id']);
        name = name.replace("\"", "").replace("\"", "");

        description = JSON.stringify(data["addons"][i]['description']);
        description = description.replace("\"", "").replace("\"", "");
        if (name == document.URL.split("/")[document.URL.split("/").length - 1]) {
            valid_page = true;
            break;
        }
    }
    if (valid_page) {
        var title = name;
        title = title.replace("_", " ");
        title = title.charAt(0).toUpperCase() + title.slice(1);
        document.getElementById("name").innerHTML = title;
        document.getElementById("description").innerHTML = marked.parse(description);
        document.getElementById("install_command").innerHTML = `<span style="color:pink;">lpm</span> <span style="color:lightyellow">install</span> <span style="color:skyblue">${name}</span>`;
    }
    else {
        document.write("<h1>THIS IS NOT A PLUGIN</h1>");
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
    return 0;
}

let _ = main();
