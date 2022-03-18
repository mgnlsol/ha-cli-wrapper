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
use super::npm::npm_login;
use std::io::ErrorKind;
use std::process::Command;

#[derive(std::cmp::PartialEq, Debug)]
pub enum CommandResultCode {
    Ok,
    NotInstalled,
    Error,
}

#[derive(Debug)]
pub struct CommandResult {
    pub message: String,
    pub code: CommandResultCode,
}

pub fn start_execute(args: &[String]) {
    match call_ha(args) {
        Ok(_result) => println!(),
        Err(error) => {
            if error.code == CommandResultCode::NotInstalled {
                let loginok = match npm_login() {
                    Ok(_auth) => true,
                    Err(_error) => false,
                };

                if !loginok {
                    println!("Authentication failed");
                    return;
                }
                match install_ha_cli() {
                    Ok(_result) => println!("ha-cli installed successfully"),
                    Err(error) => println!("Couldn't install ha cli {:#?}", error.message),
                }
            } else {
                println!("Error {:#?}", error);
            }
        }
    }
}

pub fn call_ha(args: &[String]) -> Result<CommandResult, CommandResult> {
    call_command("ha-cli", args)
}

pub fn install_ha_cli() -> Result<CommandResult, CommandResult> {
    let args: Vec<String> = vec![
        String::from("install"),
        String::from("@magnolia-dx/ha-cli"),
        String::from("-g"),
    ];
    call_command("npm", &args)
}

pub fn call_command(cmd: &str, args: &[String]) -> Result<CommandResult, CommandResult> {
    let status = Command::new(cmd).args(args).status();

    match status {
        Ok(res) => match res.code() {
            Some(0) => Ok(CommandResult {
                message: format!("Successfully called command {}", cmd),
                code: CommandResultCode::Ok,
            }),
            _ => Err(CommandResult {
                message: format!("Unkown Error. Couln't execute command {}", cmd),
                code: CommandResultCode::Error,
            }),
        },
        Err(res) => match res.kind() {
            ErrorKind::NotFound => Err(CommandResult {
                message: format!("Command >{}< not found", cmd),
                code: CommandResultCode::NotInstalled,
            }),
            _ => Err(CommandResult {
                message: format!("Unkown Error. Couln't execute command {}", cmd),
                code: CommandResultCode::Error,
            }),
        },
    }
}
