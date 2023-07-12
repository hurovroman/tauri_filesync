use crate::command::file::File;
use crate::command::search::search_files;
use crate::utils::CommandData;

/// filter file path for documents
static ACCEPTABLE_SUFFIXES: &[&str] = &[
    "ppt", "pot", "pps", "pptx", "pptm", "potx", "potm", "ppam", "ppsx", "ppsm", "sldx", "sldm",
    "odp", "fodp", "otp", "doc", "dot", "docx", "docm", "dotx", "dotm", "docb", "odt", "fodt",
    "ott", "ots", "xls", "xlt", "xlm", "xlsx", "xlsm", "xltx", "xltm", "xla", "xlam", "ods",
    "fods", "xml", "xslt", "html", "xhtml", "htm", "txt", "rtf", "c", "h", "cpp", "hpp", "cxx",
    "hxx", "java", "js", "rb", "py", "cs", "m", "sh", "php", "css", "go", "ps", "rs", "pdf",
];

// get the documents from the default documents dir of the OS
// return an instance of the CommandData and vector of the path if any
#[tauri::command]
pub fn fetch_documents() -> Result<CommandData<Vec<File>>, CommandData<()>> {
    // if there is an error getting the documents path, fire an error
    let document_dir = dirs::document_dir();
    let Some(document_dir) = document_dir else{
        return Err(CommandData::err("error getting the documents dir",  ()));
    };

    let entries = search_files("*", &document_dir)
        .into_iter()
        .filter(|f| ACCEPTABLE_SUFFIXES.contains(&f.file_format.as_str()))
        .collect();

    Ok(CommandData::ok("retrieved all documents", entries))
}

#[cfg(test)]
mod tests {
    use crate::command::documents::{fetch_documents, ACCEPTABLE_SUFFIXES};
    #[test] // see if there are files in the documents directory path
    fn _fetch_documents_() {
        let docs = fetch_documents().ok();
        assert!(docs.is_some());

        let files = docs.unwrap().data.unwrap();
        for file in files {
            let file_format = file.file_format;
            assert!(ACCEPTABLE_SUFFIXES.contains(&file_format.as_str()));
        }
    }
}
