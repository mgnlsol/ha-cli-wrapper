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
mod lib;
use lib::commands::{start_execute, call_command};
use lib::npm::npm_login;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let args: Vec<String> = args.drain(1..args.len()).collect();

    if args.len() > 0 {
        match args.get(0).unwrap().as_str() {
            "login" => match npm_login() {
                Ok(auth) => println!("Your token is: {}", auth.encode()),
                Err(error) => println!("Login failed {:#?}", error),
            },
            "self-update" => match call_command("npm", &vec![
                String::from("update"),
                String::from("-g"),
                String::from("@magnolia-dx/ha-cli"),
            ]) {
                Ok(_res) => println!("System updated"),
                Err(error) => println!("Login failed {:#?}", error),
            },
            _ => start_execute(&args),
        }
    } else {
        start_execute(&args);
    }
}
