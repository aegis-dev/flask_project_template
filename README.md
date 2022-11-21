# flask_project_template
Project template for [Flask game framework](https://github.com/aegis-dev/flask)

## Prerequisites
Install wasm-pack
https://rustwasm.github.io/wasm-pack/installer/

## Build
```
wasm-pack build --target web
```

## Running
You can server files with any static file server and here is a simple python example
```
# In root project directory
python -m http.server
```