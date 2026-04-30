fn main() {
    embed_resource::compile("oro-manifest.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
