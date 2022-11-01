use crate::{error::Error, relative_path::RelativePath};
use clap::{command, Arg, Command};
use std::{collections::HashMap, path::PathBuf};
use toml::Value;

#[derive(Debug)]
pub enum Action {
    AddHelper {
        profile: RelativePath,
        helper: RelativePath,
    },
    AddModule {
        profile: RelativePath,
        module: RelativePath,
    },
    AddTemplate {
        profile: RelativePath,
        template: RelativePath,
    },
    ApplyProfile {
        profile: RelativePath,
    },
    ChangeSetting {
        profile: RelativePath,
        name: String,
        value: Option<Value>,
    },
    Nothing,
    Restore,
    RmHelper {
        profile: RelativePath,
        helper: RelativePath,
    },
    RmModule {
        profile: RelativePath,
        module: RelativePath,
    },
    RmTemplate {
        profile: RelativePath,
        template: RelativePath,
    },
}

pub fn main() -> Result<Action, Error> {
    let matches = cli().get_matches();

    Ok(match matches.subcommand() {
        Some(("apply", matches)) => {
            let profile = PathBuf::from(
                matches
                    .get_one::<String>("profile")
                    .expect("profile is required"),
            )
            .into();

            Action::ApplyProfile { profile }
        }

        Some(("helper", matches)) => match matches.subcommand() {
            Some(("add", matches)) => {
                let profile = PathBuf::from(
                    matches
                        .get_one::<String>("profile")
                        .expect("profile is required"),
                )
                .into();

                let helper = PathBuf::from(
                    matches
                        .get_one::<String>("helper")
                        .expect("helper is required"),
                )
                .into();

                Action::AddHelper { profile, helper }
            }

            Some(("remove", matches)) => {
                let profile = PathBuf::from(
                    matches
                        .get_one::<String>("profile")
                        .expect("profile is required"),
                )
                .into();

                let helper = PathBuf::from(
                    matches
                        .get_one::<String>("helper")
                        .expect("helper is required"),
                )
                .into();

                Action::RmHelper { profile, helper }
            }
            _ => Action::Nothing,
        },

        Some(("module", matches)) => match matches.subcommand() {
            Some(("add", matches)) => {
                let profile = PathBuf::from(
                    matches
                        .get_one::<String>("profile")
                        .expect("profile is required"),
                )
                .into();

                let module = PathBuf::from(
                    matches
                        .get_one::<String>("module")
                        .expect("module is required"),
                )
                .into();

                Action::AddModule { profile, module }
            }

            Some(("remove", matches)) => {
                let profile = PathBuf::from(
                    matches
                        .get_one::<String>("profile")
                        .expect("profile is required"),
                )
                .into();

                let module = PathBuf::from(
                    matches
                        .get_one::<String>("module")
                        .expect("module is required"),
                )
                .into();

                Action::RmModule { profile, module }
            }
            _ => Action::Nothing,
        },

        Some(("restore", _)) => Action::Restore,

        Some(("settings", matches)) => match matches.subcommand() {
            Some(("set", matches)) => {
                let profile = PathBuf::from(
                    matches
                        .get_one::<String>("profile")
                        .expect("profile is required"),
                )
                .into();

                let name = matches
                    .get_one::<String>("name")
                    .expect("setting name is required")
                    .to_owned();

                // FIXME: this is *really* bad, find some way to parse TOML without a table
                let value = Some(toml::from_str::<'_, HashMap<String, Value>>(&format!(
                    "data = {:?}",
                    matches
                        .get_one::<String>("value")
                        .expect("setting value is required")
                ))?)
                .map(|map| {
                    map.values()
                        .next()
                        .expect("setting value is required")
                        .to_owned()
                });

                Action::ChangeSetting {
                    profile,
                    name,
                    value,
                }
            }

            Some(("remove", matches)) => {
                let profile = PathBuf::from(
                    matches
                        .get_one::<String>("profile")
                        .expect("profile is required"),
                )
                .into();

                let name = matches
                    .get_one::<String>("name")
                    .expect("setting name is required")
                    .to_owned();

                Action::ChangeSetting {
                    profile,
                    name,
                    value: None,
                }
            }

            _ => Action::Nothing,
        },

        Some(("template", matches)) => match matches.subcommand() {
            Some(("add", matches)) => {
                let profile = PathBuf::from(
                    matches
                        .get_one::<String>("profile")
                        .expect("profile is required"),
                )
                .into();

                let template = PathBuf::from(
                    matches
                        .get_one::<String>("template")
                        .expect("template is required"),
                )
                .into();

                Action::AddTemplate { profile, template }
            }

            Some(("remove", matches)) => {
                let profile = PathBuf::from(
                    matches
                        .get_one::<String>("profile")
                        .expect("profile is required"),
                )
                .into();

                let template = PathBuf::from(
                    matches
                        .get_one::<String>("template")
                        .expect("template is required"),
                )
                .into();

                Action::RmTemplate { profile, template }
            }
            _ => Action::Nothing,
        },

        _ => Action::Nothing,
    })
}

fn cli() -> Command {
    command!()
        .subcommand(
            Command::new("apply")
                .about("Apply a given profile")
                .alias("a")
                .arg(
                    Arg::new("profile")
                        .help("The profile to apply")
                        .value_name("PROFILE")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("helper")
                .about("Add or remove helpers from a given profile")
                .alias("h")
                .subcommand(
                    Command::new("add")
                        .about("Add a helper for a given profile")
                        .alias("a")
                        .arg(
                            Arg::new("helper")
                                .help("The helper to add")
                                .value_name("HELPER")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("profile")
                                .help("The profile to add to")
                                .value_name("PROFILE")
                                .index(2)
                                .required(true),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a helper for a given profile")
                        .aliases(&["rm", "r"])
                        .arg(
                            Arg::new("helper")
                                .help("The helper to remove")
                                .value_name("HELPER")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("profile")
                                .help("The profile to remove from")
                                .value_name("PROFILE")
                                .index(2)
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("module")
                .about("Add or remove modules from a given profile")
                .aliases(&["mod", "m"])
                .subcommand(
                    Command::new("add")
                        .about("Add a module for a given profile")
                        .alias("a")
                        .arg(
                            Arg::new("module")
                                .help("The module to add")
                                .value_name("MODULE")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("profile")
                                .help("The profile to add to")
                                .value_name("PROFILE")
                                .index(2)
                                .required(true),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a module for a given profile")
                        .aliases(&["rm", "r"])
                        .arg(
                            Arg::new("module")
                                .help("The module to remove")
                                .value_name("MODULE")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("profile")
                                .help("The profile to remove from")
                                .value_name("PROFILE")
                                .index(2)
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("restore")
                .about("Restore backed up config files")
                .alias("r"),
        )
        .subcommand(
            Command::new("settings")
                .about("Add, remove, or change settings from a given profile")
                .alias("s")
                .subcommand(
                    Command::new("set")
                        .about("Add or change a setting for a given profile")
                        .alias("s")
                        .arg(
                            Arg::new("name")
                                .help("The name of the setting to add or change")
                                .value_name("NAME")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("profile")
                                .help("The profile the setting belongs to")
                                .value_name("PROFILE")
                                .index(2)
                                .required(true),
                        )
                        .arg(
                            Arg::new("value")
                                .help("The (new) value of the setting")
                                .value_name("VALUE")
                                .index(3)
                                .required(true),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a setting from a given profile")
                        .aliases(&["rm", "r"])
                        .arg(
                            Arg::new("name")
                                .help("The setting to remove")
                                .value_name("NAME")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("profile")
                                .help("The profile to remove from")
                                .value_name("PROFILE")
                                .index(2)
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("template")
                .about("Add or remove templates from a given profile")
                .aliases(&["temp", "t"])
                .subcommand(
                    Command::new("add")
                        .about("Add a template for a given profile")
                        .alias("a")
                        .arg(
                            Arg::new("template")
                                .help("The template to add")
                                .value_name("TEMPLATE")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("profile")
                                .help("The profile to add to")
                                .value_name("PROFILE")
                                .index(2)
                                .required(true),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a template for a given profile")
                        .aliases(&["rm", "r"])
                        .arg(
                            Arg::new("template")
                                .help("The template to remove")
                                .value_name("TEMPLATE")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::new("profile")
                                .help("The profile to remove from")
                                .value_name("PROFILE")
                                .index(2)
                                .required(true),
                        ),
                ),
        )
}
