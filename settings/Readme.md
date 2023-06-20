# Setup

## Link the file
- copy the schema file into the schemas folder
    ``` 
    mkdir -p $HOME/.local/share/glib-2.0/schemas/
    ```
    ```
    cp src/org.gtk_rs.Settings.gschema.xml $HOME/.local/share/glib-2.0/schemas/
    ```
- link the file with our application
    ```
    glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
    ```