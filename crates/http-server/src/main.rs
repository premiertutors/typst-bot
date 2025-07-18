use actix_web::{post, web, App, HttpResponse, HttpServer};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use worker_lib::{render_with_resolution, Sandbox};

#[derive(Debug, thiserror::Error)]
#[error("Invalid theme")]
struct InvalidTheme;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Theme {
    Transparent,
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

impl FromStr for Theme {
    type Err = InvalidTheme;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "transparent" | "t" => Self::Transparent,
            "light" | "l" => Self::Light,
            "dark" | "d" => Self::Dark,
            _ => return Err(InvalidTheme),
        })
    }
}

impl Theme {
    const fn preamble(self) -> &'static str {
        match self {
            Self::Transparent => "",
            Self::Light => "#set page(fill: white)\n",
            Self::Dark => concat!(
                "#set page(fill: rgb(49, 51, 56))\n",
                "#set text(fill: rgb(219, 222, 225))\n",
            ),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid page size")]
struct InvalidPageSize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PageSize {
    Preview,
    Auto,
    Default,
}

impl Default for PageSize {
    fn default() -> Self {
        Self::Preview
    }
}

impl FromStr for PageSize {
    type Err = InvalidPageSize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "preview" | "p" => Self::Preview,
            "auto" | "a" => Self::Auto,
            "default" | "d" => Self::Default,
            _ => return Err(InvalidPageSize),
        })
    }
}

impl PageSize {
    const fn preamble(self) -> &'static str {
        match self {
            Self::Preview => "#set page(width: 300pt, height: auto, margin: 10pt)\n",
            Self::Auto => "#set page(width: auto, height: auto, margin: 10pt)\n",
            Self::Default => "",
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Preamble {
    page_size: PageSize,
    theme: Theme,
}

impl Preamble {
    fn preamble(self) -> String {
        let page_size = self.page_size.preamble();
        let theme = self.theme.preamble();
        if theme.is_empty() && page_size.is_empty() {
            String::new()
        } else {
            format!(
                concat!(
                    "// Begin preamble\n",
                    "// Page size:\n",
                    "{page_size}",
                    "// Theme:\n",
                    "{theme}",
                    "// End preamble\n",
                ),
                page_size = page_size,
                theme = theme,
            )
        }
    }
}

#[derive(Deserialize)]
struct Req {
    code: String,
    #[serde(default)]
    theme: Option<Theme>,
    #[serde(default)]
    page_size: Option<PageSize>,
    #[serde(default)]
    resolution: Option<f32>,
}

#[derive(Serialize)]
struct Resp {
    images: Vec<String>,
    more_pages: usize,
    warnings: String,
}

#[post("/render")]
async fn do_render(body: web::Json<Req>, data: web::Data<Sandbox>) -> HttpResponse {
    let mut source = body.code.clone();
    
    // Apply preamble based on theme and page_size parameters
    let preamble = Preamble {
        theme: body.theme.unwrap_or_default(),
        page_size: body.page_size.unwrap_or_default(),
    };
    source.insert_str(0, &preamble.preamble());
    
    let resolution = body.resolution.unwrap_or(1000.0);
    let out = web::block(move || render_with_resolution(&data, source, resolution)).await;
    match out {
        Ok(Ok(o)) => HttpResponse::Ok().json(Resp {
            images: o.images.into_iter().map(|img| BASE64_STANDARD.encode(img)).collect(),
            more_pages: o.more_pages,
            warnings: o.warnings,
        }),
        Ok(Err(e)) => HttpResponse::BadRequest().body(e),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sandbox = Sandbox::new();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sandbox.clone()))
            .service(do_render)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
