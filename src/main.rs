use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct ColorParams {
    _color: Option<String>,
}

// --- WORKING VULNERABLE COLOR ROUTE ---
#[get("/vulnerable_color")]
async fn vulnerable_color(_params: web::Query<ColorParams>) -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Vulnerable Color Example</title>
        </head>
        <body>
            <h1>This color is being applied: <span id="color-name"></span></h1>

            <script>
                const urlParams = new URLSearchParams(window.location.search);
                const color = urlParams.get('color');

                if (color) {
                    // Apply the color to the header (this part is safe)
                    document.querySelector('h1').style.color = color;

                    // DANGEROUS SINK: Display the color name using .innerHTML
                    // .innerHTML parses the string as HTML, allowing script execution
                    // through event handlers like 'onerror'.
                    document.getElementById('color-name').innerHTML = color;
                }
            </script>
        </body>
        </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

// --- SECURE COLOR ROUTE ---
#[get("/secure_color")]
async fn secure_color() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Secure Color Example</title>
        </head>
        <body>
            <h1>This color is being applied: <span id="color-name"></span></h1>

            <script>
                const urlParams = new URLSearchParams(window.location.search);
                const color = urlParams.get('color');

                if (color) {
                    document.querySelector('h1').style.color = color;

                    // SAFE SINK: .textContent treats the entire string as plain text.
                    // It will be displayed, but never interpreted as HTML or executed.
                    document.getElementById('color-name').textContent = color;
                }
            </script>
        </body>
        </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:3000");
    println!("- Vulnerable Color Page: http://127.0.0.1:3000/vulnerable_color");
    println!("- Secure Color Page:     http://127.0.0.1:3000/secure_color");

    HttpServer::new(|| App::new().service(vulnerable_color).service(secure_color))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
