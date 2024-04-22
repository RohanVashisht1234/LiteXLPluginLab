// CONSTANTS
var jsonURL = "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/manifest.json";
var card_data = "<div class=\"card\" style=\"width: 18rem;\">\n<div class=\"card-body\">\n  <h5 class=\"card-title\">Card title</h5>\n  <h6 class=\"card-subtitle mb-2 text-muted\">Card subtitle</h6>\n  <p class=\"card-text\">Some quick example text to build on the card title and make up the bulk of the card's content.</p>\n  <a href=\"#\" class=\"card-link\">Card link</a>\n  <a href=\"#\" class=\"card-link\">Another link</a>\n</div>\n</div>";
function handle_json_data(data) {
    console.log(data);
    for (var i = 0; i < data["addons"].length; i++) {
        // The following is done to exclude the first 3 unnessecary entries.
        if (i < 3) {
            continue;
        }
        var name = JSON.stringify(data["addons"][i]['id']);
        var name = name.replace("\"", "").replace("\"", "");
        var title = name;
        title = title.replace("_", " ");
        title = title.charAt(0).toUpperCase() + title.slice(1);
        var description = JSON.stringify(data["addons"][i]['description']);
        var parent_div = document.getElementById("place_cards_here");
        parent_div.innerHTML += "<div class=\"card\" style=\"width: 18rem;\">\n        <div class=\"card-body\">\n          <h5 class=\"card-title\">".concat(title, "</h5>\n          <h6 class=\"card-subtitle mb-2 text-muted\">").concat(name, "</h6>\n          <p class=\"card-text\">Some quick example text to build on the card title and make up the bulk of the card's content.</p>\n          <a href=\"#\" class=\"card-link\">Card link</a>\n          <a href=\"#\" class=\"card-link\">Another link</a>\n        </div>\n        </div>");
        // document.writeln(name + "is", description + "<br/>");
        console.log(data[i]);
    }
}
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
