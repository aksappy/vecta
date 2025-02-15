use std::{collections::HashMap, fs, path::Path};

use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::QueryParser,
    schema::{Schema, TextOptions, STORED, TEXT},
    Document, Index, IndexWriter, ReloadPolicy, SegmentId, TantivyDocument,
};
use walkdir::{DirEntry, WalkDir};

/// This will build schema for documents. By default, it includes fields for title and body.
/// You may customize the schema by providing a HashMap with field names and TextOptions.
/// See https://docs.rs/tantivy/latest/tantivy/schema/struct.TextOptions.html for more details.
pub fn build_doc_schema(schema_map: Option<HashMap<String, TextOptions>>) -> Schema {
    let mut schema_builder = Schema::builder();
    if let Some(map) = schema_map {
        for (field_name, options) in map {
            schema_builder.add_text_field(&field_name, options);
        }
    } else {
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT);
    }
    schema_builder.build()
}

fn create_index_if_not_exists(index_path: &str, schema: Schema) -> Index {
    let index = if !Path::new(index_path).exists() {
        // create a new directory if it doesn't exist
        fs::create_dir_all(index_path).unwrap();
        Index::create_in_dir(index_path, schema)
    } else {
        Index::open_in_dir(index_path)
    };
    match index {
        Ok(index) => index,
        Err(err) => panic!("Failed to create or open index: {}", err),
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn merge_indices() {
    let schema = build_doc_schema(None);
    let index = create_index_if_not_exists("./index", schema.clone());
    let mut index_writer: IndexWriter = index.writer(500_000_000).unwrap();
    // Get the list of all segments (you'd normally let Tantivy handle this)
    let segment_ids = index_writer
        .index()
        .searchable_segments()
        .unwrap()
        .into_iter()
        .map(|segment| segment.id())
        .map(|id| SegmentId::from(id))
        .collect::<Vec<SegmentId>>();

    // You may decide to merge the largest segments
    // In a real application, you might use your own logic to select segments
    // For this example, let's just merge all of them.
    index_writer.merge(segment_ids.as_slice());
}

/// Index a directory. All text files in the directory will be indexed, with contents. Other file names will be indexed.
pub fn index_directory(directory_path: String) {
    let schema = build_doc_schema(None);
    let index = create_index_if_not_exists("./index", schema.clone());
    // TODO take this from a global config
    let mut index_writer: IndexWriter = index.writer(50_000_000).unwrap();

    let walker = WalkDir::new(directory_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());
        if path.is_file() {
            let content = fs::read_to_string(&path);
            match content {
                Ok(content) => {
                    let title = path.canonicalize().unwrap().to_str().unwrap().to_string();
                    let title_field = schema.get_field("title").unwrap();
                    let body_field = schema.get_field("body").unwrap();
                    let result = index_writer.add_document(doc!(
                        title_field => title,
                        body_field => content.to_owned()
                    ));
                    match result {
                        Ok(_) => (),
                        Err(err) => panic!("Failed to add document: {}", err),
                    }
                }
                Err(err) => {
                    println!("cannot read contents of this type of file {:?}", path)
                }
            }
        }
    }
    index_writer.commit().unwrap();
}

pub fn read_index(search_string: String) {
    let schema = build_doc_schema(None);
    // TODO get this directory from the arguments
    let directory = MmapDirectory::open("./index").unwrap(); // Or FSDirectory::open(index_path)?
    let index = Index::open(directory).unwrap();
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .unwrap();
    let searcher = reader.searcher();
    let title_field = schema.get_field("title").unwrap();
    let body_field = schema.get_field("body").unwrap();
    let query_parser = QueryParser::for_index(&index, vec![title_field, body_field]);
    let query = query_parser.parse_query(search_string.as_str()).unwrap();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();
    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();
        println!("{}", retrieved_doc.to_json(&schema.clone()));
    }
}
