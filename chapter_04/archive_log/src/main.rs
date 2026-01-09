fn main() {
    let mut primary = String::from("Log Entry 1; ");
    let mut archive = String::from("Old Logs: ");

    archive_log(&mut primary, &mut archive);
    println!("{primary} | {archive}")
}

fn archive_log(primary: &mut String, archive: &mut String) {
    archive.push_str(primary.as_str());
    primary.clear();
}
