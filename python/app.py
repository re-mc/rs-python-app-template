"""
Rust / Python App Template
Author: AitherNight
"""

import rust
import os, sys
from time import sleep

# Import flexx (library used for the gui), if it isn't installed, install it
try:
    from flexx import flx
except ModuleNotFoundError:
    print('Flexx not found, installing...')
    if sys.platform not in ['linux', 'macos', 'other']:
        print('Windows detected.')
    else:
        print('Linux detected.')
        os.system(f'python3 -m pip install flexx')
        exit(0)


flx.assets.associate_asset(__name__, 'https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/css/materialize.min.css')
flx.assets.associate_asset(__name__, 'https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/js/materialize.min.js')


class PythonApp(flx.PyWidget):
    def init(self):
        super().init()
        self.vbox = flx.VBox()
        self.rust_version = flx.Label(text=f'Application Version: {str(rust.get_pkg_ver())}')
        self.example_button = flx.Button(text='Click Me!')
    
    @flx.reaction('example_button.pointer_click')
    def when_clicked(self, *events):
        with self.vbox:
            self.new_text = flx.Label(text=rust.get_test_text())



def main():
    app = flx.App(PythonApp, title='My Awesome App', icon='https://rust-lang.org/logos/rust-logo-512x512.png')
    app.launch('app')
    
    # Before starting the gui, hide the console window
    rust.hide_console_window()

    flx.run()
