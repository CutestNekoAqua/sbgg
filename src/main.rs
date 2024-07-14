use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

#[get("/")]
async fn hello() -> impl Responder {
    let res = gen_pdf();
    if let Ok(good) = res {
        return HttpResponse::Ok().body(good);
    }
    HttpResponse::UnprocessableEntity().body("Ein Fehler ist aufgetreten.")
}

fn gen_pdf() -> anyhow::Result<Vec<u8>> {
    let latex = r#"
\documentclass{article}
\begin{document}
Hello, world!
\end{document}
"#;
    let res = tectonic::latex_to_pdf(latex);
    if let Ok(good) = res {
        return Ok(good);
    } else {
        return Err(anyhow::anyhow!("Awa error mommy"));
    }
}

#[derive(Parser, Debug)]
#[clap(author = "April John", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    name: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Testing if PDF engine works..");
    let latex = r#"
\documentclass{article}
\begin{document}
Hello, world!
\end{document}
"#;
    let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).expect("processing failed");
    println!("Success! Output PDF size is {} bytes", pdf_data.len());

    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 45565))?
        .run()
        .await
}
