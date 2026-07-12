use std::fs;
use std::env;



fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: mdx <filename.mdx>");
            std::process::exit(1);

    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename)
    .expect("failed to read");

    //doing styling here lol
    // this looks messy but i hve got like no other choice not ai btw

    let mut html_output = String::from("<html>\n<head>\n<title>Converted Markdown</title>\n<style>\nbody {font-family:sans-serif; max-width:700px; margin: 40px auto; padding:0 20px; line-height:1.6}\ncode{background:#eee; padding:2px 6px; border-radius:4px;}\nblockquote{border-left:4px solid #ccc; padding-left:16px; color:#555}\nhr{border:none;border-top:1px solid #ccc;margin:24px 0}\n</style>\n</head>\n<body>\n");

    let mut in_list = false;

    for line in contents.lines(){

        if line.trim().is_empty() {
    continue;
}




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

        }


        }

        let final_line = if line.contains("**") {
    new_line
  } else {
    line.to_string()
   };

   // giving random names lol
   let mut newer_line = String::new(); 


           if final_line.contains("*"){

                    let parts: Vec<&str> = final_line.split("*").collect();
                            for (i, piece) in parts.iter().enumerate() {

                                if i % 2 == 1 {newer_line.push_str("<i>");
                                                newer_line.push_str(piece);
                                                newer_line.push_str("</i>");
                                }

                                else {
                                newer_line.push_str(piece)

                                }
                            }
                        }

                        let final_final_line = if final_line.contains("*") {

    newer_line
} else {
    final_line
};

let mut code_line = String::new();

if final_final_line.contains("`"){
        let parts: Vec<&str> = final_final_line.split("`").collect();

            for (i, piece) in parts.iter().enumerate() {

                        if i % 2 == 1 {
            code_line.push_str("<code>");
            code_line.push_str(piece);
            code_line.push_str("</code>");

            }
else {
            code_line.push_str(piece);
}
            }
}

let final_code_line = if final_final_line.contains("`") {
    code_line
} else {
    final_final_line
};

if final_code_line.starts_with("- "){
    if !in_list {
        html_output.push_str("<ul>\n");
               in_list = true;
    }

    let item_text = final_code_line.strip_prefix("- ").unwrap();
    let html_line = format!("<li>{}</li>", item_text);
        html_output.push_str(&html_line);
    html_output.push_str("\n");

}

else {
    if in_list {
                html_output.push_str("</ul>\n");
        in_list = false;
    }

            if final_code_line.starts_with("### ") {

            let heading_text  =final_code_line.strip_prefix("### ").unwrap();
            let html_line = format!("<h3>{}</h3>",heading_text);
            html_output.push_str(&html_line);

            // this adds anew_line if you dont know lol
            html_output.push_str("\n");
        }

                   else if final_code_line.starts_with("## ") {

            let heading_text  =final_code_line.strip_prefix("## ").unwrap();
            let html_line = format!("<h2>{}</h2>",heading_text);
            html_output.push_str(&html_line);

            // this adds anew_line if you dont know lol
            html_output.push_str("\n");
                   
                }
                   
       else if final_code_line.starts_with("# ") {

            let heading_text  =final_code_line.strip_prefix("# ").unwrap();
            let html_line = format!("<h1>{}</h1>",heading_text);
            html_output.push_str(&html_line);

            // this adds anew_line if you dont know lol
            html_output.push_str("\n");

        }

        else if final_code_line.starts_with("---") {
    html_output.push_str("<hr>\n");

}

    else if final_code_line.starts_with("> "){
            let quote_text = final_code_line.strip_prefix("> ").unwrap();
    let html_line = format!("<blockquote>{}</blockquote>", quote_text);
    html_output.push_str(&html_line);
    html_output.push_str("\n");


    }

        else{
                let heading_text  =final_code_line;
                let html_line = format!("<p>{}</p>",heading_text);
            html_output.push_str(&html_line);   
            html_output.push_str("\n");

        }
    }
}

// lol i am running out of line names


    html_output.push_str("\n</body>\n</html>");

    fs::write("output.html", html_output).expect("lol dont expect nothin")

}
    