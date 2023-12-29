use rocket::response::Redirect;

#[macro_use]
extern crate rocket;

mod github;
mod google;
mod twitter;
mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "tw" => twitter::construct_twitter_url(&cmd),
        "gh" => github::construct_github_url(&cmd),
        _ => google::construct_google_search_url(&cmd),
    };
    Redirect::to(redirect_url)
}
