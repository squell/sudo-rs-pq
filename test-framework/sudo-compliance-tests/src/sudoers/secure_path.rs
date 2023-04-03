use sudo_test::{Command, Env, TextFile};

use crate::{Result, SUDOERS_ALL_ALL_NOPASSWD};

#[test]
fn if_unset_searches_program_in_invoking_users_path() -> Result<()> {
    let path = "/root/my-script";
    let env = Env(SUDOERS_ALL_ALL_NOPASSWD)
        .file(path, TextFile("#!/bin/sh").chmod("100"))
        .build()?;

    Command::new("sh")
        .args(["-c", "export PATH=/root; cd /; /usr/bin/sudo my-script"])
        .exec(&env)?
        .assert_success()?;

    Ok(())
}

#[test]
fn if_set_searches_program_in_secure_path() -> Result<()> {
    let path = "/root/my-script";
    let env = Env("\
Defaults secure_path=.:/root
ALL ALL=(ALL:ALL) NOPASSWD: ALL")
    .file(path, TextFile("#!/bin/sh").chmod("100"))
    .build()?;

    let match_in_relative_path_when_path_is_unset = "unset PATH; cd /bin; /usr/bin/sudo true";
    let match_in_absolute_path_when_path_is_unset = "unset PATH; cd /; /usr/bin/sudo my-script";
    // `true` is in `/bin/true`
    let match_in_relative_path_when_path_is_set = "export PATH=/usr/bin; cd /bin; sudo true";
    // default PATH does not include `/root`
    let match_in_absolute_path_when_path_is_set = "cd /; /usr/bin/sudo my-script";

    let scripts = [
        match_in_relative_path_when_path_is_unset,
        match_in_absolute_path_when_path_is_unset,
        match_in_relative_path_when_path_is_set,
        match_in_absolute_path_when_path_is_set,
    ];

    for script in scripts {
        println!("{script}");

        Command::new("sh")
            .args(["-c", script])
            .exec(&env)?
            .assert_success()?;
    }

    Ok(())
}

#[test]
fn if_set_it_does_not_search_in_original_user_path() -> Result<()> {
    let env = Env("\
        Defaults secure_path=/root
ALL ALL=(ALL:ALL) NOPASSWD: ALL")
    .build()?;

    let output = Command::new("sudo").arg("true").exec(&env)?;

    assert!(!output.status().success());
    assert_eq!(Some(1), output.status().code());

    if sudo_test::is_original_sudo() {
        assert_contains!(output.stderr(), "sudo: true: command not found");
    }

    Ok(())
}

#[ignore]
#[test]
fn if_set_it_becomes_the_path_set_for_program_execution() -> Result<()> {
    let secure_path = ".:/root";
    let env = Env(format!(
        "Defaults secure_path={secure_path}
ALL ALL=(ALL:ALL) NOPASSWD: ALL"
    ))
    .build()?;

    let user_path_set = "cd /; sudo /usr/bin/printenv PATH";
    let user_path_unset = "unset PATH; cd /; /usr/bin/sudo /usr/bin/printenv PATH";
    let scripts = [user_path_set, user_path_unset];

    for script in scripts {
        println!("{script}");

        let path = Command::new("sh")
            .args(["-c", script])
            .exec(&env)?
            .stdout()?;

        assert_eq!(secure_path, &path);
    }

    Ok(())
}
