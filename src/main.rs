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
\documentclass[parskip=half,paper=a4]{scrlttr2}

\usepackage{polyglossia}
\usepackage{fontspec}

\renewcommand{\familydefault}{\sfdefault}

\setmainfont{Lato}
\setsansfont{Lato}

\setdefaultlanguage{german}

\newkomavar
    [Geburtsdatum]
    {dob}
\newkomavar
    [Geburtsort]
    {birthplace}

\newkomavar[bisheriger Geschlechtseintrag]{previoussex}
\newkomavar[bisherige(r) Vorname(n)]{previousname}
\newkomavar[neuer Geschlechtseintrag]{newsex}
\newkomavar[neue(r) Vorname(n)]{newname}

\begin{document}

\setkomavar{fromname}{Max Mustermann}
\setkomavar{date}{1. August. 2024}
\setkomavar{fromaddress}{strasse\\plzustadt}
\setkomavar{fromemail}{anon@example.com}
\setkomavar{fromphone}{+49~221~69\,800\,700}

\setkomavar{dob}{9. September 1999}
\setkomavar{birthplace}{Geisterstadt}

\setkomavar{previoussex}{männlich}
\setkomavar{previousname}{Max}

\setkomavar{newsex}{weiblich}
\setkomavar{newname}{Erika}

\newcommand\wantsname{true}
\newcommand\wantssex{false}

\begin{letter}{amtname, amtstrasse, amtstadt}
\opening{Sehr geehrte Sachbearbeiter*innen,}
hiermit melde ich, \usekomavar{fromname}, geboren am \usekomavar{dob} in \usekomavar{birthplace}, die Änderung meines Geschlechtseintrags und Vornamens nach §4 SBGG an.

\ifdefstring{\wantssex}{true}{ % true
Mein \usekomavar*{previoussex} \textit{\usekomavar{previoussex}} sollen in den neuen Geschlechtseintrag \textit{\usekomavar{newsex}} geändert werden.
}{%false
Mein Geschlechtseintrag soll gestrichen werden.
}

\ifdefstring{\wantsname}{true}{ % true
Mein \usekomavar*{previousname} \textit{\usekomavar{previousname}} sollen in den neuen Vornamen \textit{\usekomavar{newname}} geändert werden.
}


Zur Abgabe der Erklärung [nach § 2 SBGG] würde ich gerne einen Termin mit Ihnen vereinbaren.

Zur Terminvereinbarung  können Sie mich auch per E‐Mail unter \usekomavar{fromemail} oder telefonisch unter \usekomavar{fromphone} erreichen.

\closing{Mit freundlichen Grüßen}
(Unterschrift)
\end{letter}

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
    println!("Downloading needed TeX files and testing if PDF engine works..");
    let latex = r#"
\documentclass[parskip=half,paper=a4]{scrlttr2}

\usepackage{polyglossia}
\usepackage{fontspec}

\renewcommand{\familydefault}{\sfdefault}

\setmainfont{Lato}
\setsansfont{Lato}

\setdefaultlanguage{german}

\newkomavar
    [Geburtsdatum]
    {dob}
\newkomavar
    [Geburtsort]
    {birthplace}

\newkomavar[bisheriger Geschlechtseintrag]{previoussex}
\newkomavar[bisherige(r) Vorname(n)]{previousname}
\newkomavar[neuer Geschlechtseintrag]{newsex}
\newkomavar[neue(r) Vorname(n)]{newname}

\begin{document}

\setkomavar{fromname}{Max Mustermann}
\setkomavar{date}{1. August. 2024}
\setkomavar{fromaddress}{strasse\\plzustadt}
\setkomavar{fromemail}{anon@example.com}
\setkomavar{fromphone}{+49~221~69\,800\,700}

\setkomavar{dob}{9. September 1999}
\setkomavar{birthplace}{Geisterstadt}

\setkomavar{previoussex}{männlich}
\setkomavar{previousname}{Max}

\setkomavar{newsex}{weiblich}
\setkomavar{newname}{Erika}

\newcommand\wantsname{true}
\newcommand\wantssex{false}

\begin{letter}{amtname, amtstrasse, amtstadt}
\opening{Sehr geehrte Sachbearbeiter*innen,}
hiermit melde ich, \usekomavar{fromname}, geboren am \usekomavar{dob} in \usekomavar{birthplace}, die Änderung meines Geschlechtseintrags und Vornamens nach §4 SBGG an.

\ifdefstring{\wantssex}{true}{ % true
Mein \usekomavar*{previoussex} \textit{\usekomavar{previoussex}} sollen in den neuen Geschlechtseintrag \textit{\usekomavar{newsex}} geändert werden.
}{%false
Mein Geschlechtseintrag soll gestrichen werden.
}

\ifdefstring{\wantsname}{true}{ % true
Mein \usekomavar*{previousname} \textit{\usekomavar{previousname}} sollen in den neuen Vornamen \textit{\usekomavar{newname}} geändert werden.
}


Zur Abgabe der Erklärung [nach § 2 SBGG] würde ich gerne einen Termin mit Ihnen vereinbaren.

Zur Terminvereinbarung  können Sie mich auch per E‐Mail unter \usekomavar{fromemail} oder telefonisch unter \usekomavar{fromphone} erreichen.

\closing{Mit freundlichen Grüßen}
(Unterschrift)
\end{letter}

\end{document}

"#;
    let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).expect("processing failed");
    println!("Success! Output PDF size is {} bytes", pdf_data.len());

    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 45565))?
        .run()
        .await
}
