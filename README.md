# rs-python-app-template

Webapp using rust backend, and python controlling the front-end.

release builds will hide the console window

Comment out the first line in main.rs to hide the window after gui has started, rather than never starting a console window.


The exe has a rust icon by default, you can change this in build.rs, or by replacing the .ico file in assets.


The gui's icon is set in app.py (it is not from a local file)


The entire interface is all in app.py
rust.py is purely for ides, and has no affect to the application.