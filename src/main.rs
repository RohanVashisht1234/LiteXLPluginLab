use rohanasan::{rohanasan, send_file, serve, Request, DEFAULT_HTML_HEADER};

fn handle(req: Request) -> String {
    if req.path == "/" {
        send_file(DEFAULT_HTML_HEADER, "./html/index.html", req)
    } else if req.path == "/@plugins" {
        send_file(DEFAULT_HTML_HEADER, "./html/plugins.html", req)
    } else if req.path.starts_with("/@plugins/") {
        send_file(DEFAULT_HTML_HEADER, "./html/plugin_slug.html", req)
    } else if req.path == "/@about" {
        send_file(DEFAULT_HTML_HEADER, "./html/about.html", req)
    } else {
        send_file(DEFAULT_HTML_HEADER, "./html/index.html", req)
    }
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
