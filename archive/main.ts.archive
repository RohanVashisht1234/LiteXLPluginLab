// @ts-ignore Import module
import { marked } from "https://cdn.jsdelivr.net/npm/marked/lib/marked.esm.js";

const jsonURL: string = "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/manifest.json";

function handle_json_data(data: object): void {
    console.log(data);
    for (let i: number = 0; i < data["addons"].length; i++) {
        // The following is done to exclude the first 3 unnessecary entries.
        if (i < 3) {
            continue;
        }
        var name: string = JSON.stringify(data["addons"][i]['id']);
        name = name.replace("\"", "").replace("\"", "");
        var title: string = name;
        title = title.replace("_", " ");
        title = title.charAt(0).toUpperCase() + title.slice(1);
        var description: string = JSON.stringify(data["addons"][i]['description']);
        description = description.replace("\"", "").replace("\"", "");
        var markdownDiv = document.getElementsByClassName('markdownContent');
        var parent_div: HTMLInputElement = document.getElementById("place_cards_here") as HTMLInputElement;
        parent_div.innerHTML += `<div class="card" style="width: 18rem;">
        <div class="card-body">
          <h5 class="card-title">${title}</h5>
          <h6 class="card-subtitle mb-2 text-muted">${name}</h6>
          <p class="card-text markdownContent">${marked.parse(description)}</p>
          <a href="#" class="card-link btn btn-primary">Card link</a>
          <a href="#" class="card-link btn btn-outline-secondary">Another link</a>
        </div>
        </div>`;
        // document.writeln(name + "is", description + "<br/>");
        console.log(data[i]);
    }
}

function main(): number {
    fetch(jsonURL).then((response: Response): Promise<string> => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        return response.json();
    }).then((data: string): void => {
        handle_json_data(JSON.parse(JSON.stringify(data)));
    }).catch((error: Error): void => {
        console.error('Error fetching JSON:', error);
    });
    return 0;
}

let _ = main();

