use std::fmt::format;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiR09SS0EiLCJpYXQiOjE2ODM2MTU5NDEsInN1YiI6ImFnZW50LXRva2VuIn0.VblN4ffRj4Mz9zE85lP7cE5aznsG2wqGBl7L2NF2oVaVsU1bmi3WquwWC9xAlQKMhEaakqtckFFFG9482_410-QXMXhJTOI4id71Cn0Tefa_vonrFvdVrO7ui4tbxBlJag-BpCVo96Nksqyd3usu3-18YEbT5D8jxgPunQdOrDohNytTAWlkt2Vgtrz1Uz8ZuLYcD1tZKW1EgpGfICq7ndocnSFiiIvO-wcMn-JuI9fjEBwVC086R2RKFHF6RZDOPL_DqU1DC-ZnkNz7dNp1CM9TMk3Ps1hhUa5ttrNTIthHdBUF260HIIFbUP5cZuzy2B_fwlfnbl6rK_EaF0SwYrlKWhgqWOUUQCyITAlqOWnNDZ6PGcvNsaPvBRhba4DECroKPs3PTmeGlOGSj1we3WhzOcuCictsAAVfcnw2Ex8nvCgi3lcGItBvtrs-xrVIiXE8i8Rud2Wv2C9yXKRz16D96I16fqcQ5qxRo9eJmDS9yXGThGvPXr4M0Vyvacdl";
    let client = reqwest::Client::new();
    let res = client.get("https://api.spacetraders.io/v2/my/agent").header("Authorization", format!("Bearer {}", token)).send().await?;

    // let res = reqwest::get("https://api.spacetraders.io/v2/my/agent").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    let client = reqwest::Client::new();
    let res = client.get("https://api.spacetraders.io/v2/systems/X1-DF55/waypoints/X1-DF55-20250Z").header("Authorization", format!("Bearer {}", token)).send().await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
