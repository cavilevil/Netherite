use std::fs;
use std::io;
use std::fs::File;
use reqwest::blocking;
use std::path::PathBuf;
use std::error::Error;
use std::env;
use std::process::Command;

fn main() {
    // variables
    let username = std::env::var("USERNAME").unwrap();
    let mut core_choice = String::new();
    let mut folder_name_choice = String::new();
    let mut url = String::new();

    // loop in case the user enters something that is not an integer or not listed in available core choices
    loop {
        // used a newline for the list of available
        println!("What server software do you wish to use?\n Current options (CASE SENSITIVE!!): Vanilla, Paper, Forge"); // forge worken't
    
        let mut core_choice = String::new();
    
        io::stdin().read_line(&mut core_choice).expect("Failed to read!");
    
        let core_choice: i32 = match core_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // matches the user choice
        match core_choice {
            
            "Paper" => {
                url = "https://api.papermc.io/v2/projects/paper/versions/1.19.4/builds/519/downloads/paper-1.19.4-519.jar".to_string();
                break;
            }
            "Vanilla" => {
                url = "https://piston-data.mojang.com/v1/objects/8f3112a1049751cc472ec13e397eade5336ca7ae/server.jar".to_string();
                break;
            }
            "Forge" => {
                url = "https://maven.minecraftforge.net/net/minecraftforge/forge/1.19.4-45.0.49/forge-1.19.4-45.0.49-installer.jar".to_string(); // must run command, ARTEM
                break;
            }
            
            // user entered something that is not listed above
            _ => {
                println!("Invalid input, try again!");
                continue;
            }
        }
    }

    println!("Enter a name for your server folder: ");

    io::stdin()
        .read_line(&mut folder_name_choice)
        .expect("Failed to get the name for the folder!");

    let folder_name_choice = folder_name_choice.trim().to_lowercase();

    // makes a directory with user's Windows username, and a folder name that they've chosen
    let path = format!("C:\\Users\\{}\\Desktop\\{}", username, folder_name_choice);    

    // checks if an error has occured making a folder
    match fs::create_dir(&path){
        Ok(_) => println!("Folder created!"),
        Err(e) => println!("An error occured when creating the folder! {}", e)
    };

    download_required_files(&url, &path);
}

fn download_required_files(url: &String, download_folder: &String) -> Result<(), Box<dyn Error>> {
    // Create a GET request to download the file
    let mut response = reqwest::blocking::get(url)?;

    // Extract the filename from the URL
    let file_name = match url.rsplit_once("/") {
        Some((_, name)) => name,
        None => "file",
    };

    // Create the output file path
    let mut out_path = PathBuf::from(&download_folder);

    out_path.push(&file_name);

    // Create the output file and copy the contents of the response into it
    let mut out_file = File::create(&out_path)?;

    println!("Downloading {}", &file_name);
    
    io::copy(&mut response, &mut out_file)?;

    // creating a command that extracts the .jar file contents(java -jar doesnt extract files without jar xf)
    println!("Extracting the .jar file contents");
    let output = Command::new("jar")
        .current_dir(&download_folder)
        .arg("xf")
        .arg(format!("{}", &out_path.display()))
        .output()
        .expect("Failed to extract the contents of .jar file!");
    
    // creating a command that finishes the server setup
    println!("Finishing the server setup");
    let output = Command::new("java")
        .current_dir(&download_folder)
        .arg(format!("-jar"))
        .arg(format!("{}", &file_name))
        .output()
        .expect("Failed to setup the server files!");

    Ok(())

}
