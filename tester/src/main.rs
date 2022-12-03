use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executables = fs::read_dir("target/debug/")?;

    let mut error = false;

    for file in executables {
        let file = file?;
        let path = file.path();
        if let Some(name) = path.file_name() {
            let metadata = file.metadata()?;

            let permissions = metadata.permissions().mode();

            if file.file_type()?.is_file() && permissions & 0o111 != 0 && name != "tester" {
                // file is executable

                let mut answer_path = PathBuf::new();
                answer_path.push(&name);
                answer_path.push("answer");

                let answer = fs::read_to_string(answer_path)?;

                let output = Command::new(&path).output()?;

                let stdout = String::from_utf8_lossy(&output.stdout);

                let split_stdout = stdout.split("\n").collect::<Vec<_>>();

                for (i, part) in answer.split("\n").enumerate() {
                    if part != split_stdout[i] {
                        eprintln!("Invalid answer to question {} part {}\nCorrect answer: {}\nProvided answer: {}", name.to_string_lossy(), i + 1, part, split_stdout[i]);
                        error = true;
                    } else {
                        println!(
                            "Correct answer to question {} part {}",
                            name.to_string_lossy(),
                            i + 1
                        );
                    }
                }
            }
        }
    }

    if error {
        Err("Invalid answer to one or more of the questions.")?
    } else {
        Ok(())
    }
}
