use std::{io::Result, format};
use actix_cors::Cors;
use actix_web::{HttpServer, App, get, post, HttpResponse, web::{ Json, Path, self}, middleware::NormalizePath};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use sled_json::JsonDb;

lazy_static! {
    static ref DB: JsonDb = JsonDb::open("idk").unwrap();
}

#[derive(Serialize, Deserialize)]
struct Resp {
    info: String
}


#[derive(Serialize, Deserialize, Debug)]
struct GenUrl {
    channel_url: String,
    url: String
}

#[actix_web::main]
async fn main() -> Result<()>{
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(NormalizePath::default()).wrap(cors).service(get_idk).service(get_id).route("/", web::get().to(root)).default_service(web::get().to(default_service))
    }).bind(("localhost", 8090)).unwrap().run().await.unwrap();
    Ok(())
}

#[get("/{id}")]
async fn get_idk(id: Path<String>) -> HttpResponse {
    match DB.get::<GenUrl>(&id.to_string()) {
        Ok(r) => match r {
            Some(e) => {
                HttpResponse::Ok().body(html_idk(e.channel_url, e.url))
            }
            None => HttpResponse::NotFound().json(Resp{info: "Sorry The Content You finding is NotFound In the Database!".into()})
        }
        Err(_) => HttpResponse::InternalServerError().json(Resp{info: "Sorry Error is in server!".into()})
    }
}

#[post("/gen-url")]
async fn get_id(req: Json<GenUrl>) -> HttpResponse {
    let uuid_gen: String = uuid::Uuid::new_v4().to_string().split("-").collect();
    match DB.insert(&uuid_gen, &req.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(Resp{info: uuid_gen}),
        Err(_) => HttpResponse::InternalServerError().json(Resp{info: "Sorry Error is in server!".into()})
    }
}

async fn root() -> HttpResponse {
    HttpResponse::Ok().body(root_html())
}

fn root_html() -> String {
r#"
<!DOCTYPE html>
<html>

<head>
  <title>Generate Link</title>
  <style>
    body {
      background-color: #000;
      color: #fff;
      font-family: Arial, sans-serif;
      text-align: center;
    }

    h1 {
      font-size: 36px;
      margin-bottom: 20px;
    }

    form {
      display: flex;
      flex-direction: column;
      align-items: center;
      margin-top: 50px;
    }

    label {
      font-size: 18px;
      margin-bottom: 10px;
    }

    input {
      width: 300px;
      height: 30px;
      font-size: 16px;
      padding: 5px;
      margin-bottom: 20px;
      border-radius: 5px;
      border: none;
    }

    button {
      width: 200px;
      height: 40px;
      font-size: 18px;
      background-color: #4CAF50;
      color: #fff;
      border-radius: 5px;
      border: none;
      cursor: pointer;
      transition: background-color 0.3s;
    }

    button:hover {
      background-color: #45a049;
    }

    .invalid {
      border: 2px solid red;
    }
  </style>
</head>

<body>
  <h1>Generate Link</h1>

  <form id="linkForm">
    <label for="channelUrl">YouTube Channel Link:</label>
    <input type="text" id="channelUrl" required>

    <label for="otherLink">Other Link:</label>
    <input type="text" id="otherLink" required>

    <button type="button" onclick="generateLink()" id="generateButton" disabled>Generate Link</button>
  </form>

  <div id="result" style="display: none;">
    <h2>Generated Link:</h2>
    <p id="generatedLink"></p>
  </div>

  <script>
    var channelUrlInput = document.getElementById("channelUrl");
    var otherLinkInput = document.getElementById("otherLink");
    var generateButton = document.getElementById("generateButton");

    // Add input event listeners to both inputs
    channelUrlInput.addEventListener("input", validateInput);
    otherLinkInput.addEventListener("input", validateInput);

    function validateInput() {
      var channelUrl = channelUrlInput.value;
      var otherLink = otherLinkInput.value;
      var isValid = isValidUrl(channelUrl) && isValidUrl(otherLink);

      // Add or remove the "invalid" class based on input validity
      if (isValid) {
        channelUrlInput.classList.remove("invalid");
        otherLinkInput.classList.remove("invalid");
        generateButton.disabled = false; // Enable the button
      } else {
        channelUrlInput.classList.add("invalid");
        otherLinkInput.classList.add("invalid");
        generateButton.disabled = true; // Disable the button
      }
    }

    function isValidUrl(text) {
      var urlRegex = /^(https?:\/\/)?((([a-z\d]([a-z\d-]*[a-z\d])*)\.)+[a-z]{2,}|((\d{1,3}\.){3}\d{1,3}))(\/[-a-z\d%_.~+]*)*(\?[;&a-z\d%_.~+=-]*)?(#[-a-z\d_]*)?$/i;
      return urlRegex.test(text);
    }

    function generateLink() {
      var channelUrl = channelUrlInput.value;
      var otherLink = otherLinkInput.value;

      // Create the JSON payload
      var payload = {
        channel_url: channelUrl,
        url: otherLink
      };

      // Make the request
      fetch('http://localhost:8090/gen-url', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
      })
        .then(response => response.json())
        .then(data => {
          // Extract the number from the response
          var num = data.info;

          // Display the generated link
          var generatedLink = document.getElementById("generatedLink");
          generatedLink.textContent = "http://localhost:8090/" + num;

          // Show the result section
          var resultSection = document.getElementById("result");
          resultSection.style.display = "block";
        })
        .catch(error => {
          console.error('Error:', error);
        });
    }
  </script>
</body>

</html>

"#.to_string()
}

fn html_idk(channel_url: String, url: String) -> String {
    format!(r#"<!DOCTYPE html>
<html>

<head>
  <title>Minecraft Subscription</title>
  <style>
    body {{
      background-color: #4c8eaf;
      font-family: 'Minecraft', sans-serif;
      text-align: center;
      margin: 0;
      padding: 20px;
    }}

    h1 {{
      color: #fff;
    }}

    .container {{
      max-width: 600px;
      margin: 0 auto;
    }}

    .text-field {{
      display: inline-block;
      width: 300px;
      padding: 10px;
      font-size: 18px;
      border: none;
      background-color: #fff;
      color: #333;
    }}

    .button {{
      display: inline-block;
      padding: 10px 20px;
      margin-top: 10px;
      font-size: 18px;
      font-weight: bold;
      border: none;
      background-color: #333;
      color: #fff;
      cursor: pointer;
    }}

    .button:disabled {{
      opacity: 0.5;
      cursor: not-allowed;
    }}
  </style>
</head>

<body>
  <div class="container">
    <h1>Subscribe To Unlock</h1>
    <br>
    <button class="button" id="button1" onclick="openURL()">Subscribe</button>
    <button class="button" id="button2" onclick="Bruh()" disabled>Unlock Link</button>
  </div>

  <script>
    function Bruh() {{
      var url = "{}";
      window.open(url, '_blank');
    }}

    function openURL() {{
      var url = "{}";
      window.open(url, '_blank');

      document.getElementById('button1').disabled = true;
      setTimeout(function () {{
        document.getElementById('button2').disabled = false;
        document.getElementById('button2').style.opacity = '1';
      }}, 10000);
    }}
  </script>
</body>

</html>"#, url, channel_url)
}


async fn default_service() -> HttpResponse {
    HttpResponse::NotFound().json(Resp{info: "The Content You finding is NotFound in Database!".into()})
}
