use keyring::Entry;

const SERVICE: &str = "glucotray";

pub fn save_credentials(username: &str, password: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE, username)?;
    entry.set_password(password)?;
    Ok(())
}

pub fn get_password(username: &str) -> Result<String, keyring::Error> {
    let entry = Entry::new(SERVICE, username)?;
    entry.get_password()
}

pub fn delete_credentials(username: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE, username)?;
    entry.delete_credential()?;
    Ok(())
}