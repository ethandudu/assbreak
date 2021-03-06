//local
use super::func::*;
use super::config::*;
use super::brute::brute_force;

pub fn input_text_password_submit() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    print!("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ");
    let chrome = read_input();
    let mut bool_chrome: bool;
    if chrome == "yes" {
        bool_chrome = true;
        println!("[!] The program will show you chrome and its brute-force operation");
    } else {
        bool_chrome = false;
        println!("[!] You will not see chrome however you will be able to see the logs.");
    }

    default_config(website, username, path_file, bool_chrome);
}

pub fn input_email_password_submit() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    print!("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ");
    let chrome = read_input();
    let mut bool_chrome: bool;
    if chrome == "yes" {
        bool_chrome = true;
        println!("[!] The program will show you chrome and its brute-force operation");
    } else {
        bool_chrome = false;
        println!("[!] You will not see chrome however you will be able to see the logs.");
    }

    config_field_email(website, username, path_file, bool_chrome);
}
pub fn input_text_password_button() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    print!("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ");
    let chrome = read_input();
    let mut bool_chrome: bool;
    if chrome == "yes" {
        bool_chrome = true;
        println!("[!] The program will show you chrome and its brute-force operation");
    } else {
        bool_chrome = false;
        println!("[!] You will not see chrome however you will be able to see the logs.");
    }

    config_field_button(website, username, path_file, bool_chrome);
}
pub fn input_email_password_button() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    print!("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ");
    let chrome = read_input();
    let mut bool_chrome: bool;
    if chrome == "yes" {
        bool_chrome = true;
        println!("[!] The program will show you chrome and its brute-force operation");
    } else {
        bool_chrome = false;
        println!("[!] You will not see chrome however you will be able to see the logs.");
    }

    config_field_email_button(website, username, path_file, bool_chrome);
}

pub fn default() {
    // website for bruteforce
    print!("[~] The site you want to try brute-force : ");
    let website = read_input();

    // input username
    print!("[~] Enter the username selector : ");
    let username_input = read_input();

    // input password
    print!("[~] Enter the password selector : ");
    let password_input = read_input();

    // input button
    print!("[~] Enter the button (login) selector : ");
    let form_button = read_input();

    // username to crack
    print!("[~] Enter the username to brute-force : ");
    let username = read_input();

    // dictionary_password
    print!("[~] Enter the path of the password dictionary : ");
    let path_file = read_input();

    print!("[~] Do you want to see chrome in the middle of a brute force operation (yes or no) ? ");
    let chrome = read_input();
    let mut bool_chrome: bool;
    if chrome == "yes" {
        bool_chrome = true;
        println!("[!] The program will show you chrome and its brute-force operation");
    } else {
        bool_chrome = false;
        println!("[!] You will not see chrome however you will be able to see the logs.");
    }

    brute_force(website, username_input, password_input, form_button, username, path_file, bool_chrome);
}