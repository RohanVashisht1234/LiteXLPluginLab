pub mod database;
use database::{BOTTOM, TOP};

use rohanasan::{
    rohanasan, send_file_top_bottom, serve, Request, DEFAULT_404_HEADER,
    DEFAULT_HTML_HEADER,
};

fn handle(req: Request) -> String {
    if req.path == "/" {
        // Send index page on path /
        send_file_top_bottom(DEFAULT_HTML_HEADER, "./html/index.html", TOP.replace("{%TITLE%}", "Index").as_str(), BOTTOM, req)
    } else if req.path == "/@plugins" {
        // Send plugins page on path /@plugins
        send_file_top_bottom(DEFAULT_HTML_HEADER, "./html/plugins.html", TOP.replace("{%TITLE%}", "Plugins").as_str(), BOTTOM, req)
    } else if req.path.starts_with("/@plugins/") {
        // Send plugin_slug page on path /@plugins/*
        send_file_top_bottom(
            DEFAULT_HTML_HEADER,
            "./html/plugin_slug.html",
            TOP.replace("{%TITLE%}", "View plugin").as_str(),
            BOTTOM,
            req,
        )
    } else {
        // Send 404 on all other pages
        send_file_top_bottom(DEFAULT_404_HEADER, "./html/404.html", TOP, BOTTOM, req)
    }
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
