"use strict";

import { marked } from "https://cdn.jsdelivr.net/npm/marked/lib/marked.esm.js";

const searchParams = new URLSearchParams(window.location.search);
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
        name = data["addons"][i]['id'];

        description = data["addons"][i]['description'];

        if (name == searchParams.get("plugin")) {
            valid_page = true;
            break;
        }
    }
    if (valid_page) {
        var title = name;
        title = title.replace("_", " ");
        title = title[0].toUpperCase() + title.slice(1);
        if ("name" in data["addons"][i]) {
            title = data["addons"][i]['name'];
        }
        document.getElementById("name").innerHTML = title;
        document.getElementById("description").innerHTML = marked.parse(description);
        document.getElementById("install_command").innerHTML = `<span style="color:pink;">lpm</span> <span style="color:lightyellow">install</span> <span style="color:skyblue">${name}</span>`;
        document.getElementById("install_command_miq").innerHTML = `<span style="color:pink;">local</span> <span style="color:lightyellow">config</span> = <span style="color:lightyellow">require</span> <span style="color:skyblue">'core.config'</span><br><br>
<span style="color:lightyellow">config</span>.plugins.miq.plugins = {<br>
&nbsp;&nbsp;&nbsp;&nbsp;...<br>
&nbsp;&nbsp;&nbsp;&nbsp;<span style="color:skyblue">'${name}'</span>,<br>
&nbsp;&nbsp;&nbsp;&nbsp;...<br>
}
`;
    }
    else {
        window.location = "/404";
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
