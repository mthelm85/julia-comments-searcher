use std::env;
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::*,
    Index,
    directory::MmapDirectory,
    ReloadPolicy
};

fn main() -> tantivy::Result<()> {
    let search_string = env::args().nth(1).expect("no search term provided");
    let index_path = dirs::data_local_dir().unwrap().join("JuliaSearch");
    let index = Index::open(MmapDirectory::open(index_path).unwrap())?;
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;
    let searcher = reader.searcher();
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("file", TEXT | STORED);
    schema_builder.add_text_field("comments", TEXT | STORED);
    let schema = schema_builder.build();
    let file = schema.get_field("file").unwrap();
    let comments = schema.get_field("comments").unwrap();
    let query_parser = QueryParser::for_index(&index, vec![file, comments]);
    let query = query_parser.parse_query(&search_string)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        let fname = retrieved_doc.field_values()[0].value().text().unwrap();
        let parts = fname.split("\\").collect::<Vec<_>>();
        println!("File: {:#?}", parts[4..parts.len()].join("/"));
        println!("Comment: {:#?}", retrieved_doc.field_values()[1].value().text().unwrap());
        println!("");
    }
    Ok(())
}
