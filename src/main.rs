use rohanasan::{rohanasan, send_file, send_http_response, serve, Request, DEFAULT_HTML_HEADER};
mod database;

fn handle(req: Request) -> String {
    if req.path == "/" {
        send_file(DEFAULT_HTML_HEADER, "./html/index.html", req)
    } else if req.path == "/@plugins" {
        send_file(DEFAULT_HTML_HEADER, "./html/see_the_list.html", req)
    } else {
        send_file(DEFAULT_HTML_HEADER, "./html/index.html", req)
    }
}

fn main() {
    rohanasan! {
        serve(8080, handle)
    }
}
