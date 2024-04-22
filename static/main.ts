// CONSTANTS
const jsonURL: string = "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/manifest.json";

const card_data = `<div class="card" style="width: 18rem;">
<div class="card-body">
  <h5 class="card-title">Card title</h5>
  <h6 class="card-subtitle mb-2 text-muted">Card subtitle</h6>
  <p class="card-text">Some quick example text to build on the card title and make up the bulk of the card's content.</p>
  <a href="#" class="card-link">Card link</a>
  <a href="#" class="card-link">Another link</a>
</div>
</div>`;


function handle_json_data(data: object): void {
    console.log(data);
    for (let i: number = 0; i < data["addons"].length; i++) {
        // The following is done to exclude the first 3 unnessecary entries.
        if (i < 3) {
            continue;
        }
        var name: string = JSON.stringify(data["addons"][i]['id']);
        var name: string = name.replace("\"", "").replace("\"", "");
        var title: string = name;
        title = title.replace("_", " ")
        title = title.charAt(0).toUpperCase() + title.slice(1);
        var description: string = JSON.stringify(data["addons"][i]['description']);
        var parent_div = document.getElementById("place_cards_here") as HTMLInputElement;
        parent_div.innerHTML += `<div class="card" style="width: 18rem;">
        <div class="card-body">
          <h5 class="card-title">${title}</h5>
          <h6 class="card-subtitle mb-2 text-muted">${name}</h6>
          <p class="card-text">Some quick example text to build on the card title and make up the bulk of the card's content.</p>
          <a href="#" class="card-link">Card link</a>
          <a href="#" class="card-link">Another link</a>
        </div>
        </div>`;
        // document.writeln(name + "is", description + "<br/>");
        console.log(data[i]);
    }
}


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

