use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    amtname: String,
    amtstreet: String,
    amtcity: String,
    fname: String,
    lname: String,
    street: String,
    city: String,
    email: String,
    phone: String,
    dob: String,
    birthplace: String,
    change_name: Option<bool>,
    prev_gender: String,
    post_gender: String, // remove the x
    post_fname: Option<String>,
}

#[get("/")]
async fn hello(data: web::Query<Data>) -> impl Responder {
    let keep_gender = &data.post_gender.eq("xnone");
    let res = gen_pdf(
        data.amtname,
        data.amtstreet,
        data.amtcity,
        data.fname,
        data.lname,
        data.street,
        data.city,
        data.email,
        data.phone,
        data.dob,
        data.birthplace,
        data.change_name.unwrap_or(false),
        keep_gender,
        data.data.prev_gender,
        data.post_gender.split("x").collect()[1],
        data.post_fname.unwrap_or("".to_string()),
    );
    if let Ok(good) = res {
        return HttpResponse::Ok().body(good);
    }
    HttpResponse::UnprocessableEntity().body("Ein Fehler ist aufgetreten.")
}

fn gen_pdf(
    amtname: String,
    amtstreet: String,
    amtcity: String,
    fname: String,
    lname: String,
    street: String,
    city: String,
    email: String,
    phone: String,
    dob: String,
    birthplace: String,
    change_name: bool,
    keep_gender: bool,
    prev_gender: String,
    post_gender: String, // remove the x
    post_fname: String,
) -> anyhow::Result<Vec<u8>> {
    let slashslash = r#"\\"#;

    let mut latex = r#"
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
\setkomavar{date}{1. August. 2024} % remove this once past 1st August
\setkomavar{fromaddress}{strasse\\plzustadt}
\setkomavar{fromemail}{anon@example.com}
\setkomavar{fromphone}{08000800}

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

"#.to_string();
    latex = latex.replace("amtname", &amtname);
    latex = latex.replace("amtstrasse", &amtstreet);
    latex = latex.replace("amtstadt", &amtcity);

    latex = latex.replace(
        "wantsname{true}",
        "wantsname{" + &change_name.to_string() + "}",
    );
    latex = latex.replace(
        "wantssex{false}",
        "wantssex{" + &keep_gender.to_string() + "}",
    );

    latex = latex.replace("{newsex}{weiblich}", "{newsex}{" + &post_gender + "}");
    latex = latex.replace("{newname}{Erika}", "{newname}{" + &post_fname + "}");

    latex = latex.replace(
        "{previoussex}{männlich}",
        "{previoussex}{" + &prev_gender + "}",
    );
    latex = latex.replace("{previousname}{Max}", "{previousname}{" + &fname + "}");

    latex = latex.replace("{dob}{9. September 1999}", "{dob}{" + &dob + "}");
    latex = latex.replace(
        "{birthplace}{Geisterstadt}",
        "{birthplace}{" + &birthplace + "}",
    );

    latex = latex.replace(
        "{fromname}{Max Mustermann}",
        "{fromname}{" + &fname + " " + &lname + "}",
    );
    latex = latex.replace(
        "{fromaddress}{strasse\\plzustadt}",
        "{fromaddress}{" + &street + &slashslash + &city + "}",
    );
    latex = latex.replace(
        "{fromemail}{anon@example.com}",
        "{fromemail}{" + &email + "}",
    );
    latex = latex.replace("{fromphone}{08000800}", "{fromphone}{" + &phone + "}");

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
