use std::fs;

fn main() {

    let contents = fs::read_to_string("test.md")
    .expect("failed to read");

    let mut html_output = String::from("");

    for line in contents.lines(){

        if line.starts_with("# ") {

            let heading_text  = line.strip_prefix("# ").unwrap();
            let html_line = format!("<h1>{}</h1>",heading_text);
            html_output.push_str(&html_line);

            // this adds a line if you dont know lol
            html_output.push_str("\n");



        }

        else{
                let heading_text  = line;
                let html_line = format!("<p>{}</p>",heading_text);
            html_output.push_str(&html_line)        }
    }

    fs::write("output.html", html_output).expect("lol dont expect nothin")
}
