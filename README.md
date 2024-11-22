# mdbook-generate-summary

A Rust project that generates a summary for an mdBook project.

## Getting Started

These instructions will help you set up and run the project on your local machine for development and testing purposes.

### To install (only supported via Local Crate)
At the minute only supported locally not supported on crate.io.

1. Clone/Fork the repo. 
2. Install and build the repo. 
3. Install on the current mdbook you are working in. cargo install --path [local path url] 


### Considerations  

TO form the root directory files there will have to be a corresponding  read me file as the generator will presume that there is one provided. For now the summary generator only looks at the root src file. 

For example, 

```
project_root/
├── README.md
├── src/
│   ├── my_cool_folder.rs
│   ├── cool_folder/
│   │   ├── cool_stuff.rs
│   │   └── README.md
│   └── cool-stuff.rs
    ├── SUMMARY.md
    └── chapter_1.md
```

Will generate 

```
# Summary 
- [My Cool Folder](./my_cool_folder/README.rs)
    - [My Cool Folder](./my_cool_folder/cool_stuff.md)
- [More Cool Stuff](./more-cool-stuff.md)
- [Chapter 1](./Chapter_1.md)
```