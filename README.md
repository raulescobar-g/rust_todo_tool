# TODO COLLECTOR
## Small tool that crawls directory and collects 'TODO' type comments. Comments get put in a .txt file.

- Supported languages
    - rust
    - cpp
    - c
    - java
    - javascript
    - python

- Flags
    - -s => does not output anything to console, instead only outputs to the .txt file
    - -g => by default script ignores files in your .gitignore, but this flag will disable that
    - -k {KEYWORD} => by default scirpt looks for TODO,FIXME,XXX keywords, but this flag allows you to specify another keyword you want the crawler to look for, currently only takes in 1 custom keyword
    - -r {/path/to/file} => by default crawls the directory you are currently in, but this flag allows you to input another directory to crawl (output file still goes in to your current directory)


