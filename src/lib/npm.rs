use super::auth::{AuthenticationError, Credentials};
/**
 * This file Copyright (c) 2010-2022 Magnolia International
 * Ltd.  (http://www.magnolia-cms.com). All rights reserved.
 *
 *
 * This program and the accompanying materials are made
 * available under the terms of the Magnolia Network Agreement
 * which accompanies this distribution, and is available at
 * http://www.magnolia-cms.com/mna.html
 *
 * Any modifications to this file must keep this entire header
 * intact.
 *
 */
use dialoguer::Password;
use dirs::home_dir;
use std::env;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

/// Prompts user for username and password credentials
/// and is updating .npmrc file in users home directory with nexus credentials
///
/// # Example
/// let auth: Credentials = npm_login();
/// println!("your credentials are {#:?}", auth);
///
/// # .npmrc file format
/// ```
/// // Magnolia DX private repisotory
/// @magnolia-ea:registry=https://npm.magnolia-cms.com/repository/npm-enterprise/
/// //npm.magnolia-cms.com/repository/npm-enterprise/:always-auth=true
/// //npm.magnolia-cms.com/repository/npm-enterprise/:_auth=YXNkZmFzZGY6YWxza2RmamFzZGY=
/// ```
fn gather_credentials() -> Result<Credentials, AuthenticationError> {
    let npm_repo = "https://npm.magnolia-cms.com";

    match env::var("MGNL_HA_AUTH_TOKEN") {
        Ok(token) => {
            return Ok(Credentials {
                username: "TOKEN".to_string(),
                password: token,
            })
        }
        Err(_e) => println!(" HA has to download all depenencies first.\n Please login withyour Nexus / Magnolia DX Core Credentials."),
    };

    let mut line = String::new();
    print!(" Magnolia Username: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    let username = line.trim().to_string();

    let password = Password::new()
        .with_prompt(" Magnolia Password")
        .interact()
        .unwrap();

    let result = Credentials { username, password };

    let test_url = format!("{}/service/rest/v1/search", npm_repo);

    let client = reqwest::blocking::Client::new();

    let loggedin = match client
        .get(test_url)
        .basic_auth(&result.username, Some(&result.password))
        .send()
    {
        Ok(_result) => _result.status() != 401,
        Err(_error) => false,
    };

    if !loggedin {
        return Err(AuthenticationError {
            message: String::from("You credentials are wrong, cannot login"),
        });
    }

    Ok(result)
}

pub fn npm_login() -> Result<Credentials, AuthenticationError> {
    let result: Credentials = gather_credentials()?;
    let encodes_credentials = result.encode();
    let mut hd = home_dir().unwrap();
    hd.push(".npmrc");

    let target_path = hd.to_str().unwrap();

    if npm_exists(target_path) {
        return Ok(result);
    }

    let npmrc = format!(
        r#"

// Magnolia DX private repisotory
@magnolia-dx:registry=https://npm.magnolia-cms.com/repository/npm-enterprise/
//npm.magnolia-cms.com/repository/npm-enterprise/:always-auth=true
//npm.magnolia-cms.com/repository/npm-enterprise/:_auth={}

// Magnolia DX private repisotory
@magnolia-ea:registry=https://npm.magnolia-cms.com/repository/npm-enterprise/
//npm.magnolia-cms.com/repository/npm-enterprise/:always-auth=true
//npm.magnolia-cms.com/repository/npm-enterprise/:_auth={}
"#,
        encodes_credentials, encodes_credentials
    );

    println!("Updated NPM settings at {:#?}", target_path);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(target_path)
        .unwrap();

    match file.write_all(npmrc.as_bytes()) {
        Ok(_res) => println!("Successfull authenticated and system is setup."),
        Err(error) => panic!("Error setting system up {:#?}", error),
    }

    Ok(result)
}

fn npm_exists(npmfile: &str) -> bool {
    println!("npm file exists");
    let content = match read_to_string(npmfile) {
        Ok(file_content) => file_content,
        Err(_error) => return false,
    };

    check_string(&content, "@magnolia-dx") && check_string(&content, "@magnolia-ea")
}

fn check_string(content: &str, search_string: &str) -> bool {
    match content.find(search_string) {
        Some(_position) => true,
        None => false,
    }
}
