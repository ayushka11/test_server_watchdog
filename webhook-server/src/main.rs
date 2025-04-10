use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use serde::Deserialize;
use reqwest::header::{ACCEPT, USER_AGENT};
use regex::Regex;
use std::env;
use std::process::Command;
use base64::decode;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
struct PullRequest {
    number: u32,
    merged: bool,
}

#[derive(Deserialize, Debug)]
struct Sender {
    login: String,
}

#[derive(Deserialize, Debug)]
struct WorkflowRun {
    conclusion: String,
}

#[derive(Deserialize, Debug)]
struct Payload {
    action: String,
    workflow_run: WorkflowRun,
}

#[post("/webhook")]
async fn webhook_handler(payload: web::Json<Payload>) -> impl Responder {
    let payload = payload.into_inner();

    // println!("Received webhook: {:?}", payload);

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    if payload.action == "completed" && payload.workflow_run.conclusion == "success" {
        println!(
            "Received webhook",
        );

        let github_owner = "ayushka11";
        let github_repo = "test";
        // let github_token = env::var("GITHUB_TOKEN").ok();

        // fetch commit from merge_commit.txt
        let mut file = std::fs::File::open("merge_commit.txt").unwrap();
        let mut merge_commit = String::new();
        std::io::Read::read_to_string(&mut file, &mut merge_commit).unwrap();
        merge_commit = merge_commit.trim().to_string();
        // println!("Merge commit from file: {}", merge_commit);

        let client = reqwest::Client::new();
        let commits_url = format!(
            "https://api.github.com/repos/{}/{}/commits?sha=build&per_page=2",
            github_owner, github_repo
        );

        let commit_response = client
            .get(&commits_url)
            .bearer_auth(github_token.clone())
            .header(USER_AGENT, "rust-webhook-server")
            .send()
            .await;

        let commits = match commit_response {
            Ok(resp) => resp.json::<serde_json::Value>().await.unwrap(),
            Err(e) => return HttpResponse::InternalServerError().body(format!("GitHub error: {}", e)),
        };

        let arr = commits.as_array().unwrap();
        if arr.len() < 2 {
            return HttpResponse::BadRequest().json("Not enough commits to compare");
        }

        let merge_commit = arr[0]["sha"].as_str().unwrap();
        let mut base_commit = arr[1]["sha"].as_str().unwrap();
        // base_commit = "a23359ccb0f9e96966f80cc17615d53b6aa9ecec";

        println!("Merge commit: {}", merge_commit);
        println!("Base commit: {}", base_commit);

        // store the merge commit in a file
        let mut file = std::fs::File::create("merge_commit.txt").unwrap();
        std::io::Write::write_all(&mut file, merge_commit.as_bytes()).unwrap();

        let diff_url = format!(
            "https://api.github.com/repos/{}/{}/compare/{}...{}",
            github_owner, github_repo, base_commit, merge_commit
        );

        let diff_resp = client
            .get(&diff_url)
            .header(USER_AGENT, "rust-webhook-server")
            .header(ACCEPT, "application/vnd.github.v3.diff")
            .send()
            .await
            .unwrap();

        let diff_data = diff_resp.text().await.unwrap();
        println!("Received PR Diff:\n{}", diff_data);

        let re = Regex::new(r"access/([^/]+)/([^/]+)/([\w\d]+)").unwrap();
        if let Some(caps) = re.captures(&diff_data) {
            let project = caps.get(1).unwrap().as_str();
            let cloud_provider = caps.get(2).unwrap().as_str();
            let hash = caps.get(3).unwrap().as_str();

            println!("Project: {}", project);
            println!("Cloud Provider: {}", cloud_provider);
            println!("Hash: {}", hash);

            let file_url = format!(
                "https://api.github.com/repos/{}/{}/contents/names/{}?ref=build",
                github_owner, github_repo, hash
            );

            let file_resp = client
                .get(&file_url)
                .header(USER_AGENT, "rust-webhook-server")
                .header(ACCEPT, "application/vnd.github.v3+json")
                .send()
                .await
                .unwrap();

            let file_json = file_resp.json::<serde_json::Value>().await.unwrap();

            if let Some(base64_content) = file_json["content"].as_str() {
                let clean_base64 = base64_content.replace("\n", "");
                let decoded = decode(clean_base64).unwrap();
                let decoded_str = String::from_utf8(decoded).unwrap();

                println!("Decoded File Content: {}", decoded_str);

                // Run command
                // add_user_to_group(&decoded_str.trim(), cloud_provider);
            }
        }

        return HttpResponse::Ok().json("Processed diff successfully.");
    }

    HttpResponse::Ok().json("No action taken.")
}

// test for checking group REMOVE for original functioning

use serde::Serialize;

#[derive(Deserialize)]
struct GroupRequest {
    user: String,
    group: String,
}

#[derive(Serialize)]
struct GroupResponse {
    status: String,
    stderr: String,
    stdout: String,
}

#[post("/test-group")]
async fn test_group_handler(req: web::Json<GroupRequest>) -> impl Responder {
    let user = req.user.trim();
    let group = req.group.trim();

    println!("Attempting to add user '{}' to group '{}'", user, group);

    add_user_to_group(user, group);

    HttpResponse::Ok().json("User added to group successfully.")
}

// test code ends

#[allow(dead_code)]
fn add_user_to_group(user: &str, group: &str) {
    let check_user = Command::new("id")
        .arg(user)
        .output()
        .expect("Failed to run id command");

    if !check_user.status.success() {
        println!("User {} does not exist. Creating user...", user);
        let create_user = Command::new("sudo")
            .arg("useradd")
            .arg(user)
            .output()
            .expect("Failed to create user");

        if !create_user.status.success() {
            eprintln!("Failed to create user: {:?}", String::from_utf8_lossy(&create_user.stderr));
            return;
        }
    }

    let output = Command::new("sudo")
        .arg("usermod")
        .arg("-aG")
        .arg(group)
        .arg(user)
        .output()
        .expect("Failed to run usermod");

    if output.status.success() {
        println!("User {} added to group {}", user, group);
    } else {
        eprintln!("Error: {:?}", String::from_utf8_lossy(&output.stderr));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("2000".to_string());

    HttpServer::new(|| {
        App::new()
            .service(webhook_handler)
            .service(test_group_handler) // include both services
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
