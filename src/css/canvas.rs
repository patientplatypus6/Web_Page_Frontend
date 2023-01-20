use std::collections::HashMap;

pub fn styles() -> HashMap<String, String>{
    let mut style = HashMap::new();
    style.insert("main".to_string(), "
        display: flex;
        flex-direction: row;
        height: 100vh;
        background: lightgrey;
    ".to_string());
    style.insert("flexcontainer".to_string(),"
        display: flex;
        width: 100%;
        flex-direction: row;
    ".to_string());
    style.insert("flex1".to_string(),"
        flex: 1;
    ".to_string());
    style.insert("paintbarcontainer".to_string(),"
        flex: 1;
        background: darkgrey;
    ".to_string());
    style.insert("nav_item".to_string(), "font-size: 20px;".to_string());

    style
}

