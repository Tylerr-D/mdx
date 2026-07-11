use std::fs;

fn main() {

    let contents = fs::read_to_string("test.md")
    .expect("failed to read");

    let mut html_output = String::from("<html>\n<body>\n");

    for line in contents.lines(){

        let mut new_line = String::new();

        if line.contains("**"){
            // advanced rust lol
                    let parts: Vec<&str> = line.split("**").collect();
                            for (i, piece) in parts.iter().enumerate() {

                                if i % 2 == 1 {new_line.push_str("<b>");
                                                new_line.push_str(piece);
                                                new_line.push_str("</b>");
                                }

                                else {
                                new_line.push_str(piece)

                                }

            println!("piece {}: '{}'", i, piece);
        }


        }

        let final_line = if line.contains("**") {
    new_line
  } else {
    line.to_string()
   };

        if final_line.starts_with("# ") {

            let heading_text  =final_line.strip_prefix("# ").unwrap();
            let html_line = format!("<h1>{}</h1>",heading_text);
            html_output.push_str(&html_line);

            // this adds anew_line if you dont know lol
            html_output.push_str("\n");



        }

        else{
                let heading_text  =final_line;
                let html_line = format!("<p>{}</p>",heading_text);
            html_output.push_str(&html_line)        }
    }
    html_output.push_str("\n</body>\n</html>");

    fs::write("output.html", html_output).expect("lol dont expect nothin")

}
