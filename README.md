# scraper-extism

This is a plugin is intended to be used with the Extism plugin development kit (PDK).

## Features
- Strips all HTML tags and returns plain text content within a specified selector.
- Uses a [forked version](https://github.com/bradyjoslin/scraper) of rust `scraper` for HTML parsing
- Can be compiled to WebAssembly (WASM) for cross-platform usage.

## Usage

Here is an example of how to use the `scraper` function using [assembllm](https://github.com/bradyjoslin/assembllm/):

```yaml
name: "Scrape text prescript"
description: |
  Using a prescript, get the contents of a remote URL, then scrape the contents of a 
  specified selector using a WebAssembly module. Finally, calculate the percentage
  of characters saved by the scraping process.

tasks:
  - name: scrape
    pre_script: >
      let wasm = "https://github.com/bradyjoslin/scraper-extism/releases/latest/download/scraper_extism.wasm";
      let content = Get("https://bradyjoslin.com/blog/remote-vs-code/");
      let content_length = len(content);
      let params = (
        {
          "html": content, 
          "selector": "div.post-content"
        } 
        | toJSON()
      );
      let post_content = Extism(wasm, "scraper", params);
      let post_length = len(post_content);
      let percent_saved = ((content_length - post_length) / content_length * 100);
      "Characters reduced (percent): " + string(percent_saved)
    plugin: openai
    prompt: |
      echo exactly what i send you.  no omissions or modifications
```
